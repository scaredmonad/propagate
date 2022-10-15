use propagate::Event;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Status {
    Active,
    Inactive,
}

fn main() {
    let mut event: Event<Status> = Event::new();
    event.on(Status::Active, &|| println!("Status is active #1!"));
    event.on(Status::Active, &|| println!("Status is active #2!"));
    event.on(Status::Inactive, &|| println!("Status is inactive!"));
    event.dispatch(Status::Active);
}
