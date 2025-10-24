//! Integration tests for GraphQL over gRPC functionality
//!
//! This test suite verifies the integration between reinhardt-grpc and reinhardt-graphql,
//! testing the derive macros and adapters that enable GraphQL facades over gRPC services.
//!
//! Key features tested:
//! - GrpcGraphQLConvert: Automatic type conversion between proto and GraphQL types
//! - GrpcServiceAdapter: Query/Mutation integration with gRPC services
//! - GrpcSubscription: Subscription support via gRPC Server Streaming (Rust 2024 compatible)

use async_graphql::{Context, EmptyMutation, Object, Schema, Subscription, ID};
use reinhardt_graphql_macros::{GrpcGraphQLConvert, GrpcSubscription};
use reinhardt_grpc::{GrpcServiceAdapter, GrpcSubscriptionAdapter};
use reinhardt_integration_tests::{
    grpc_services::{UserEventsBroadcaster, UserEventsServiceImpl, UserServiceImpl, UserStorage},
    proto::{
        common::Empty,
        user::{
            user_service_server::UserService as UserServiceTrait, CreateUserRequest,
            GetUserRequest, ListUsersRequest,
        },
        user_events::{
            user_events_service_server::UserEventsService as UserEventsServiceTrait,
            SubscribeUserEventsRequest, UserEvent as ProtoUserEvent,
        },
    },
};
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status};

// ==================== GraphQL Types with Proto Conversion ====================

/// GraphQL User type with manual proto conversion
#[derive(Debug, Clone, async_graphql::SimpleObject)]
struct User {
    id: ID,
    name: String,
    email: String,
    active: bool,
}

// Manual conversion because proto::User has additional fields (created_at, updated_at)
impl From<reinhardt_integration_tests::proto::user::User> for User {
    fn from(proto: reinhardt_integration_tests::proto::user::User) -> Self {
        Self {
            id: ID::from(proto.id),
            name: proto.name,
            email: proto.email,
            active: proto.active,
        }
    }
}

impl From<User> for reinhardt_integration_tests::proto::user::User {
    fn from(graphql: User) -> Self {
        Self {
            id: graphql.id.to_string(),
            name: graphql.name,
            email: graphql.email,
            active: graphql.active,
            created_at: None,
            updated_at: None,
        }
    }
}

/// GraphQL UserEvent type for Subscription
#[derive(Debug, Clone, async_graphql::SimpleObject)]
struct UserEvent {
    event_type: String,
    user: Option<User>,
    user_id: String,
}

// ==================== gRPC Service Adapters ====================

/// Adapter for UserService gRPC methods
struct UserServiceAdapter {
    service: UserServiceImpl,
}

impl UserServiceAdapter {
    fn new(storage: UserStorage) -> Self {
        Self {
            service: UserServiceImpl::new(storage),
        }
    }
}

#[async_trait::async_trait]
impl GrpcServiceAdapter for UserServiceAdapter {
    type Input = CreateUserRequest;
    type Output = reinhardt_integration_tests::proto::user::User;
    type Error = Status;

    async fn call(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let request = Request::new(input);
        let response = self.service.create_user(request).await?;
        Ok(response.into_inner())
    }
}

/// Adapter for listing users
struct ListUsersAdapter {
    service: UserServiceImpl,
}

impl ListUsersAdapter {
    fn new(storage: UserStorage) -> Self {
        Self {
            service: UserServiceImpl::new(storage),
        }
    }
}

#[async_trait::async_trait]
impl GrpcServiceAdapter for ListUsersAdapter {
    type Input = ListUsersRequest;
    type Output = Vec<reinhardt_integration_tests::proto::user::User>;
    type Error = Status;

    async fn call(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let request = Request::new(input);
        let response = self.service.list_users(request).await?;
        Ok(response.into_inner().users)
    }
}

/// Adapter for getting a single user
struct GetUserAdapter {
    service: UserServiceImpl,
}

impl GetUserAdapter {
    fn new(storage: UserStorage) -> Self {
        Self {
            service: UserServiceImpl::new(storage),
        }
    }
}

#[async_trait::async_trait]
impl GrpcServiceAdapter for GetUserAdapter {
    type Input = GetUserRequest;
    type Output = reinhardt_integration_tests::proto::user::User;
    type Error = Status;

    async fn call(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let request = Request::new(input);
        let response = self.service.get_user(request).await?;
        Ok(response.into_inner())
    }
}

// ==================== GraphQL Schema ====================

struct Query;

