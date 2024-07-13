// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(not(feature = "send"))]
use std::rc::Rc;
#[cfg(not(feature = "send"))]
pub type ReferenceCounted<T> = Rc<T>;
#[cfg(feature = "send")]
use std::sync::Arc;
#[cfg(feature = "send")]
pub type ReferenceCounted<T> = Arc<T>;

#[cfg(feature = "send")]
#[cfg(feature = "async")]
#[async_trait::async_trait]
/// Subscribers are notified when a Publisher sends a message. They receive a reference to the
/// message and can choose to do whatever they want with it.
pub trait Subscriber<T>: Send + Sync {
    async fn update(&self, message: &T);
}

#[cfg(not(feature = "send"))]
#[cfg(feature = "async")]
#[async_trait::async_trait]
/// Subscribers are notified when a Publisher sends a message. They receive a reference to the
/// message and can choose to do whatever they want with it.
pub trait Subscriber<T>: Sync {
    async fn update(&self, message: &T);
}

#[cfg(feature = "send")]
#[cfg(not(feature = "async"))]
/// Subscribers are notified when a Publisher sends a message. They receive a reference to the
/// message and can choose to do whatever they want with it.
pub trait Subscriber<T>: Send {
    fn update(&self, message: &T);
}

#[cfg(not(feature = "send"))]
#[cfg(not(feature = "async"))]
/// Subscribers are notified when a Publisher sends a message. They receive a reference to the
/// message and can choose to do whatever they want with it.
pub trait Subscriber<T> {
    fn update(&self, message: &T);
}

/// Publishers are responsible for sending messages to all of their subscribers. They keep a list
/// of subscribers and offer methods to add and remove subscribers from the list. It does not keep
/// track of the messages it sends or offer any other behavior.
///
/// # Example
/// ```
/// use pubserve::Publisher;
/// let mut publisher = Publisher::<String>::new();
/// publisher.publish("Hello, World!".to_string()); // .await, if async feature is enabled
/// ```
pub struct Publisher<T> {
    subscribers: Vec<ReferenceCounted<dyn Subscriber<T>>>,
}

impl<T> std::default::Default for Publisher<T> {
    fn default() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }
}

impl<T> Publisher<T> {
    /// Create a new Publisher with no subscribers.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if the Publisher has any subscribers.
    pub fn has_subscribers(&self) -> bool {
        !self.subscribers.is_empty()
    }

    /// Add a subscriber to the Publishers list of subscribers. The subscriber will be notified
    /// when the Publisher sends a message.
    ///
    /// ## Example
    ///
    /// The example assumes that the `async` feature is not enabled. If you have enabled the `async`
    /// feature, you have to add `#[async_trait]` to the `Subscriber` trait impl and `async` to the
    /// `update` method, as well as `.await`ing the `publish` method. Otherwise, the example is
    /// identical.
    ///
    /// ```
    /// use pubserve::{Publisher, Subscriber};
    ///
    /// struct MySubscriber;
    ///
    /// // A simple subscriber that prints the message it receives.
    /// impl Subscriber<String> for MySubscriber {
    ///     fn update(&self, message: &String) {
    ///        println!("Received this message: {}", message);
    ///     }
    /// }
    ///
    /// let mut publisher = Publisher::<String>::new();
    /// let subscriber = MySubscriber;
    /// let rc_subscriber = pubserve::ReferenceCounted::new(subscriber);
    /// publisher.subscribe(rc_subscriber.clone());
    /// // This will print "Received this message: Hello, World!"
    /// publisher.publish("Hello, World!".to_string());
    /// publisher.unsubscribe(rc_subscriber.clone());
    /// // The subscriber has been removed, so this will not print anything.
    /// publisher.publish("Hello, World!".to_string());
    ///
    /// // The following subscribing/unsubscribing will NOT work. If you do not understand why,
    /// // please read up on how reference counting works in Rust.
    /// publisher.subscribe(rc_subscriber.clone());
    /// publisher.unsubscribe(pubserve::ReferenceCounted::new(MySubscriber));
    /// ```
    pub fn subscribe(&mut self, subscriber: ReferenceCounted<dyn Subscriber<T>>) {
        self.subscribers.push(subscriber);
    }

