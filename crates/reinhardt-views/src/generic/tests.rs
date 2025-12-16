//! Unit tests for Generic API Views

#[cfg(test)]
mod list_api_tests {
	use crate::View;
	use crate::generic::ListAPIView;
	use reinhardt_db::orm::Model;
	use reinhardt_serializers::JsonSerializer;
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Clone, Serialize, Deserialize)]
	struct TestArticle {
		id: Option<i64>,
		title: String,
	}

	impl Model for TestArticle {
		type PrimaryKey = i64;
		fn table_name() -> &'static str {
			"articles"
		}
		fn primary_key(&self) -> Option<&Self::PrimaryKey> {
			self.id.as_ref()
		}
		fn set_primary_key(&mut self, value: Self::PrimaryKey) {
			self.id = Some(value);
		}
	}

	#[test]
	fn test_list_api_view_new() {
		let view = ListAPIView::<TestArticle, JsonSerializer<TestArticle>>::new();
		let methods = view.allowed_methods();
		assert_eq!(methods, vec!["GET", "HEAD", "OPTIONS"]);
	}

	#[test]
	fn test_list_api_view_with_ordering() {
		let view = ListAPIView::<TestArticle, JsonSerializer<TestArticle>>::new()
			.with_ordering(vec!["-created_at".to_string(), "title".to_string()]);
		let methods = view.allowed_methods();
		assert_eq!(methods, vec!["GET", "HEAD", "OPTIONS"]);
	}

	#[test]
	fn test_list_api_view_with_pagination() {
		let view =
			ListAPIView::<TestArticle, JsonSerializer<TestArticle>>::new().with_paginate_by(20);
		let methods = view.allowed_methods();
		assert_eq!(methods, vec!["GET", "HEAD", "OPTIONS"]);
	}
}

#[cfg(test)]
mod create_api_tests {
	use crate::View;
	use crate::generic::CreateAPIView;
	use reinhardt_db::orm::Model;
	use reinhardt_serializers::JsonSerializer;
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Clone, Serialize, Deserialize)]
	struct TestArticle {
		id: Option<i64>,
		title: String,
	}

	impl Model for TestArticle {
		type PrimaryKey = i64;
		fn table_name() -> &'static str {
			"articles"
		}
		fn primary_key(&self) -> Option<&Self::PrimaryKey> {
			self.id.as_ref()
		}
		fn set_primary_key(&mut self, value: Self::PrimaryKey) {
			self.id = Some(value);
		}
	}

	#[test]
	fn test_create_api_view_new() {
		let view = CreateAPIView::<TestArticle, JsonSerializer<TestArticle>>::new();
		let methods = view.allowed_methods();
		assert_eq!(methods, vec!["POST", "OPTIONS"]);
	}

	#[test]
	fn test_create_api_view_default() {
		let view = CreateAPIView::<TestArticle, JsonSerializer<TestArticle>>::default();
		let methods = view.allowed_methods();
		assert_eq!(methods, vec!["POST", "OPTIONS"]);
	}
}

#[cfg(test)]
mod update_api_tests {
	use crate::View;
	use crate::generic::UpdateAPIView;
	use reinhardt_db::orm::Model;
	use reinhardt_serializers::JsonSerializer;
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Clone, Serialize, Deserialize)]
	struct TestArticle {
		id: Option<i64>,
		title: String,
	}

	impl Model for TestArticle {
		type PrimaryKey = i64;
		fn table_name() -> &'static str {
			"articles"
		}
		fn primary_key(&self) -> Option<&Self::PrimaryKey> {
			self.id.as_ref()
		}
		fn set_primary_key(&mut self, value: Self::PrimaryKey) {
			self.id = Some(value);
		}
	}

	#[test]
	fn test_update_api_view_new() {
		let view = UpdateAPIView::<TestArticle, JsonSerializer<TestArticle>>::new();
		let methods = view.allowed_methods();
		assert_eq!(methods, vec!["PUT", "PATCH", "OPTIONS"]);
	}

	#[test]
	fn test_update_api_view_with_lookup_field() {
		let view = UpdateAPIView::<TestArticle, JsonSerializer<TestArticle>>::new()
			.with_lookup_field("slug".to_string());
		let methods = view.allowed_methods();
		assert_eq!(methods, vec!["PUT", "PATCH", "OPTIONS"]);
	}

	#[test]
	fn test_update_api_view_with_partial() {
		let view =
			UpdateAPIView::<TestArticle, JsonSerializer<TestArticle>>::new().with_partial(true);
		let methods = view.allowed_methods();
		assert_eq!(methods, vec!["PUT", "PATCH", "OPTIONS"]);
	}
}