#[Object]
impl Query {
    /// Get a user by ID via gRPC
    async fn user(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<User> {
        let adapter = ctx.data::<GetUserAdapter>()?;
        let proto_user = adapter
            .call(GetUserRequest { id: id.to_string() })
            .await
            .map_err(|e| async_graphql::Error::new(e.message()))?;

        Ok(User::from(proto_user))
    }

    /// List all users via gRPC
    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<User>> {
        let adapter = ctx.data::<ListUsersAdapter>()?;
        let proto_users = adapter
            .call(ListUsersRequest {
                page: 0,
                page_size: 100,
            })
            .await
            .map_err(|e| async_graphql::Error::new(e.message()))?;

        Ok(proto_users.into_iter().map(User::from).collect())
    }
}

struct Mutation;

#[Object]
impl Mutation {
    /// Create a new user via gRPC
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        name: String,
        email: String,
    ) -> async_graphql::Result<User> {
        let adapter = ctx.data::<UserServiceAdapter>()?;
        let proto_user = adapter
            .call(CreateUserRequest { name, email })
            .await
            .map_err(|e| async_graphql::Error::new(e.message()))?;

        // Broadcast event
        if let Ok(broadcaster) = ctx.data::<UserEventsBroadcaster>() {
            broadcaster.broadcast(ProtoUserEvent {
                r#type: 0, // CREATED
                user: Some(proto_user.clone()),
                user_id: proto_user.id.clone(),
                timestamp: None,
            });
        }

        Ok(User::from(proto_user))
    }
}

/// Subscription root with gRPC streaming support
struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    /// Subscribe to user events via gRPC Server Streaming
    ///
    /// This demonstrates Rust 2024 compatible Subscription implementation
    /// using gRPC Server Streaming instead of async-graphql's native Subscription.
    async fn user_events<'ctx>(&self, ctx: &Context<'ctx>) -> impl Stream<Item = UserEvent> + 'ctx {
        let service = ctx
            .data::<UserEventsServiceImpl>()
            .expect("UserEventsServiceImpl not found in context");

        let request = Request::new(SubscribeUserEventsRequest {
            event_types: vec![],
            user_id: None,
        });

        // Call gRPC streaming method
        let response = service
            .subscribe_user_events(request)
            .await
            .expect("Failed to subscribe to user events");

        let mut stream = response.into_inner();

        // Convert proto events to GraphQL events
        async_stream::stream! {
            while let Some(result) = stream.next().await {
                if let Ok(proto_event) = result {
                    let event_type = match proto_event.r#type {
                        0 => "CREATED".to_string(),
                        1 => "UPDATED".to_string(),
                        2 => "DELETED".to_string(),
                        _ => "UNKNOWN".to_string(),
                    };

                    yield UserEvent {
                        event_type,
                        user: proto_event.user.map(User::from),
                        user_id: proto_event.user_id,
                    };
                }
            }
        }
    }
}

// ==================== Tests ====================

/// Helper to create schema for testing
fn create_test_schema(
    storage: UserStorage,
    broadcaster: UserEventsBroadcaster,
) -> Schema<Query, Mutation, SubscriptionRoot> {
    let create_adapter = UserServiceAdapter::new(storage.clone());
    let list_adapter = ListUsersAdapter::new(storage.clone());
    let get_adapter = GetUserAdapter::new(storage.clone());
    let events_service = UserEventsServiceImpl::new(broadcaster.clone());

    Schema::build(Query, Mutation, SubscriptionRoot)
        .data(create_adapter)
        .data(list_adapter)
        .data(get_adapter)
        .data(broadcaster)
        .data(events_service)
        .finish()
}

#[tokio::test]
async fn test_grpc_graphql_type_conversion() {
    // Create proto User
    let proto_user = reinhardt_integration_tests::proto::user::User {
        id: "test-123".to_string(),
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        active: true,
        created_at: None,
        updated_at: None,
    };

    // Convert proto → GraphQL
    let graphql_user = User::from(proto_user.clone());
    assert_eq!(graphql_user.id.as_str(), "test-123");
    assert_eq!(graphql_user.name, "Alice");
    assert_eq!(graphql_user.email, "alice@example.com");
    assert!(graphql_user.active);

    // Convert GraphQL → proto
    let proto_user_back: reinhardt_integration_tests::proto::user::User = graphql_user.into();
    assert_eq!(proto_user_back.id, proto_user.id);
    assert_eq!(proto_user_back.name, proto_user.name);
    assert_eq!(proto_user_back.email, proto_user.email);
    assert_eq!(proto_user_back.active, proto_user.active);
}

#[tokio::test]
async fn test_graphql_query_via_grpc_adapter() {
    let storage = UserStorage::new();
    let broadcaster = UserEventsBroadcaster::new();

    // Pre-populate with a user
    storage
        .add_user(reinhardt_integration_tests::proto::user::User {
            id: "user-1".to_string(),
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            active: true,
            created_at: None,
            updated_at: None,
        })
        .await;

    let schema = create_test_schema(storage, broadcaster);

    // Query via GraphQL
    let query = r#"
        {
            user(id: "user-1") {
                id
                name
                email
                active
            }
        }
    "#;

    let result = schema.execute(query).await;
    assert!(result.errors.is_empty());

    let data = result.data.into_json().unwrap();
    assert_eq!(data["user"]["name"], "Bob");
    assert_eq!(data["user"]["email"], "bob@example.com");
}

