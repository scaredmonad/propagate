#![feature(fn_traits)]
#![feature(box_syntax)]
use std::hash::Hash;
use std::collections::HashMap;

pub struct Event<T, K> {
    pub handlers: HashMap<T, Vec<Box<dyn FnMut(K) -> ()>>>,
}

impl<T, K> Event<T, K>
where T: Eq + Hash, K: Copy + 'static {
    pub fn new() -> Self {
        Event {
            handlers: Default::default(),
        }
    }

    pub fn on(&mut self, event: T, handler: Box<dyn FnMut(K) -> ()>) {
        if self.handlers.contains_key(&event) {
            if let Some(handlers) = self.handlers.get_mut(&event) {
                handlers.push(handler);
            }
        } else {
            self.handlers.insert(event, vec![box(handler)]);
        }
    }

    pub fn dispatch(&mut self, event: T, data: K) {
        if self.handlers.contains_key(&event) {
            if let Some(handlers) = self.handlers.get_mut(&event) {
                handlers.iter_mut().for_each(move |f| {
                    f.call_mut((data,));
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(PartialEq, Eq, Hash, Debug)]
    enum Status {
        Active,
        Inactive,
    }
    #[derive(Debug, Clone, Copy)]
    struct Counter { count: i32 }
    #[test]
    fn single_dispatch() {
        let mut event: Event<Status, ()> = Event::new();
        event.on(Status::Active, box(|()| println!("Status is active!")));
        event.dispatch(Status::Active, ());
    }
    #[test]
    fn multi_dispatch() {
        let mut event: Event<Status, ()> = Event::new();
        event.on(Status::Active, box(|()| println!("Status is active #1!")));
        event.on(Status::Active, box(|()| println!("Status is active #2!")));
        event.dispatch(Status::Active, ());
    }
    #[test]
    fn alt_dispatch() {
        let mut event: Event<Status, ()> = Event::new();
        event.on(Status::Active, box(|()| println!("Status is active!")));
        event.on(Status::Inactive, box(|()| println!("Status is inactive!")));
        event.dispatch(Status::Active, ());
        event.dispatch(Status::Inactive, ());
    }
    #[test]
    fn slice_event_type() {
        let mut event: Event<&str, ()> = Event::new();
        event.on("active", box(|()| println!("Status is active!")));
        event.dispatch("active", ());
    }
    #[test]
    fn int_event_type() {
        let mut event: Event<core::primitive::i32, ()> = Event::new();
        event.on(0, box(|()| println!("Status is active!")));
        event.dispatch(0, ());
    }
    #[test]
    fn dispatch_with_data() {
        let counter = Counter { count: 0 };
        let mut event: Event<Status, Counter> = Event::new();
        event.on(Status::Active, box(|counter| assert_eq!(counter.count, 0)));
        event.dispatch(Status::Active, counter);
    }
    #[test]
    fn dispatch_with_mut_data() {
        let counter = Counter { count: 0 };
        let mut event: Event<Status, Counter> = Event::new();
        event.on(Status::Active, box(move |mut counter| {
            counter.count += 1;
            assert_eq!(counter.count, 1);
        }));
        event.dispatch(Status::Active, counter);
    }
    #[test]
    fn sequential_exec_dispatch_with_mut_data() {
        let counter = Counter { count: 0 };
        let mut event: Event<Status, Counter> = Event::new();
        event.on(Status::Active, box(move |mut counter| {
            counter.count += 1;
        }));
        event.on(Status::Active, box(move |mut counter| {
            counter.count += 1;
        }));
        event.on(Status::Active, box(move |counter| {
            assert_eq!(counter.count, 0);
        }));
        event.dispatch(Status::Active, counter);
        assert_eq!(counter.count, 0);
    }
}
