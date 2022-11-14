//! `app-universe` provides a framework agnostic approach to managing frontend application state.
//!
//! # Example Usage
//!
//! ```rust
//! use app_universe::AppUniverseCore;
//!
//! struct TestAppState {
//!     counter: u8,
//! }
//!
//! struct MyAppState {
//!     count: u32
//! }
//!
//! pub enum Msg {
//!     Increment(u8),
//! }
//!
//! impl AppUniverseCore for TestAppState {
//!     type Message = Msg;
//!
//!     fn msg(&mut self, message: Self::Message) {
//!         match message {
//!             Msg::Increment(value) => {
//!                 self.counter += value;
//!             }
//!         }
//!     }
//! }
//!
//!
//!
//! fn main () {
//!     let state = TestAppState { counter: 0 };
//!     let mut universe = AppUniverse::new(state);
//!
//!     universe.msg(Msg::Increment(1));
//! }
//! ```
//!
//! # The API
//! The core API is mostly defined by the `AppUniverseCore` trait and the The `AppUniverse` struct.
//!
//! ## The `AppUniverseCore` trait
//!
//! The `AppUniverseCore` trait is intended to be implemented for the struct that holds your application state.
//!
//! Here's an example of what some state for a basic e-commerce app that implements `AppUniverseCore` might look like:
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
//! The Message type alias refers to an enum were each variant will be matched against in the `msg` function.
//! This Message acts like a `action` in `redux`
//!
//! ## The `msg` function
//! The `msg` function is the **ONLY** way to mutate state. The `msg` function matches against every variant in the `Message` enum. In each match branch you are expected to mutate state in some way. This is the **ONLY** plave you can mutate state.
//!
//! Now let's continue with our implementation
//!
//! ```rust
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
//! The `AppUniverse` struct is what you will interract with most of the time.
//!
//! ```rust
//! let core = MyAppUniverseCore { user: User {}, cart: vec![] };
//!
//! let mut universe = AppUniverse::new(core);
//!
//! ```

/// This is the app_universe
pub mod app_universe;
mod tests;