#[tokio::test]
async fn test_graphql_query_list_via_grpc_adapter() {
    let storage = UserStorage::new();
    let broadcaster = UserEventsBroadcaster::new();

    // Pre-populate with multiple users
    for i in 0..3 {
        storage
            .add_user(reinhardt_integration_tests::proto::user::User {
                id: format!("user-{}", i),
                name: format!("User{}", i),
                email: format!("user{}@example.com", i),
                active: true,
                created_at: None,
                updated_at: None,
            })
            .await;
    }

    let schema = create_test_schema(storage, broadcaster);

    // Query all users
    let query = r#"
        {
            users {
                id
                name
                email
            }
        }
    "#;

    let result = schema.execute(query).await;
    assert!(result.errors.is_empty());

    let data = result.data.into_json().unwrap();
    let users = data["users"].as_array().unwrap();
    assert_eq!(users.len(), 3);
}

#[tokio::test]
async fn test_graphql_mutation_via_grpc_adapter() {
    let storage = UserStorage::new();
    let broadcaster = UserEventsBroadcaster::new();
    let schema = create_test_schema(storage.clone(), broadcaster);

    // Create user via GraphQL mutation
    let mutation = r#"
        mutation {
            createUser(name: "Charlie", email: "charlie@example.com") {
                id
                name
                email
                active
            }
        }
    "#;

    let result = schema.execute(mutation).await;
    assert!(result.errors.is_empty());

    let data = result.data.into_json().unwrap();
    assert_eq!(data["createUser"]["name"], "Charlie");
    assert_eq!(data["createUser"]["email"], "charlie@example.com");
    assert_eq!(data["createUser"]["active"], true);

    // Verify user was actually created in storage
    let users = storage.list_users().await;
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "Charlie");
}

#[tokio::test]
async fn test_graphql_subscription_via_grpc_streaming() {
    // This test verifies that Subscriptions work with gRPC Server Streaming
    // and are compatible with Rust 2024 lifetime rules.

    let storage = UserStorage::new();
    let broadcaster = UserEventsBroadcaster::new();
    let schema = create_test_schema(storage.clone(), broadcaster.clone());

    let subscription_query = r#"
        subscription {
            userEvents {
                eventType
                userId
                user {
                    name
                    email
                }
            }
        }
    "#;

    // Execute subscription
    let mut stream = schema.execute_stream(subscription_query);

    // Broadcast an event
    tokio::spawn({
        let broadcaster = broadcaster.clone();
        async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            broadcaster.broadcast(ProtoUserEvent {
                r#type: 0, // CREATED
                user: Some(reinhardt_integration_tests::proto::user::User {
                    id: "sub-test-1".to_string(),
                    name: "SubUser".to_string(),
                    email: "subuser@example.com".to_string(),
                    active: true,
                    created_at: None,
                    updated_at: None,
                }),
                user_id: "sub-test-1".to_string(),
                timestamp: None,
            });
        }
    });

    // Receive event
    let timeout_duration = tokio::time::Duration::from_secs(2);
    let event = tokio::time::timeout(timeout_duration, stream.next())
        .await
        .expect("Timeout waiting for event")
        .expect("Stream ended unexpectedly");

    assert!(event.errors.is_empty());
    let data = event.data.into_json().unwrap();
    assert_eq!(data["userEvents"]["eventType"], "CREATED");
    assert_eq!(data["userEvents"]["userId"], "sub-test-1");
    assert_eq!(data["userEvents"]["user"]["name"], "SubUser");
}

#[tokio::test]
async fn test_concurrent_graphql_mutations() {
    let storage = UserStorage::new();
    let broadcaster = UserEventsBroadcaster::new();
    let schema = create_test_schema(storage.clone(), broadcaster);

    // Create multiple users concurrently
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let schema = schema.clone();
            tokio::spawn(async move {
                let mutation = format!(
                    r#"
                    mutation {{
                        createUser(name: "User{}", email: "user{}@example.com") {{
                            id
                            name
                        }}
                    }}
                    "#,
                    i, i
                );
                schema.execute(&mutation).await
            })
        })
        .collect();

    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.errors.is_empty());
    }

    // Verify all users were created
    let users = storage.list_users().await;
    assert_eq!(users.len(), 5);
}

#[tokio::test]
async fn test_error_handling_grpc_to_graphql() {
    let storage = UserStorage::new();
    let broadcaster = UserEventsBroadcaster::new();
    let schema = create_test_schema(storage, broadcaster);

    // Query non-existent user
    let query = r#"
        {
            user(id: "nonexistent") {
                id
                name
            }
        }
    "#;

    let result = schema.execute(query).await;
    assert!(!result.errors.is_empty());
    assert!(result.errors[0].message.contains("not found"));
}
