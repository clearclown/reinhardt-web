//! WebSocket Redis channel integration tests
//!
//! Tests Redis-backed channel layer for distributed WebSocket systems:
//! - Room-based message broadcasting
//! - Channel subscription/publishing
//! - Group add/discard operations
//! - Multi-node message routing

use reinhardt_test::fixtures::redis_container;
use reinhardt_websockets::channels::ChannelMessage;
use reinhardt_websockets::redis_channel::{RedisChannelLayer, RedisConfig};
use reinhardt_websockets::{ChannelLayer, Message};
use rstest::*;
use std::sync::Arc;
use testcontainers::ContainerAsync;
use testcontainers_modules::testcontainers::GenericImage;

/// Test: Redis connection establishment
#[rstest]
#[tokio::test]
async fn test_redis_connection_establishment(
	#[future] redis_container: (ContainerAsync<GenericImage>, u16, String),
) {
	let (_container, _port, url) = redis_container.await;

	let config = RedisConfig::new(url);
	let layer = RedisChannelLayer::new(config).await;

	assert!(layer.is_ok(), "Should connect to Redis successfully");
}

/// Test: Send and receive message through channel
#[rstest]
#[tokio::test]
async fn test_send_receive_message(
	#[future] redis_container: (ContainerAsync<GenericImage>, u16, String),
) {
	let (_container, _port, url) = redis_container.await;

	let config = RedisConfig::new(url);
	let layer = RedisChannelLayer::new(config).await.unwrap();

	let channel = "test_channel";
	let channel_msg = ChannelMessage::new(
		"test_sender".to_string(),
		Message::text("Hello from Redis channel".to_string()),
	);

	// Send message
	layer.send(channel, channel_msg).await.unwrap();

	// Receive message
	let received = layer.receive(channel).await.unwrap();

	if let Some(msg) = received {
		assert_eq!(msg.sender(), "test_sender");
		match msg.payload() {
			Message::Text { data } => assert_eq!(data, "Hello from Redis channel"),
			_ => panic!("Expected text message"),
		}
	} else {
		panic!("Expected to receive message");
	}
}

/// Test: Group add and send
#[rstest]
#[tokio::test]
async fn test_group_add_and_send(
	#[future] redis_container: (ContainerAsync<GenericImage>, u16, String),
) {
	let (_container, _port, url) = redis_container.await;

	let config = RedisConfig::new(url);
	let layer = Arc::new(RedisChannelLayer::new(config).await.unwrap());

	let group = "chat_room_1";
	let channel1 = "user_1_channel";
	let channel2 = "user_2_channel";

	// Add channels to group
	layer.group_add(group, channel1).await.unwrap();
	layer.group_add(group, channel2).await.unwrap();

	// Send message to group
	let channel_msg = ChannelMessage::new(
		"broadcaster".to_string(),
		Message::text("Broadcast to room".to_string()),
	);
	layer.group_send(group, channel_msg).await.unwrap();

	// Both channels should receive the message
	let msg1 = layer.receive(channel1).await.unwrap();
	let msg2 = layer.receive(channel2).await.unwrap();

	for msg in [msg1, msg2] {
		if let Some(received_msg) = msg {
			assert_eq!(received_msg.sender(), "broadcaster");
			match received_msg.payload() {
				Message::Text { data } => assert_eq!(data, "Broadcast to room"),
				_ => panic!("Expected text message"),
			}
		} else {
			panic!("Expected to receive message");
		}
	}
}

/// Test: Group discard
#[rstest]
#[tokio::test]
async fn test_group_discard(
	#[future] redis_container: (ContainerAsync<GenericImage>, u16, String),
) {
	let (_container, _port, url) = redis_container.await;

	let config = RedisConfig::new(url);
	let layer = Arc::new(RedisChannelLayer::new(config).await.unwrap());

	let group = "chat_room_2";
	let channel1 = "user_3_channel";
	let channel2 = "user_4_channel";

	// Add both channels to group
	layer.group_add(group, channel1).await.unwrap();
	layer.group_add(group, channel2).await.unwrap();

	// Remove channel1 from group
	layer.group_discard(group, channel1).await.unwrap();

	// Send message to group
	let channel_msg = ChannelMessage::new(
		"sender".to_string(),
		Message::text("Only user_4 should receive".to_string()),
	);
	layer.group_send(group, channel_msg).await.unwrap();

	// Only channel2 should receive the message
	let msg2 = layer.receive(channel2).await.unwrap();
	if let Some(received_msg) = msg2 {
		match received_msg.payload() {
			Message::Text { data } => assert_eq!(data, "Only user_4 should receive"),
			_ => panic!("Expected text message"),
		}
	} else {
		panic!("Expected to receive message");
	}

	// channel1 should not receive any message (receive should timeout or return None)
	let result1 = tokio::time::timeout(
		std::time::Duration::from_millis(100),
		layer.receive(channel1),
	)
	.await;

	if let Ok(Ok(Some(_))) = result1 {
		panic!("channel1 should not receive message")
	}
}

