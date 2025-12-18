//! Signal - Fine-grained Reactive Primitive
//!
//! `Signal<T>` is the core reactive primitive that holds a value and automatically
//! tracks dependencies when accessed.
//!
//! ## Key Features
//!
//! - **Automatic Dependency Tracking**: When `get()` is called inside an Effect or Memo,
//!   the dependency is automatically recorded.
//! - **Change Notification**: When `set()` or `update()` is called, all dependent Effects
//!   are automatically scheduled for re-execution.
//! - **Lightweight**: Signal<T> is just a NodeId wrapper, making it cheap to clone and pass around.
//! - **Type-safe**: The value type is enforced at compile time.
//!
//! ## Example
//!
//! ```ignore
//! use reinhardt_page::reactive::Signal;
//!
//! // Create a signal
//! let count = Signal::new(0);
//!
//! // Read the value
//! assert_eq!(count.get(), 0);
//!
//! // Update the value
//! count.set(42);
//! assert_eq!(count.get(), 42);
//!
//! // Update with a function
//! count.update(|n| *n += 1);
//! assert_eq!(count.get(), 43);
//! ```

use core::cell::RefCell;
use core::fmt;

extern crate alloc;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::rc::Rc;

use super::runtime::{NodeId, try_with_runtime, with_runtime};

/// Internal storage for Signal values
///
/// This stores the actual values of all Signals in the system.
/// We use BTreeMap for deterministic ordering in tests.
type SignalStorage<T> = BTreeMap<NodeId, Rc<RefCell<T>>>;

thread_local! {
	/// Global storage for all Signal values
	///
	/// This is a type-erased map that stores Signal values indexed by NodeId.
	/// Each Signal<T> stores its value in this map using its unique NodeId as the key.
	static SIGNAL_VALUES: RefCell<BTreeMap<usize, Box<dyn core::any::Any>>> = RefCell::new(BTreeMap::new());
}

/// A reactive signal that holds a value and tracks dependencies
///
/// `Signal<T>` is the fundamental building block of the reactive system. It represents
/// a piece of state that can change over time, and automatically notifies dependent
/// computations when it changes.
///
/// ## Type Parameter
///
/// * `T` - The type of value stored in the signal. Must be `'static` to ensure memory safety.
///
/// ## Cloning
///
/// Signal<T> implements `Clone` and is cheap to clone - it only copies the NodeId.
/// All clones of the same Signal share the same underlying value.
#[derive(Clone)]
pub struct Signal<T: 'static> {
	/// Unique identifier for this signal
	id: NodeId,
	/// Phantom data to associate the type T with this signal
	_phantom: core::marker::PhantomData<T>,
}

