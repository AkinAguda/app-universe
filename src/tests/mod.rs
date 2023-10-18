#[cfg(test)]
mod tests {
    use crate::app_universe::*;
    use std::rc::Rc;

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
        let universe = AppUniverse::new(state);

        universe.msg(Msg::Increment(3));

        assert_eq!(universe.read().counter, 3);
    }

    #[test]
    fn subscription_works_sync() {
        use std::cell::RefCell;

        let some_value = Rc::new(RefCell::new(100));
        let some_value_clone = some_value.clone();
        let state = TestAppState { counter: 0 };

        let mut universe = AppUniverse::new(state);

        universe.subscribe(Box::new(move |universe| {
            let c = universe.read().counter;
            *some_value_clone.borrow_mut() += c;
        }));

        universe.msg(Msg::Increment(1));

        assert_eq!(*some_value.borrow(), 101);
    }

    #[test]
    fn unsubscription_works() {
        use std::cell::RefCell;

        let some_value = Rc::new(RefCell::new(100));
        let some_value_clone = some_value.clone();
        let state = TestAppState { counter: 0 };

        let mut universe = AppUniverse::new(state);

        let subscription = universe.subscribe(Box::new(move |universe| {
            let c = universe.read().counter;
            *some_value_clone.borrow_mut() += c;
        }));

        universe.msg(Msg::Increment(1));

        universe.unsubscribe(subscription).unwrap();

        universe.msg(Msg::Increment(1));

        assert_eq!(*some_value.borrow(), 101);
    }
    /* TODO Expose some methods to test the number of subs when unsubs are made*/
}