/// Test: Binary message transmission
#[rstest]
#[tokio::test]
async fn test_binary_message_transmission(
	#[future] redis_container: (ContainerAsync<GenericImage>, u16, String),
) {
	let (_container, _port, url) = redis_container.await;

	let config = RedisConfig::new(url);
	let layer = RedisChannelLayer::new(config).await.unwrap();

	let channel = "binary_channel";
	let binary_data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
	let channel_msg = ChannelMessage::new(
		"binary_sender".to_string(),
		Message::binary(binary_data.clone()),
	);

	layer.send(channel, channel_msg).await.unwrap();

	let received = layer.receive(channel).await.unwrap();
	if let Some(msg) = received {
		match msg.payload() {
			Message::Binary { data } => assert_eq!(data, &binary_data),
			_ => panic!("Expected binary message"),
		}
	} else {
		panic!("Expected to receive message");
	}
}

/// Test: Multiple messages in sequence
#[rstest]
#[tokio::test]
async fn test_multiple_messages_sequence(
	#[future] redis_container: (ContainerAsync<GenericImage>, u16, String),
) {
	let (_container, _port, url) = redis_container.await;

	let config = RedisConfig::new(url);
	let layer = RedisChannelLayer::new(config).await.unwrap();

	let channel = "multi_channel";

	// Send multiple messages
	for i in 0..5 {
		let channel_msg = ChannelMessage::new(
			format!("sender_{}", i),
			Message::text(format!("Message {}", i)),
		);
		layer.send(channel, channel_msg).await.unwrap();
	}

	// Receive all messages
	for i in 0..5 {
		let received = layer.receive(channel).await.unwrap();
		if let Some(msg) = received {
			assert_eq!(msg.sender(), format!("sender_{}", i));
			match msg.payload() {
				Message::Text { data } => assert_eq!(data, &format!("Message {}", i)),
				_ => panic!("Expected text message"),
			}
		} else {
			panic!("Expected to receive message {}", i);
		}
	}
}

/// Test: Multiple groups with same channel
#[rstest]
#[tokio::test]
async fn test_channel_in_multiple_groups(
	#[future] redis_container: (ContainerAsync<GenericImage>, u16, String),
) {
	let (_container, _port, url) = redis_container.await;

	let config = RedisConfig::new(url);
	let layer = Arc::new(RedisChannelLayer::new(config).await.unwrap());

	let channel = "multi_group_user";
	let group1 = "room_1";
	let group2 = "room_2";

	// Add same channel to multiple groups
	layer.group_add(group1, channel).await.unwrap();
	layer.group_add(group2, channel).await.unwrap();

	// Send to group1
	let msg1 = ChannelMessage::new(
		"room1_sender".to_string(),
		Message::text("From room 1".to_string()),
	);
	layer.group_send(group1, msg1).await.unwrap();

	// Send to group2
	let msg2 = ChannelMessage::new(
		"room2_sender".to_string(),
		Message::text("From room 2".to_string()),
	);
	layer.group_send(group2, msg2).await.unwrap();

	// Channel should receive both messages
	let received1 = layer.receive(channel).await.unwrap();
	let received2 = layer.receive(channel).await.unwrap();

	// Collect messages for verification
	let mut messages = vec![];
	if let Some(msg) = received1 {
		messages.push((msg.sender().to_string(), msg.payload().clone()));
	}
	if let Some(msg) = received2 {
		messages.push((msg.sender().to_string(), msg.payload().clone()));
	}

	assert_eq!(messages.len(), 2);

	// Verify both messages were received (order might vary)
	let has_room1_msg = messages.iter().any(|(sender, payload)| {
		sender == "room1_sender"
			&& matches!(payload, Message::Text { data } if data == "From room 1")
	});
	let has_room2_msg = messages.iter().any(|(sender, payload)| {
		sender == "room2_sender"
			&& matches!(payload, Message::Text { data } if data == "From room 2")
	});

	assert!(has_room1_msg, "Should receive message from room 1");
	assert!(has_room2_msg, "Should receive message from room 2");
}
