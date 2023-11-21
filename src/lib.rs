//! `app-universe` provides a framework agnostic approach to managing frontend application state.
//! It essentially funtions like [redux](https://github.com/reduxjs/redux) from the javascript ecosystem.
//!
//! # Example Usage
//!
//! ```rust
//! use app_universe::{ AppUniverse, AppUniverseCore };
//!
//! struct TestAppState {
//!     counter: u8,
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
//! fn main() {
//!     let state = TestAppState { counter: 0 };
//!     let mut universe = AppUniverse::new(state);
//!
//!     universe.msg(Msg::Increment(1));
//!
//!     let subscription = universe.subscribe(Box::new(move |universe| {
//!         println!("Counter value is {}", universe.read().counter);
//!     }));
//!
//!     universe.msg(Msg::Increment(1));
//!
//!     universe.unsubscribe(subscription).unwrap();
//! }
//! ```
//!
//! # The API
//! The core API is mostly defined by the `AppUniverseCore` trait and the `AppUniverse` struct.
//!
//! ## The `AppUniverseCore` trait
//!
//! The `AppUniverseCore` trait is intended to be implemented for the struct that holds your application state.
//!
//! Let's implement `AppUniverseCore` for our application state. First we define our state:
//!
//! ```rust
//! use app_universe::{ AppUniverse, AppUniverseCore };
//!
//! struct Product {
//!    id: u16
//! }
//! struct MyAppState {
//!     cart: Vec<Product>
//! }
//!
//! # enum Msg {
//! #   AddProductToCart(Product),
//! # }
//! #
//! impl AppUniverseCore for MyAppState {
//!    # type Message = Msg;
//!    # fn msg(&mut self, message: Self::Message) {
//!    #   todo!();
//!    # }
//!     // We Will fill this in soon
//! }
//!
//! fn main() {
//!     // We will populate this soon
//! }
//! ```
//! Whenever a struct implements `AppUniverseCore`, it expects you to define 2 things:
//!
//! - A `Message` type
//! - A `msg` function
//!
//! ## The `Message` type
//! The Message type alias refers to an enum were each variant will be matched against in the `msg` function.
//! This Message acts like a `action` in `redux`.
//!
//! ## The `msg` function
//! The `msg` function is the **ONLY** way to mutate state. The `msg` function matches against every variant in the `Message` enum. In each match branch you are expected to mutate state in some way. This is the **ONLY** plave you can mutate state.
//!
//! Now let's continue with our implementation
//!
//! ```rust
//! # use app_universe::{ AppUniverse, AppUniverseCore };
//! #
//! # struct Product {
//! #  id: u16
//! # }
//! # struct MyAppState {
//! #   cart: Vec<Product>
//! # }
//! // ...
//! enum Msg {
//!   AddProductToCart(Product),
//! }
//!
//! impl AppUniverseCore for MyAppState {
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
//! fn main() {
//!     // We will populate this soon
//! }
//! ```
//! ## The `AppUniverse`
//! The `AppUniverse` struct is what you will interract with most of the time.
//! In our main function, we want to create a new app universe.
//!
//! ```rust
//! # use app_universe::{ AppUniverse, AppUniverseCore };
//! #
//! # struct Product {
//! #  id: u16
//! # }
//! # struct MyAppState {
//! #   cart: Vec<Product>
//! # }
//!
//! # enum Msg {
//! # AddProductToCart(Product),
//! # }
//! #
//! # impl AppUniverseCore for MyAppState {
//! #   type Message = Msg;
//! #
//! #   fn msg(&mut self, message: Self::Message) {
//! #       match message {
//! #           Msg::AddProductToCart(product) => {
//! #               self.cart.push(product);
//! #           }
//! #       }
//! #   }
//! # }
//! // ...
//! fn main() {
//!     let core = MyAppState { cart: vec![] };
//!     let mut universe = AppUniverse::new(core);
//! }
//! ```
//! ## Subscribing to the `AppUniverse`
//! Subscribing to the `AppUniverse` essentially means passing a callback that should be called whenever state changes.
//! A subscriber function will recieve the `AppUniverse` as an argument whenever it's called. Let's subscribe to out universe in our example.
//!
//! ```rust
//! # use app_universe::{ AppUniverse, AppUniverseCore };
//! #
//! # struct Product {
//! #  id: u16
//! # }
//! #
//! # struct MyAppState {
//! #   cart: Vec<Product>
//! # }
//! #
//! # enum Msg {
//! # AddProductToCart(Product),
//! # }
//! # impl AppUniverseCore for MyAppState {
//! #   type Message = Msg;
//! #
//! #   fn msg(&mut self, message: Self::Message) {
//! #       match message {
//! #           Msg::AddProductToCart(product) => {
//! #               self.cart.push(product);
//! #           }
//! #       }
//! #   }
//! # }
//! fn main() {
//!     let core = MyAppState { cart: vec![] };
//!     let mut universe = AppUniverse::new(core);
//!     
//!     let subscription = universe.subscribe(Box::new(|universe| { /* Do something */ }));
//! }
//! ```

mod app_universe;
mod tests;
pub use crate::app_universe::*;

// I want the subscription to be removed when the subscriptions go out of scope
