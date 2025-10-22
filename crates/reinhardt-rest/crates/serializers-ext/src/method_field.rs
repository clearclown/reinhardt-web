//! Extended method field implementations
//!
//! This module provides advanced method field types for serializers,
//! including async methods, caching, and conditional fields.

use reinhardt_serializers::MethodFieldError;
use serde_json::Value;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

/// Type alias for boxed futures returning serializer values
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// An async method field that supports asynchronous value computation
///
/// This field allows you to compute values asynchronously, useful for
/// operations that require I/O or other async operations.
///
/// # Examples
///
/// ```
/// use serializers_ext::method_field::AsyncMethodField;
/// use serde_json::{json, Value};
/// use reinhardt_serializers::MethodFieldError;
///
/// #[tokio::main]
/// async fn main() {
///     let field = AsyncMethodField::new(|obj: &Value| {
///         Box::pin(async move {
///             // Simulate async operation
///             tokio::time::sleep(std::time::Duration::from_millis(10)).await;
///             Ok(json!("computed_value"))
///         })
///     });
///
///     let obj = json!({"id": 1});
///     let value = field.get_value(&obj).await.unwrap();
///     assert_eq!(value, json!("computed_value"));
/// }
/// ```
pub struct AsyncMethodField<T, F>
where
    F: Fn(&T) -> BoxFuture<'static, Result<Value, MethodFieldError>>,
{
    method: F,
    _phantom: PhantomData<T>,
}

impl<T, F> AsyncMethodField<T, F>
where
    F: Fn(&T) -> BoxFuture<'static, Result<Value, MethodFieldError>>,
{
    /// Create a new async method field
    ///
    /// # Arguments
    ///
    /// * `method` - An async function that computes the field value
    ///
    /// # Examples
    ///
    /// ```
    /// use serializers_ext::method_field::AsyncMethodField;
    /// use serde_json::{json, Value};
    /// use reinhardt_serializers::MethodFieldError;
    ///
    /// let field = AsyncMethodField::new(|obj: &Value| {
    ///     Box::pin(async move {
    ///         Ok(json!("async_result"))
    ///     })
    /// });
    /// ```
    pub fn new(method: F) -> Self {
        Self {
            method,
            _phantom: PhantomData,
        }
    }

    /// Get the computed value asynchronously
    ///
    /// # Arguments
    ///
    /// * `obj` - The object to compute the value for
    ///
    /// # Returns
    ///
    /// A future that resolves to the computed value
    pub async fn get_value(&self, obj: &T) -> Result<Value, MethodFieldError> {
        (self.method)(obj).await
    }
}

/// A method field that caches computed results
///
/// This field computes values once and caches the result for subsequent calls,
/// improving performance for expensive computations.
///
/// # Examples
///
/// ```
/// use serializers_ext::method_field::CachedMethodField;
/// use serde_json::{json, Value};
/// use reinhardt_serializers::MethodFieldError;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use std::sync::Arc;
///
/// let counter = Arc::new(AtomicUsize::new(0));
/// let counter_clone = counter.clone();
///
/// let field = CachedMethodField::new(move |obj: &Value| {
///     counter_clone.fetch_add(1, Ordering::SeqCst);
///     Ok(json!("cached_value"))
/// });
///
/// let obj = json!({"id": 1});
///
/// // First call computes the value
/// let value1 = field.get_value(&obj).unwrap();
/// assert_eq!(counter.load(Ordering::SeqCst), 1);
///
/// // Second call returns cached value
/// let value2 = field.get_value(&obj).unwrap();
/// assert_eq!(counter.load(Ordering::SeqCst), 1); // Still 1, not 2
/// assert_eq!(value1, value2);
/// ```
pub struct CachedMethodField<T, F>
where
    F: Fn(&T) -> Result<Value, MethodFieldError>,
{
    method: F,
    cache: Arc<Mutex<Option<Value>>>,
    _phantom: PhantomData<T>,
}

