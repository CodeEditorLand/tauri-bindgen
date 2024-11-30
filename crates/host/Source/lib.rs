use std::{
	any::Any,
	collections::HashMap,
	sync::{Arc, RwLock},
};

pub use tauri_bindgen_host_macro::*;
#[doc(hidden)]
pub use {anyhow, async_trait::async_trait, bitflags, ipc_router_wip, serde, tauri, tracing};
pub type Result<T> = anyhow::Result<T>;

pub type ResourceId = u32;

#[derive(Default)]
pub struct ResourceTable(RwLock<ResourceTableInner>);

#[derive(Default)]
pub struct ResourceTableInner {
	map:HashMap<ResourceId, Arc<dyn Any + Send + Sync>>,
	next_rid:ResourceId,
}

impl ResourceTable {
	/// Create an empty table.
	#[must_use]
	pub fn new() -> Self {
		Self(RwLock::new(ResourceTableInner { map:HashMap::new(), next_rid:0 }))
	}

	/// Insert a resource at the next available index.
	///
	/// # Errors
	///
	/// Returns an error if the table is full.
	///
	/// # Panics
	///
	/// Panics if the resource is already borrowed.
	pub fn push<T:Any + Send + Sync>(&self, a:Arc<T>) -> Result<ResourceId> {
		let mut inner = self.0.write().unwrap();
		// NOTE: The performance of this new key calculation could be very bad
		// once keys wrap around.
		if inner.map.len() == u32::MAX as usize {
			return Err(anyhow::anyhow!("table has no free keys"));
		}

		loop {
			let key = inner.next_rid;

			inner.next_rid += 1;

			if inner.map.contains_key(&key) {
				continue;
			}

			inner.map.insert(key, a);

			return Ok(key);
		}
	}

	/// Check if the table has a resource at the given index.
	///
	/// # Panics
	///
	/// Panics if the resource is already borrowed.
	pub fn contains_key(&self, key:ResourceId) -> bool {
		self.0.read().unwrap().map.contains_key(&key)
	}

	/// Check if the resource at a given index can be downcast to a given type.
	/// Note: this will always fail if the resource is already borrowed.
	///
	/// # Panics
	///
	/// Panics if the resource is already borrowed.
	pub fn is<T:Any + Sized>(&self, key:ResourceId) -> bool {
		if let Some(r) = self.0.read().unwrap().map.get(&key) {
			r.is::<T>()
		} else {
			false
		}
	}

	/// Get an Arc reference to a resource of a given type at a given index.
	/// Multiple immutable references can be borrowed at any given time.
	///
	/// # Errors
	///
	/// Returns an error if the resource is not of the given type.
	///
	/// # Panics
	///
	/// Panics if the resource is already borrowed.
	pub fn get<T:Any + Send + Sync + Sized>(&self, key:ResourceId) -> Result<Arc<T>> {
		if let Some(r) = self.0.read().unwrap().map.get(&key).cloned() {
			r.downcast::<T>().map_err(|_| anyhow::anyhow!("element is a different type"))
		} else {
			Err(anyhow::anyhow!("key not in table"))
		}
	}

	/// Get a mutable reference to a resource of a given type at a given index.
	///
	/// # Errors
	///
	/// Returns an error if the resource is not of the given type or if the
	/// resource is already borrowed.
	///
	/// # Panics
	///
	/// Panics if the resource is already borrowed.
	pub fn get_mut<T:Any>(&mut self, key:ResourceId) -> Result<&mut T> {
		let entry = self
			.0
			.get_mut()
			.unwrap()
			.map
			.get_mut(&key)
			.ok_or(anyhow::anyhow!("key not in table"))?;

		let entry =
			Arc::get_mut(entry).ok_or(anyhow::anyhow!("cannot mutably borrow shared file"))?;

		entry
			.downcast_mut::<T>()
			.ok_or_else(|| anyhow::anyhow!("element is a different type"))
	}

	/// Remove a resource at a given index from the table and returns it.
	///
	/// # Panics
	///
	/// Panics if the resource is already borrowed.
	pub fn take<T:Any + Send + Sync>(&self, key:ResourceId) -> Option<Arc<T>> {
		self.0.write().unwrap().map.remove(&key).map(|r| r.downcast::<T>().unwrap())
	}
}
