# [App-Universe](https://github.com/AkinAguda/app-universe)

A framework agnostic approach to managing frontend application state based on [app-world](https://github.com/chinedufn/app-world).

# Example Usage

```rust
mod app_universe;
use app_universe::{ AppUniverse, AppUniverseCore };

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

fn main () {
    let state = TestAppState { counter: 0 };
    let mut universe = AppUniverse::new(state);

    universe.msg(Msg::Increment(1));

    let subscription = universe.subscribe(Box::new(move |universe| {
        println!("Counter value is {}", universe.read().counter);
    }));

    universe.msg(Msg::Increment(1));

    universe.unsubscribe(subscription).unwrap();
}
```

## Inspiration

- [App-World](https://crates.io/crates/app-world)
