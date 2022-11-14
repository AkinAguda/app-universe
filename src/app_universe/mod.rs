#![deny(missing_docs)]

use std::sync::{Arc, RwLock, RwLockReadGuard};

/// Cloning an `AppUniverse` is a very cheap operation.
///
/// All clones hold pointers to the same inner state.
pub struct AppUniverse<U: AppUniverseCore> {
    universe: Arc<RwLock<U>>,
    subscribers: Arc<RwLock<Vec<Box<dyn FnMut(AppUniverse<U>)>>>>,
}

/// Defines how messages that indicate that something has happened get sent to the universe.
pub trait AppUniverseCore: Sized {
    /// Indicates that something has happened.
    type Message;

    /// Send a message to the state object.
    /// This will usually lead to a state update
    fn msg(&mut self, message: Self::Message);
}

/// This wrapper defines the type of a universe

impl<U: AppUniverseCore + 'static> AppUniverse<U> {
    /// This creates a new app_universe
    pub fn new(universe_core: U) -> Self {
        let universe = Arc::new(RwLock::new(universe_core));
        Self {
            universe,
            subscribers: Arc::new(RwLock::new(vec![])),
        }
    }

    /// Acquire write access to the AppUniverse then send a message.
    pub fn msg(&self, msg: U::Message) {
        self.universe.write().unwrap().msg(msg);
        for subscriber in self.subscribers.write().unwrap().iter_mut() {
            (subscriber)(self.clone());
        }
    }

    /// Acquire read access to AppUniverse.
    pub fn read(&self) -> RwLockReadGuard<'_, U> {
        self.universe.read().unwrap()
    }

    /// Subscribe to the Universe
    pub fn subscribe(&mut self, subscriber_fn: Box<dyn FnMut(AppUniverse<U>)>) {
        self.subscribers.write().unwrap().push(subscriber_fn);
    }

    /// Acquire write access to AppUniverse.
    ///
    /// Under normal circumstances you should only ever write to the universe through the `.msg()`
    /// method.
    ///
    /// This .write() method is useful when writing tests where you want to quickly set up some
    /// initial state.
    #[cfg(feature = "test-utils")]
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, W> {
        self.universe.write().unwrap()
    }
}

impl<W: AppUniverseCore> Clone for AppUniverse<W> {
    fn clone(&self) -> Self {
        AppUniverse {
            universe: self.universe.clone(),
            subscribers: self.subscribers.clone(),
        }
    }
}