impl<T, F> CachedMethodField<T, F>
where
    F: Fn(&T) -> Result<Value, MethodFieldError>,
{
    /// Create a new cached method field
    ///
    /// # Arguments
    ///
    /// * `method` - A function that computes the field value
    ///
    /// # Examples
    ///
    /// ```
    /// use serializers_ext::method_field::CachedMethodField;
    /// use serde_json::{json, Value};
    /// use reinhardt_serializers::MethodFieldError;
    ///
    /// let field = CachedMethodField::new(|obj: &Value| {
    ///     Ok(json!("expensive_computation"))
    /// });
    /// ```
    pub fn new(method: F) -> Self {
        Self {
            method,
            cache: Arc::new(Mutex::new(None)),
            _phantom: PhantomData,
        }
    }

    /// Get the computed value, using cache if available
    ///
    /// # Arguments
    ///
    /// * `obj` - The object to compute the value for
    ///
    /// # Returns
    ///
    /// The computed or cached value
    pub fn get_value(&self, obj: &T) -> Result<Value, MethodFieldError> {
        let mut cache = self.cache.lock().unwrap();

        if let Some(cached_value) = cache.as_ref() {
            return Ok(cached_value.clone());
        }

        let value = (self.method)(obj)?;
        *cache = Some(value.clone());
        Ok(value)
    }

    /// Clear the cache
    ///
    /// # Examples
    ///
    /// ```
    /// use serializers_ext::method_field::CachedMethodField;
    /// use serde_json::{json, Value};
    /// use reinhardt_serializers::MethodFieldError;
    ///
    /// let field = CachedMethodField::new(|obj: &Value| {
    ///     Ok(json!("value"))
    /// });
    ///
    /// let obj = json!({"id": 1});
    /// field.get_value(&obj).unwrap();
    /// field.clear_cache();
    /// // Next call will recompute
    /// ```
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        *cache = None;
    }
}

/// A conditional method field that computes values based on a condition
///
/// This field only computes and returns values when a condition is met.
/// When the condition is not met, it returns `null`.
///
/// # Examples
///
/// ```
/// use serializers_ext::method_field::ConditionalMethodField;
/// use serde_json::{json, Value};
/// use reinhardt_serializers::MethodFieldError;
///
/// let field = ConditionalMethodField::new(
///     |obj: &Value| obj.get("active").and_then(|v| v.as_bool()).unwrap_or(false),
///     |obj: &Value| Ok(json!("active_value"))
/// );
///
/// // When condition is true
/// let active_obj = json!({"active": true});
/// let value = field.get_value(&active_obj).unwrap();
/// assert_eq!(value, json!("active_value"));
///
/// // When condition is false
/// let inactive_obj = json!({"active": false});
/// let value = field.get_value(&inactive_obj).unwrap();
/// assert_eq!(value, Value::Null);
/// ```
pub struct ConditionalMethodField<T, C, M>
where
    C: Fn(&T) -> bool,
    M: Fn(&T) -> Result<Value, MethodFieldError>,
{
    condition: C,
    method: M,
    _phantom: PhantomData<T>,
}