#[cfg(test)]
mod destroy_api_tests {
	use crate::View;
	use crate::generic::DestroyAPIView;
	use reinhardt_db::orm::Model;
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Clone, Serialize, Deserialize)]
	struct TestArticle {
		id: Option<i64>,
		title: String,
	}

	impl Model for TestArticle {
		type PrimaryKey = i64;
		fn table_name() -> &'static str {
			"articles"
		}
		fn primary_key(&self) -> Option<&Self::PrimaryKey> {
			self.id.as_ref()
		}
		fn set_primary_key(&mut self, value: Self::PrimaryKey) {
			self.id = Some(value);
		}
	}

	#[test]
	fn test_destroy_api_view_new() {
		let view = DestroyAPIView::<TestArticle>::new();
		let methods = view.allowed_methods();
		assert_eq!(methods, vec!["DELETE", "OPTIONS"]);
	}

	#[test]
	fn test_destroy_api_view_with_lookup_field() {
		let view = DestroyAPIView::<TestArticle>::new().with_lookup_field("slug".to_string());
		let methods = view.allowed_methods();
		assert_eq!(methods, vec!["DELETE", "OPTIONS"]);
	}
}

#[cfg(test)]
mod composite_api_tests {
	use crate::View;
	use crate::generic::{
		ListCreateAPIView, RetrieveDestroyAPIView, RetrieveUpdateAPIView,
		RetrieveUpdateDestroyAPIView,
	};
	use reinhardt_db::orm::Model;
	use reinhardt_serializers::JsonSerializer;
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Clone, Serialize, Deserialize)]
	struct TestArticle {
		id: Option<i64>,
		title: String,
	}

	impl Model for TestArticle {
		type PrimaryKey = i64;
		fn table_name() -> &'static str {
			"articles"
		}
		fn primary_key(&self) -> Option<&Self::PrimaryKey> {
			self.id.as_ref()
		}
		fn set_primary_key(&mut self, value: Self::PrimaryKey) {
			self.id = Some(value);
		}
	}

	#[test]
	fn test_list_create_api_view_new() {
		let view = ListCreateAPIView::<TestArticle, JsonSerializer<TestArticle>>::new();
		let methods = view.allowed_methods();
		assert_eq!(methods, vec!["GET", "HEAD", "POST", "OPTIONS"]);
	}

	#[test]
	fn test_retrieve_update_api_view_new() {
		let view = RetrieveUpdateAPIView::<TestArticle, JsonSerializer<TestArticle>>::new();
		let methods = view.allowed_methods();
		assert_eq!(methods, vec!["GET", "HEAD", "PUT", "PATCH", "OPTIONS"]);
	}

	#[test]
	fn test_retrieve_destroy_api_view_new() {
		let view = RetrieveDestroyAPIView::<TestArticle, JsonSerializer<TestArticle>>::new();
		let methods = view.allowed_methods();
		assert_eq!(methods, vec!["GET", "HEAD", "DELETE", "OPTIONS"]);
	}

	#[test]
	fn test_retrieve_update_destroy_api_view_new() {
		let view = RetrieveUpdateDestroyAPIView::<TestArticle, JsonSerializer<TestArticle>>::new();
		let methods = view.allowed_methods();
		assert_eq!(
			methods,
			vec!["GET", "HEAD", "PUT", "PATCH", "DELETE", "OPTIONS"]
		);
	}
}
