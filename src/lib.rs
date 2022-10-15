#![feature(fn_traits)]
#![feature(box_syntax)]
use std::collections::HashMap;

pub struct Event<'a, T> {
    pub handlers: HashMap<T, Vec<Box<&'a dyn FnMut() -> ()>>>,
}

impl<'a, T> Event<'a, T>
where T: Eq + std::hash::Hash {
    pub fn new() -> Self {
        Event {
            handlers: Default::default(),
        }
    }

    pub fn on(&mut self, event: T, handler: &'a dyn FnMut() -> ()) {
        if self.handlers.contains_key(&event) {
            if let Some(handlers) = self.handlers.get_mut(&event) {
                handlers.push(box(handler));
            }
        } else {
            self.handlers.insert(event, vec![box(handler)]);
        }
    }

    pub fn dispatch(&mut self, event: T) {
        if self.handlers.contains_key(&event) {
            println!("Found!!!");
            if let Some(mut handlers) = self.handlers.get_mut(&event) {
                handlers.iter_mut().for_each(move |f| {
                    // f.call_mut(());
                    println!("Called!");
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
    #[test]
    fn it_works() {
        let mut event: Event<Status> = Event::new();
        event.on(Status::Active, &|| println!("Status is active #1!"));
        event.on(Status::Active, &|| println!("Status is active #2!"));
        event.on(Status::Inactive, &|| println!("Status is inactive!"));
        event.dispatch(Status::Active);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
