# propagate

Synchronous event emitter for trivial systems without higher-rank trait bounds. `propagate` is based on sync events with `FnMut` handlers.

## Installation

Point to this repository in your `Cargo.toml`.

## Usage

An event can be of any type: `u32` or other number types, `&str` slices or even enums, which we will use in our example.

```rust
use propagate::Event;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Status {
    Active,
    Inactive,
}

let mut event: Event<Status, ()> = Event::new();

event.on(Status::Active, box(|()| println!("Status is active #1!")));
event.on(Status::Active, box(|()| println!("Status is active #2!")));
event.on(Status::InActive, box(|()| println!("Status is inactive!")));

event.dispatch(Status::Active, ());
```

To pass data to the boxed mutable closures on dispatch calls, explicitly pass the type to the event and call dispatch with the instance. Note that the `Copy + Clone` constraint is required:

```rust
#[derive(Debug, Clone, Copy)]
struct Counter { count: i32 }

let counter = Counter { count: 0 };
let mut event: Event<Status, Counter> = Event::new();

event.on(Status::Active, box(|counter| assert_eq!(counter.count, 0)));

event.dispatch(Status::Active, counter);
```

More examples can be found in the unit tests in `lib.rs`.

## Caveat

Data wrapped with `Rc<RefMut<T>>` cannot be changed from within the closures (i.e. using `RefMut`) and outside of it since `'static` needs to be implemented in the closure scope which moves it and so the data gets owned. There is likely a workaround.

```rust
use std::rc::Rc;
use std::borrow::Borrow;
use std::cell::{RefCell, Ref, RefMut};

let mut counter: Rc<RefCell<Counter>> = Rc::new(RefCell::new(Counter { count: 0 }));
let mut event: Event<Status, ()> = Event::new();

event.on(Status::Active, box(move |_| {
    let counter: &mut RefMut<Counter> = &mut counter.borrow_mut();
    counter.count += 1;
}));

event.dispatch(Status::Active, ());

assert_eq!(&mut counter.borrow_mut().count, &mut 1);
```

## License

MIT License © Abdullahi M.
