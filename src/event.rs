use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, PartialEq, Debug)]
pub enum EventNames {
    Sit,
    Stand,
    Walk,
    Flag,
    About,
    Main,
}

#[derive(Clone, Debug)]
pub struct Event {
    pub name: EventNames,
}

pub struct EventQueue {
    subscribers: HashMap<String, VecDeque<Box<Event>>>,
}

/// simple event queue, any subscriber can access any event pool
/// if subscriber consumes events from another pool the events will be lost
impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            subscribers: HashMap::new(),
        }
    }
    pub fn push(&mut self, e: Box<Event>) {
        self.subscribers
            .iter_mut()
            .for_each(|(_, x)| x.push_back(e.clone()));
    }

    /// creates an event pool for a subscriber
    pub fn subscribe(&mut self, subscriber: &str) {
        &self
            .subscribers
            .insert(subscriber.to_string(), VecDeque::new());
    }

    fn subscriber(&mut self, subscriber: &str) -> &mut VecDeque<Box<Event>> {
        self.subscribers.get_mut(subscriber).unwrap()
    }

    /// gets all events for subscriber, empties the event pool
    pub fn poll(&mut self, subscriber: &str) -> Vec<Box<Event>> {
        self.subscriber(subscriber).drain(..).collect()
    }
}
