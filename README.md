# spectra

Simple, generic observer pattern implementation for Rust.

## Usage

spectra is really simple to use. Start by creating a `Publisher` for your data type:

```rust
use spectra::Publisher;

let publisher = Publisher::<i32>::new();
```

Then, create a subscriber that will listen to the publisher:

```rust
use spectra::Subscriber;

struct MySubscriber;

impl Subscriber<i32> for MySubscriber {
    fn update(&self, data: &i32) {
        // Do whatever you want with the data!
        println!("Received data: {}", data);
    }
}
```

Finally, subscribe the observer to the publisher:

```rust
use spectra::ReferenceCounted;

let observer = MySubscriber;
let reference = ReferenceCounted::new(&observer);
publisher.subscribe(reference);
```

Now, whenever you publish data, all subscribed observers will be notified:

```rust
publisher.publish(42);
// STDOUT: "Received data: 42"
```

Unsubscribing is just as easy:

```rust
publisher.unsubscribe(reference);
publisher.publish(42);
// Nothing will be printed
```

It is of course possible to have multiple implementations of `Subscriber` be subscribed to the
same `Publisher`. All of them will be notified immediately when data is published.

## Features

No features are enabled by default. The following features are available:

| Feature | Description                                                                                                          |
| ------- | -------------------------------------------------------------------------------------------------------------------- |
| `send`  | Makes the `Subscriber` trait `Send` and thus thread-safe.                                                            |
| `async` | Makes the `Subscriber` trait `Sync` and `async` via the [`async_trait` crate](https://crates.io/crates/async-trait). |

The `send` and `async` features can both be enabled at the same time, but do not have to be, should
you only need one of them.
