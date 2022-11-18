#![deny(missing_docs)]

use std::sync::{Arc, RwLock, RwLockReadGuard};

/// This is the AppUniverse struct that holds the universe (state) in an Arc<RwLock> and internally, the subscribers.
///
/// Cloning the AppUniverse is really cheap and all clones hold pointers to the same inner state.
pub struct AppUniverse<U: AppUniverseCore> {
    universe: Arc<RwLock<U>>,
    subscribers: Arc<RwLock<Vec<Box<dyn FnMut(AppUniverse<U>)>>>>,
}

/// This is a subscription struct. Typically, you are NOT supposed to use this struct for anything other than passing it into the universe to during unsubscription
pub struct UniverseSubscription {
    /// This holds the address of the subscription function in memory
    pub address: usize,
}

/// Defines how messages that indicate that something has happened get sent to the universe.
pub trait AppUniverseCore: Sized {
    /// Indicates that something has happened.
    type Message;

    /// Send a message to the state object.
    /// This will usually lead to a state update
    fn msg(&mut self, message: Self::Message);
}

fn type_id_of_val<T>(_: &T) -> usize {
    type_id_of_val::<T> as usize
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

    /// This function takes a subscriber function that takes and is run anytime state changes.
    ///
    /// A subscriber function `subscriber_fn` is a function that will be called whenever state changes and it will pass in the updated state
    pub fn subscribe(
        &mut self,
        subscriber_fn: Box<dyn FnMut(AppUniverse<U>)>,
    ) -> UniverseSubscription {
        let address = type_id_of_val(&subscriber_fn);
        self.subscribers.write().unwrap().push(subscriber_fn);
        UniverseSubscription { address }
    }

    /// This function takes a subscription and removed the subscriber function so that it is no longer gets called whenever state changes
    pub fn unsubscribe(&mut self, subscription: UniverseSubscription) {
        let mut subscribers = self.subscribers.write().unwrap();
        let mut index_to_remove: Option<usize> = None;
        for (index, subscriber) in subscribers.iter().enumerate() {
            if type_id_of_val(subscriber) == subscription.address {
                index_to_remove = Some(index);
                break;
            }
        }
        if let Some(index) = index_to_remove {
            subscribers.swap_remove(index);
        }
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