impl<T: 'static> Signal<T> {
	/// Create a new Signal with the given initial value
	///
	/// # Arguments
	///
	/// * `value` - Initial value for the signal
	///
	/// # Example
	///
	/// ```ignore
	/// let count = Signal::new(0);
	/// assert_eq!(count.get(), 0);
	/// ```
	pub fn new(value: T) -> Self {
		let id = NodeId::new();

		// Store the value in global storage
		SIGNAL_VALUES.with(|storage| {
			let mut storage = storage.borrow_mut();
			let type_id = core::any::TypeId::of::<T>();
			let type_key = type_id_to_usize(type_id);

			// Get or create storage for this type
			let type_storage = storage
				.entry(type_key)
				.or_insert_with(|| Box::new(BTreeMap::<NodeId, Rc<RefCell<T>>>::new()));

			// Downcast to the concrete type
			if let Some(typed_storage) = type_storage.downcast_mut::<SignalStorage<T>>() {
				typed_storage.insert(id, Rc::new(RefCell::new(value)));
			}
		});

		Self {
			id,
			_phantom: core::marker::PhantomData,
		}
	}

	/// Get the current value of the signal
	///
	/// This automatically tracks the dependency if called from within an Effect or Memo.
	///
	/// # Example
	///
	/// ```ignore
	/// let count = Signal::new(42);
	/// assert_eq!(count.get(), 42);
	/// ```
	pub fn get(&self) -> T
	where
		T: Clone,
	{
		// Track dependency with the runtime
		with_runtime(|rt| rt.track_dependency(self.id));

		// Get the value from storage
		self.get_untracked()
	}

	/// Get the current value without tracking dependencies
	///
	/// This is useful when you want to read a signal's value without creating
	/// a dependency relationship.
	///
	/// # Example
	///
	/// ```ignore
	/// let count = Signal::new(42);
	/// // This won't create a dependency
	/// let value = count.get_untracked();
	/// ```
	pub fn get_untracked(&self) -> T
	where
		T: Clone,
	{
		SIGNAL_VALUES.with(|storage| {
			let storage = storage.borrow();
			let type_id = core::any::TypeId::of::<T>();
			let type_key = type_id_to_usize(type_id);

			if let Some(type_storage) = storage.get(&type_key)
				&& let Some(typed_storage) = type_storage.downcast_ref::<SignalStorage<T>>()
				&& let Some(value_cell) = typed_storage.get(&self.id)
			{
				return value_cell.borrow().clone();
			}

			panic!("Signal value not found - this should never happen");
		})
	}

	/// Set the signal to a new value
	///
	/// This notifies all dependent Effects and Memos that the signal has changed.
	///
	/// # Arguments
	///
	/// * `value` - New value for the signal
	///
	/// # Example
	///
	/// ```ignore
	/// let count = Signal::new(0);
	/// count.set(42);
	/// assert_eq!(count.get(), 42);
	/// ```
	pub fn set(&self, value: T) {
		SIGNAL_VALUES.with(|storage| {
			let storage = storage.borrow();
			let type_id = core::any::TypeId::of::<T>();
			let type_key = type_id_to_usize(type_id);

			if let Some(type_storage) = storage.get(&type_key)
				&& let Some(typed_storage) = type_storage.downcast_ref::<SignalStorage<T>>()
				&& let Some(value_cell) = typed_storage.get(&self.id)
			{
				*value_cell.borrow_mut() = value;
			}
		});

		// Notify runtime of the change
		with_runtime(|rt| rt.notify_signal_change(self.id));
	}

	/// Update the signal's value using a function
	///
	/// This is more efficient than `get()` + `set()` because it only notifies
	/// dependents once.
	///
	/// # Arguments
	///
	/// * `f` - Function that takes a mutable reference to the current value
	///
	/// # Example
	///
	/// ```ignore
	/// let count = Signal::new(0);
	/// count.update(|n| *n += 1);
	/// assert_eq!(count.get(), 1);
	/// ```
	pub fn update<F>(&self, f: F)
	where
		F: FnOnce(&mut T),
	{
		SIGNAL_VALUES.with(|storage| {
			let storage = storage.borrow();
			let type_id = core::any::TypeId::of::<T>();
			let type_key = type_id_to_usize(type_id);

			if let Some(type_storage) = storage.get(&type_key)
				&& let Some(typed_storage) = type_storage.downcast_ref::<SignalStorage<T>>()
				&& let Some(value_cell) = typed_storage.get(&self.id)
			{
				f(&mut *value_cell.borrow_mut());
			}
		});

		// Notify runtime of the change
		with_runtime(|rt| rt.notify_signal_change(self.id));
	}

	/// Get the NodeId of this signal
	///
	/// This is mainly for internal use by the runtime and tests.
	pub fn id(&self) -> NodeId {
		self.id
	}
}

impl<T: 'static> Drop for Signal<T> {
	fn drop(&mut self) {
		// Remove from runtime's dependency graph (ignore if TLS is destroyed)
		let _ = try_with_runtime(|rt| rt.remove_node(self.id));

		// Remove from storage (ignore if TLS is destroyed)
		let _ = SIGNAL_VALUES.try_with(|storage| {
			let mut storage = storage.borrow_mut();
			let type_id = core::any::TypeId::of::<T>();
			let type_key = type_id_to_usize(type_id);

			if let Some(type_storage) = storage.get_mut(&type_key)
				&& let Some(typed_storage) = type_storage.downcast_mut::<SignalStorage<T>>()
			{
				typed_storage.remove(&self.id);
			}
		});
	}
}