    /// Remove a subscriber from the Publishers list of subscribers. The subscriber will no longer
    /// be notified when the Publisher sends a message.
    ///
    /// Important: This method uses the `ReferenceCounted::ptr_eq` method to compare the pointers
    /// of the subscriber to remove with the pointers of the subscribers in the list. The
    /// [ReferenceCounted] subscriber passed in must point to the same address as the subscriber
    /// that was added to the list.
    ///
    /// ## Example
    ///
    /// The example assumes that the `async` feature is not enabled. If you have enabled the `async`
    /// feature, you have to add `#[async_trait]` to the `Subscriber` trait impl and `async` to the
    /// `update` method, as well as `.await`ing the `publish` method. Otherwise, the example is
    /// identical.
    ///
    /// ```
    /// use pubserve::{Publisher, Subscriber};
    ///
    /// struct MySubscriber;
    ///
    /// // A simple subscriber that prints the message it receives.
    /// impl Subscriber<String> for MySubscriber {
    ///     fn update(&self, message: &String) {
    ///        println!("Received this message: {}", message);
    ///     }
    /// }
    ///
    /// let mut publisher = Publisher::<String>::new();
    /// let subscriber = MySubscriber;
    /// let rc_subscriber = pubserve::ReferenceCounted::new(subscriber);
    /// publisher.subscribe(rc_subscriber.clone());
    /// // This will print "Received this message: Hello, World!"
    /// publisher.publish("Hello, World!".to_string());
    /// publisher.unsubscribe(rc_subscriber.clone());
    /// // The subscriber has been removed, so this will not print anything.
    /// publisher.publish("Hello, World!".to_string());
    ///
    /// // The following subscribing/unsubscribing will NOT work. If you do not understand why,
    /// // please read up on how reference counting works in Rust.
    /// publisher.subscribe(rc_subscriber.clone());
    /// publisher.unsubscribe(pubserve::ReferenceCounted::new(MySubscriber));
    /// ```
    pub fn unsubscribe(&mut self, subscriber: ReferenceCounted<dyn Subscriber<T>>) {
        self.subscribers
            .retain(|s| !ReferenceCounted::ptr_eq(s, &subscriber));
    }

    #[cfg(not(feature = "async"))]
    /// Publish a message to all subscribers.
    pub fn publish(&self, message: T) {
        for subscriber in &self.subscribers {
            subscriber.update(&message);
        }
    }

    #[cfg(feature = "async")]
    /// Publish a message to all subscribers.
    pub async fn publish(&self, message: T) {
        for subscriber in &self.subscribers {
            subscriber.update(&message).await;
        }
    }
}

#[cfg(test)]
#[cfg(not(feature = "async"))]
#[test]
fn test_publisher() {
    struct MySubscriber;
    impl Subscriber<String> for MySubscriber {
        fn update(&self, message: &String) {
            println!("Received this message: {}", message);
        }
    }

    let mut publisher = Publisher::<String>::new();
    let subscriber = MySubscriber;
    let subscriber = ReferenceCounted::new(subscriber);
    publisher.subscribe(subscriber.clone());
    publisher.publish("Hello, World!".to_string());
    publisher.unsubscribe(subscriber);
    publisher.publish("Hello, World!".to_string());
}

#[cfg(test)]
#[cfg(feature = "async")]
#[tokio::test]
async fn test_publisher() {
    struct MySubscriber;
    #[async_trait::async_trait]
    impl Subscriber<String> for MySubscriber {
        async fn update(&self, message: &String) {
            println!("Received this message: {}", message);
        }
    }

    let mut publisher = Publisher::<String>::new();
    let subscriber = MySubscriber;
    let subscriber = ReferenceCounted::new(subscriber);
    publisher.subscribe(subscriber.clone());
    publisher.publish("Hello, World!".to_string()).await;
    publisher.unsubscribe(subscriber);
    publisher.publish("Hello, World!".to_string()).await;
}
