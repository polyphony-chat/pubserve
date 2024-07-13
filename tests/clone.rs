// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(not(feature = "async"))]
#[test]
fn test_clone() {
    use std::cell::RefCell;

    use pubserve::{Publisher, ReferenceCounted, Subscriber};

    struct MySubscriber {
        vec: RefCell<Vec<i32>>,
    }

    impl Subscriber<i32> for MySubscriber {
        fn update(&self, message: &i32) {
            self.vec.borrow_mut().push(*message);
            dbg!(self.vec.borrow());
        }
    }

    let mut publisher = Publisher::<i32>::new();
    let shared = ReferenceCounted::new(MySubscriber {
        vec: RefCell::new(Vec::new()),
    });
    publisher.subscribe(shared.clone());
    publisher.publish(42);
    assert!(shared.vec.borrow().contains(&42));
    // Test, if cloning the publisher, then dropping the original publisher, still allows the
    // subscriber to receive messages.
    let publisher_clone = publisher.clone();
    drop(publisher);
    publisher_clone.publish(43);
    assert!(shared.vec.borrow().contains(&43));
}