impl<T: fmt::Debug + Clone + 'static> fmt::Debug for Signal<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Signal")
			.field("id", &self.id)
			.field("value", &self.get_untracked())
			.finish()
	}
}

/// Convert TypeId to usize for use as a map key
///
/// This is a workaround for TypeId not implementing Ord.
/// We use a simple hash function to convert TypeId to usize.
fn type_id_to_usize(type_id: core::any::TypeId) -> usize {
	// Simple hash: XOR the high and low 64 bits of the TypeId
	// TypeId is 128 bits internally, but we can't access it directly
	// So we use format! and hash the string representation
	let type_id_str = alloc::format!("{:?}", type_id);
	let mut hash: usize = 0;
	for byte in type_id_str.bytes() {
		hash = hash.wrapping_mul(31).wrapping_add(byte as usize);
	}
	hash
}

#[cfg(test)]
mod tests {
	use super::super::runtime::NodeType;
	use super::*;
	use serial_test::serial;

	#[test]
	#[serial]
	fn test_signal_creation() {
		let signal = Signal::new(42);
		assert_eq!(signal.get_untracked(), 42);
	}

	#[test]
	#[serial]
	fn test_signal_set() {
		let signal = Signal::new(0);
		assert_eq!(signal.get_untracked(), 0);

		signal.set(100);
		assert_eq!(signal.get_untracked(), 100);
	}

	#[test]
	#[serial]
	fn test_signal_update() {
		let signal = Signal::new(0);

		signal.update(|n| *n += 1);
		assert_eq!(signal.get_untracked(), 1);

		signal.update(|n| *n *= 2);
		assert_eq!(signal.get_untracked(), 2);
	}

	#[test]
	#[serial]
	fn test_signal_clone() {
		let signal1 = Signal::new(42);
		let signal2 = signal1.clone();

		assert_eq!(signal1.get_untracked(), 42);
		assert_eq!(signal2.get_untracked(), 42);

		signal1.set(100);
		assert_eq!(signal1.get_untracked(), 100);
		assert_eq!(signal2.get_untracked(), 100);
	}

	#[test]
	#[serial]
	fn test_multiple_signals() {
		let signal1 = Signal::new(10);
		let signal2 = Signal::new(20);
		let signal3 = Signal::new("hello");

		assert_eq!(signal1.get_untracked(), 10);
		assert_eq!(signal2.get_untracked(), 20);
		assert_eq!(signal3.get_untracked(), "hello");

		signal1.set(30);
		signal2.set(40);
		signal3.set("world");

		assert_eq!(signal1.get_untracked(), 30);
		assert_eq!(signal2.get_untracked(), 40);
		assert_eq!(signal3.get_untracked(), "world");
	}

	#[test]
	#[serial]
	fn test_signal_dependency_tracking() {
		let signal = Signal::new(42);

		// Without observer, get() should still work
		assert_eq!(signal.get(), 42);

		// With observer, dependency should be tracked
		with_runtime(|rt| {
			let observer_id = NodeId::new();
			rt.push_observer(super::super::runtime::Observer {
				id: observer_id,
				node_type: NodeType::Effect,
				cleanup: None,
			});

			// This should track the dependency
			let _ = signal.get();

			rt.pop_observer();

			// Verify dependency was tracked
			let graph = rt.dependency_graph.borrow();
			let signal_node = graph.get(&signal.id()).unwrap();
			assert!(signal_node.subscribers.contains(&observer_id));
		});
	}

	#[test]
	#[serial]
	fn test_signal_change_notification() {
		let signal = Signal::new(0);

		with_runtime(|rt| {
			let effect_id = NodeId::new();

			// Manually set up dependency
			{
				let mut graph = rt.dependency_graph.borrow_mut();
				graph
					.entry(signal.id())
					.or_default()
					.subscribers
					.push(effect_id);
			}

			// Change the signal
			signal.set(42);

			// Verify the effect was scheduled for update
			let pending = rt.pending_updates.borrow();
			assert!(pending.contains(&effect_id));
		});
	}
}
