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
    fn unsubscription_works_sync() {
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

        universe.unsubscribe(subscription);

        universe.msg(Msg::Increment(1));

        assert_eq!(*some_value.borrow(), 101);
    }

    #[test]
    fn multiple_subscriptions_unsubscriptions_works_sync() {
        use std::cell::RefCell;

        let some_value = Rc::new(RefCell::new(100));
        let some_other_value = Rc::new(RefCell::new(0));
        let some_value_clone = some_value.clone();
        let some_other_value_clone = some_other_value.clone();
        let state = TestAppState { counter: 0 };

        let mut universe = AppUniverse::new(state);

        let sub1 = universe.subscribe(Box::new(move |universe| {
            let c = universe.read().counter;
            *some_value_clone.borrow_mut() += c;
        }));

        let sub2 = universe.subscribe(Box::new(move |universe| {
            let g = universe.read().counter;
            *some_other_value_clone.borrow_mut() += g;
        }));

        // This should make some_value increase by 1 => 101
        // This should aslo make some_other_value increase by 1 => 1
        universe.msg(Msg::Increment(1));

        // This should unsubscribe the some_other_value increment function
        universe.unsubscribe(sub2);

        // This should make some_value increase by the new value of counter which is 2 => 103
        universe.msg(Msg::Increment(1));

        // This should unsubscribe the some_value increment function
        universe.unsubscribe(sub1);

        // This should only cause an increment in state
        universe.msg(Msg::Increment(1));

        assert_eq!(*some_value.borrow(), 103);
        assert_eq!(*some_other_value.borrow(), 1);
    }
}
