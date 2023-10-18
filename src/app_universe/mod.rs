#![deny(missing_docs)]

use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

/// This is the internal subscription used to hold the subscriber function.
struct Subscription<U: AppUniverseCore>(Box<dyn FnMut(AppUniverse<U>)>);

type UniverseSubscriptionParameter<U> = Rc<RefCell<Subscription<U>>>;

/// The `UniverseSubscription` is the public subscription that is returned
/// whenever the `subscribe` method on `AppUniverse` is called.
/// Its only purpose is to be passed into the `unsubscribe` method on
/// `AppUniverse` whenever it's called.
pub struct UniverseSubscription<U: AppUniverseCore>(UniverseSubscriptionParameter<U>);

/// This is the holds the application state "universe" and the subscriptions to
/// that state. The only way to read information about the state publicly is by calling
/// the `read` method on `AppUniverse`. There is no way to publicly access the subscriptions
///
/// Cloning the AppUniverse is really cheap and all clones hold pointers to the same inner state.
pub struct AppUniverse<U: AppUniverseCore> {
    universe: Rc<RefCell<U>>,
    subscriptions: Rc<RefCell<Vec<UniverseSubscriptionParameter<U>>>>,
}

/// This trait defines the blueprint for the "core" of a universe.
/// This means that for you to create an instance of `AppUniverse`,
/// the value you pass in as your state "core" has to implement `AppUniverseCore`.
pub trait AppUniverseCore: Sized {
    /// The `Message` typically is an enum with multiple variants that define an
    /// action that could occur.
    type Message;

    /// The `msg` method should typically mutate state in some way. It should
    /// react to the variant of `Message` sent in as mutate the state.
    fn msg(&mut self, message: Self::Message);
}

/// This wrapper defines the type of a universe
impl<U: AppUniverseCore + 'static> AppUniverse<U> {
    /// This creates a new app_universe
    pub fn new(universe_core: U) -> Self {
        let universe = Rc::new(RefCell::new(universe_core));
        Self {
            universe,
            subscriptions: Rc::new(RefCell::new(vec![])),
        }
    }

    /// This method allows for mutation of state by sending a message
    pub fn msg(&self, msg: U::Message) {
        self.universe.borrow_mut().msg(msg);
        for subscriber in self.subscriptions.borrow_mut().iter() {
            (subscriber.borrow_mut().0)(self.clone());
        }
    }

    /// Acquire read access to the state.
    pub fn read(&self) -> Ref<'_, U> {
        self.universe.borrow()
    }

    /// This function takes a subscriber function that runs anytime the state changes.
    ///
    /// A subscriber function `subscriber_fn` is a function that will be called whenever state changes and it will pass in the updated state
    pub fn subscribe(
        &mut self,
        subscriber_fn: Box<dyn FnMut(AppUniverse<U>)>,
    ) -> UniverseSubscription<U> {
        let subscription = Rc::new(RefCell::new(Subscription(subscriber_fn)));

        let universe_subscription = UniverseSubscription(subscription.clone());

        self.subscriptions.borrow_mut().push(subscription.clone());

        universe_subscription
    }

    /// This function takes a subscription and removes the subscriber function so that it is no longer gets called whenever state changes
    pub fn unsubscribe(&mut self, subscription: UniverseSubscription<U>) -> Result<(), &str> {
        let sub_len_before = self.subscriptions.borrow().len();

        self.subscriptions
            .borrow_mut()
            .retain(|sub| !Rc::ptr_eq(sub, &subscription.0));

        let sub_len_after = self.subscriptions.borrow().len();

        if sub_len_before != sub_len_after {
            return Ok(());
        } else {
            return Err("Subscription not found");
        }
    }
}

impl<W: AppUniverseCore> Clone for AppUniverse<W> {
    fn clone(&self) -> Self {
        AppUniverse {
            universe: self.universe.clone(),
            subscriptions: self.subscriptions.clone(),
        }
    }
}
