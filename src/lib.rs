//! app-universe provides a framework agnostic approach to managing frontend application state.
//!
//! # The Data Model
//!
//! An `AppUniverse` is a type that you define that holds your application state as well as other
//! resources that you've deemed useful to have around during your application's runtime.
//!
//! Here's an example of what an AppUniverse for a basic e-commerce app frontend might look like:
//!
//! ```rust
//! # use std::collections::HashMap;
//! struct MyAppUniverse {
//!     state: MyAppState,
//!     resources: MyAppResources
//! }
//!
//! struct MyAppState {
//!     user: User,
//!     products: HashMap<Uuid, Product>
//! }
//!
//! struct MyAppResources {
//!     file_store: Box<dyn MyFileStoreTrait>,
//!     api_client: ApiClient
//! }
//!
//! # trait MyFileStoreTrait {};
//! # type ApiClient = ();
//! # type Product = ();
//! # type User = ();
//! # type Uuid = ();
//! ```
//!
//! The `MyAppUniverse` struct would be defined in your crate, but it wouldn't be used directly when
//! you were passing data around to your views.
//!
//! Instead, you wrap it in an `app_universe::AppUniverseWrapper<W>`
//!
//! ```rust
//! type MyAppUniverseWrapper = app_universe::AppUniverseWrapper<MyAppUniverse>;
//!
//! # type MyAppUniverse = ();
//! ```
//!
//! # AppUniverseWrapper<W: AppUniverse>
//!
//! The `AppUniverseWrapper` prevents direct mutable access to your application state, so you cannot
//! mutate fields wherever you please.
//!
//! Instead, the [`AppUniverse`] trait defines a [`AppUniverse.msg()`] method that can be used to update
//! your application state.
//!
//! You can pass your `AppUniverseWrapper<W>` to different threads by calling
//! [`AppUniverseWrapper.clone()`]. Under the hood an [`Arc`] is used to share your data across
//! threads.
//!
//! # Example Usage
//!
//! TODO
//!
//! # When to Use app-universe
//!
//! app-universe shines in applications that do not have extreme real time rendering requirements,
//! such as almost all browser, desktop and mobile applications.
//! In games and real-time simulations, you're better off using something like an entity component
//! system to manage your application state.
//!
//! This is because app-universe is designed such that your application state can only be written to
//! from one thread at a time. This is totally fine for almost all browser, desktop and mobile
//! applications, but could be an issue for games and simulations.
//!
//! If you're writing a game or simulation you're likely better off reaching for an
//! entity-component-system library. Otherwise, you should be in good hands here.
//! which could be an issue for a high-performing game or simulation.

#![deny(missing_docs)]

use std::sync::{Arc, RwLock, RwLockReadGuard};

/// Holds application state and resources.
/// See the [crate level documentation](crate) for more details.
///
/// # Cloning
///
/// Cloning an `AppUniverseWrapper` is a very cheap operation.
///
/// All clones hold pointers to the same inner state.
pub struct AppUniverse<U: AppUniverseCore> {
    universe: Arc<RwLock<U>>,
}

/// This alloows you to create an app universe
/// TODO - Better Documentation
pub fn create_universe<U: AppUniverseCore + 'static>(universe_core: U) -> AppUniverse<U> {
    AppUniverse::new(universe_core)
}

/// Defines how messages that indicate that something has happened get sent to the universe.
pub trait AppUniverseCore: Sized {
    /// Indicates that something has happened.
    ///
    /// ```
    /// # use std::time::SystemTime;
    /// #[allow(unused)]
    /// enum MyMessageType {
    ///     IncreaseClickCounter,
    ///     SetLastPausedAt(SystemTime)
    /// }
    /// ```
    type Message;

    /// Send a message to the state object.
    /// This will usually lead to a state update
    fn msg(&mut self, message: Self::Message);
}

impl<U: AppUniverseCore + 'static> AppUniverse<U> {
    /// Create a new AppUniverseWrapper.
    fn new(universe_core: U) -> Self {
        let universe = Arc::new(RwLock::new(universe_core));
        Self { universe }
    }

    /// Acquire write access to the AppUniverse then send a message.
    pub fn msg(&self, msg: U::Message) {
        self.universe.write().unwrap().msg(msg)
    }

    /// Acquire read access to AppUniverse.
    pub fn read(&self) -> RwLockReadGuard<'_, U> {
        self.universe.read().unwrap()
    }

    /// Subscribe to the Universe
    pub fn subscribe(&self) {
        // Here I want to add subscription logic
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
        }
    }
}

#[cfg(test)]
mod tests {
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
        let mut some_value_to_update = 100;
        let state = TestAppState { counter: 0 };
        let universe = create_universe(state);
    }
}
