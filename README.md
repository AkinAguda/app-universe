[# app-universe](https://github.com/AkinAguda/app-universe)

> A framework agnostic approach to managing frontend application state based on [app-world](https://github.com/chinedufn/app-world).

# Example Usage

```rust
use app_universe::AppUniverseCore;

struct TestAppState {
    counter: u8,
}

struct MyAppState {
    count: u32
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
    let mut universe = create_universe(state);

    universe.msg(Msg::Increment(1));
}
```

## Inspiration

- [App-World](https://crates.io/crates/app-world)
