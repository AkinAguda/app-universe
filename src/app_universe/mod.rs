#![deny(missing_docs)]

use std::sync::{Arc, RwLock, RwLockReadGuard};

struct Subscription<U: AppUniverseCore> {
    subscriber_fn: Box<dyn FnMut(AppUniverse<U>)>,
    id: u16,
}

/// This is the AppUniverse struct that holds the universe (state) in an Arc<RwLock> and internally, the subscribers.
///
/// Cloning the AppUniverse is really cheap and all clones hold pointers to the same inner state.
pub struct AppUniverse<U: AppUniverseCore> {
    universe: Arc<RwLock<U>>,
    subscribers: Arc<RwLock<Vec<Subscription<U>>>>,
    /* The first value in this vector keeps a count of the current maxiumum number used as a subscriber id.
    Whenever an unsubscription occurs, it adds the id to this vector of `available_subscriber_ids`.
    Whenever all the available subscriber ids have been reassigned to a new subscription, it will assign the first
    value in `available_subscriber_ids` as the new subscriber id and increment it, ready to be re_used as a `subscriber_id`.

    The major cost here is in space. If we have 10000 subscriptions and 998 unsubscriptions for instance, we will have 998 `available_subscriber_ids` + the counter at index 0
    */
    available_subscriber_ids: Arc<RwLock<Vec<u16>>>,
}

/// This is a subscription struct. Typically, you are NOT supposed to use this struct for anything other than passing it into the universe to during unsubscription
pub struct UniverseSubscription {
    /// This holds the index of the subscription function in the subscriptions array
    pub subscription_id: u16,
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
            available_subscriber_ids: Arc::new(RwLock::new(vec![0])),
        }
    }

    /// Acquire write access to the AppUniverse then send a message.
    pub fn msg(&self, msg: U::Message) {
        self.universe.write().unwrap().msg(msg);
        for subscriber in self.subscribers.write().unwrap().iter_mut() {
            (subscriber.subscriber_fn)(self.clone());
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
        let mut subscribers = self.subscribers.write().unwrap();
        let mut available_subscription_ids = self.available_subscriber_ids.write().unwrap();

        let mut subscription_id = available_subscription_ids[0];

        if available_subscription_ids.len() > 1 {
            subscription_id = available_subscription_ids.pop().unwrap();
        } else {
            available_subscription_ids[0] += 1;
        }

        subscribers.push(Subscription {
            subscriber_fn,
            id: subscription_id,
        });

        UniverseSubscription { subscription_id }
    }

    /// This function takes a subscription and removed the subscriber function so that it is no longer gets called whenever state changes
    pub fn unsubscribe(&mut self, subscription: UniverseSubscription) {
        let mut subscribers = self.subscribers.write().unwrap();
        let mut available_subscriber_ids = self.available_subscriber_ids.write().unwrap();

        let mut index_to_remove: Option<usize> = None;
        for (index, subscriber) in subscribers.iter().enumerate() {
            if subscriber.id == subscription.subscription_id {
                index_to_remove = Some(index);
                break;
            }
        }
        if let Some(index) = index_to_remove {
            available_subscriber_ids.push(subscribers[index].id);
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
            available_subscriber_ids: self.available_subscriber_ids.clone(),
        }
    }
}
