//! app-universe provides a framework agnostic approach to managing frontend application state.
//!
//! # The API
//!
//! ## AppUniverseCore
//!
//! An `AppUniverseCore` is a trait that you iplement on a struct that holds your application state.
//!
//! Here's an example of what an `AppUniverseCore` for a basic e-commerce app frontend might look like:
//!
//! ```rust
//! struct MyAppUniverseCore {
//!     user: User,
//!     cart: Vec<Product>
//! }
//!
//! impl AppUniverseCore for MyAppUniverseCore {
//!     // We Will fill this in soon
//! }
//! ```
//!
//! Whenever a struct implements `AppUniverseCore`, it expects you to define 2 things:
//!
//! - A `Message` type
//! - A `msg` function
//!
//! ## The `Message` type
//! The Message type refers to an enum were each variant will be matched against in the `msg` function and in some way, will mutate state.
//! This Message acts like a `action` in `redux`
//!
//! ## The `msg` function matches against every variant in the `Message` enum and in each match branch, would in most cases mutate state in some way.
//! This is the ONLY way to mutate state
//!
//! Now let's continue with our implementation
//!
//! ```rust
//!
//! pub enum Msg {
//!     AddProductToCart(Product),
//! }
//!
//! impl AppUniverseCore for MyAppUniverseCore {
//!     type Message = Msg;
//!
//!     fn msg(&mut self, message: Self::Message) {
//!         match message {
//!             Msg::AddProductToCart(product) => {
//!                 self.cart.push(product);
//!             }
//!         }
//!     }
//! }
//! ```
//! ## The `AppUniverse`
//! The `AppUniverse` is what you will interract with most of the time.
//! The main way to create an app universe using the `create_universe` function.
//!
//! ```rust
//! let core = MyAppUniverseCore { user: User {}, cart: vec![] };
//!
//! let mut universe = create_universe(core);
//!
//! ```

#![deny(missing_docs)]

use std::sync::{Arc, RwLock, RwLockReadGuard};

/// Cloning an `AppUniverse` is a very cheap operation.
///
/// All clones hold pointers to the same inner state.
pub struct AppUniverse<U: AppUniverseCore> {
    universe: Arc<RwLock<U>>,
    subscribers: Arc<RwLock<Vec<Box<dyn Fn(UniverseWrapper<U>)>>>>,
}

/// This alloows you to create an app universe
/// TODO - Better Documentation
pub fn create_universe<U: AppUniverseCore + 'static>(universe_core: U) -> AppUniverse<U> {
    AppUniverse::new(universe_core)
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
pub type UniverseWrapper<U> = Arc<RwLock<U>>;

impl<U: AppUniverseCore + 'static> AppUniverse<U> {
    /// Create a new AppUniverseWrapper.
    fn new(universe_core: U) -> Self {
        let universe = Arc::new(RwLock::new(universe_core));
        Self {
            universe,
            subscribers: Arc::new(RwLock::new(vec![])),
        }
    }

    /// Acquire write access to the AppUniverse then send a message.
    pub fn msg(&self, msg: U::Message) {
        self.universe.write().unwrap().msg(msg);
        for subscriber in self.subscribers.read().unwrap().iter() {
            (subscriber)(self.universe.clone());
        }
    }

    /// Acquire read access to AppUniverse.
    pub fn read(&self) -> RwLockReadGuard<'_, U> {
        self.universe.read().unwrap()
    }

    /// Subscribe to the Universe
    pub fn subscribe(&mut self, subscriber_fn: Box<dyn Fn(UniverseWrapper<U>)>) {
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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    struct TestAppState {
        counter: u8,
    }

    pub enum Msg {
        Increment(u8),
    }

    impl AppUniverseCore for TestAppState {
        type Message = Msg;

        fn msg(&mut self, message: Self::Message) {
            match message {
                Msg::Increment(value) => {
                    self.counter += value;
                }
            }
        }
    }

    #[test]
    fn action_dispatch_works() {
        let state = TestAppState { counter: 0 };
        let universe = create_universe(state);

        universe.msg(Msg::Increment(3));

        assert_eq!(universe.read().counter, 3);
    }

    #[test]
    fn subscription_works() {
        use std::cell::RefCell;

        let some_value = Rc::new(RefCell::new(100));
        let some_value_clone = some_value.clone();
        let state = TestAppState { counter: 0 };

        let mut universe = create_universe(state);

        universe.subscribe(Box::new(move |universe| {
            let c = universe.read().unwrap().counter;
            *some_value_clone.borrow_mut() += c;
        }));

        universe.msg(Msg::Increment(1));

        assert_eq!(*some_value.borrow(), 101);
    }
}