impl<T, C, M> ConditionalMethodField<T, C, M>
where
    C: Fn(&T) -> bool,
    M: Fn(&T) -> Result<Value, MethodFieldError>,
{
    /// Create a new conditional method field
    ///
    /// # Arguments
    ///
    /// * `condition` - A function that determines whether to compute the value
    /// * `method` - A function that computes the field value when condition is true
    ///
    /// # Examples
    ///
    /// ```
    /// use serializers_ext::method_field::ConditionalMethodField;
    /// use serde_json::{json, Value};
    /// use reinhardt_serializers::MethodFieldError;
    ///
    /// let field = ConditionalMethodField::new(
    ///     |obj: &Value| obj.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false),
    ///     |obj: &Value| Ok(json!("enabled_value"))
    /// );
    /// ```
    pub fn new(condition: C, method: M) -> Self {
        Self {
            condition,
            method,
            _phantom: PhantomData,
        }
    }

    /// Get the computed value if condition is met, otherwise returns null
    ///
    /// # Arguments
    ///
    /// * `obj` - The object to evaluate and compute the value for
    ///
    /// # Returns
    ///
    /// The computed value if condition is true, otherwise `Value::Null`
    pub fn get_value(&self, obj: &T) -> Result<Value, MethodFieldError> {
        if (self.condition)(obj) {
            (self.method)(obj)
        } else {
            Ok(Value::Null)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_async_method_field_basic() {
        let field = AsyncMethodField::new(|_obj: &Value| {
            Box::pin(async move { Ok(json!("async_result")) })
        });

        let obj = json!({"id": 1});
        let value = field.get_value(&obj).await.unwrap();
        assert_eq!(value, json!("async_result"));
    }

    #[tokio::test]
    async fn test_async_method_field_with_delay() {
        let field = AsyncMethodField::new(|obj: &Value| {
            let id = obj.get("id").cloned();
            Box::pin(async move {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                Ok(json!({ "processed": id }))
            })
        });

        let obj = json!({"id": 42});
        let value = field.get_value(&obj).await.unwrap();
        assert_eq!(value["processed"], 42);
    }

    #[tokio::test]
    async fn test_async_method_field_error() {
        let field = AsyncMethodField::new(|_obj: &Value| {
            Box::pin(async move {
                Err(MethodFieldError::ComputationError(
                    "Async error".to_string(),
                ))
            })
        });

        let obj = json!({"id": 1});
        let result = field.get_value(&obj).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_cached_method_field_caches_result() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let field = CachedMethodField::new(move |_obj: &Value| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(json!("cached_value"))
        });

        let obj = json!({"id": 1});

        // First call computes the value
        let value1 = field.get_value(&obj).unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert_eq!(value1, json!("cached_value"));

        // Second call uses cached value
        let value2 = field.get_value(&obj).unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1); // Still 1
        assert_eq!(value2, json!("cached_value"));
    }

    #[test]
    fn test_cached_method_field_clear_cache() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let field = CachedMethodField::new(move |_obj: &Value| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            Ok(json!("cached_value"))
        });

        let obj = json!({"id": 1});

        // First call
        field.get_value(&obj).unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // Clear cache
        field.clear_cache();

        // Third call recomputes
        field.get_value(&obj).unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_cached_method_field_with_error() {
        let field = CachedMethodField::new(|_obj: &Value| {
            Err(MethodFieldError::ComputationError(
                "Computation failed".to_string(),
            ))
        });

        let obj = json!({"id": 1});
        let result = field.get_value(&obj);
        assert!(result.is_err());
    }

    #[test]
    fn test_conditional_method_field_condition_true() {
        let field = ConditionalMethodField::new(
            |obj: &Value| obj.get("active").and_then(|v| v.as_bool()).unwrap_or(false),
            |_obj: &Value| Ok(json!("active_value")),
        );

        let obj = json!({"active": true});
        let value = field.get_value(&obj).unwrap();
        assert_eq!(value, json!("active_value"));
    }

    #[test]
    fn test_conditional_method_field_condition_false() {
        let field = ConditionalMethodField::new(
            |obj: &Value| obj.get("active").and_then(|v| v.as_bool()).unwrap_or(false),
            |_obj: &Value| Ok(json!("active_value")),
        );

        let obj = json!({"active": false});
        let value = field.get_value(&obj).unwrap();
        assert_eq!(value, Value::Null);
    }

    #[test]
    fn test_conditional_method_field_missing_field() {
        let field = ConditionalMethodField::new(
            |obj: &Value| obj.get("active").and_then(|v| v.as_bool()).unwrap_or(false),
            |_obj: &Value| Ok(json!("active_value")),
        );

        let obj = json!({"id": 1});
        let value = field.get_value(&obj).unwrap();
        assert_eq!(value, Value::Null);
    }

    #[test]
    fn test_conditional_method_field_with_complex_condition() {
        let field = ConditionalMethodField::new(
            |obj: &Value| {
                obj.get("score")
                    .and_then(|v| v.as_i64())
                    .map(|s| s > 50)
                    .unwrap_or(false)
            },
            |obj: &Value| {
                let score = obj.get("score").and_then(|v| v.as_i64()).unwrap_or(0);
                Ok(json!({"grade": "pass", "score": score}))
            },
        );

        let high_score = json!({"score": 75});
        let value = field.get_value(&high_score).unwrap();
        assert_eq!(value["grade"], "pass");
        assert_eq!(value["score"], 75);

        let low_score = json!({"score": 30});
        let value = field.get_value(&low_score).unwrap();
        assert_eq!(value, Value::Null);
    }

    #[test]
    fn test_conditional_method_field_error_when_condition_true() {
        let field = ConditionalMethodField::new(
            |_obj: &Value| true,
            |_obj: &Value| {
                Err(MethodFieldError::ComputationError(
                    "Method error".to_string(),
                ))
            },
        );

        let obj = json!({"id": 1});
        let result = field.get_value(&obj);
        assert!(result.is_err());
    }
}
