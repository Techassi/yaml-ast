use crate::{
    events::{Event, IntoEvents},
    nodes::Node,
};

#[derive(Debug)]
pub struct Sequence(Vec<Node>);

impl<const N: usize> From<[Node; N]> for Sequence {
    fn from(value: [Node; N]) -> Self {
        Self(Vec::from(value))
    }
}

impl IntoEvents for Sequence {
    fn into_events(&self, events: &mut Vec<Event>) {
        events.push(Event::SequenceStart);

        for node in &self.0 {
            events.push(Event::SequenceItemStart);
            node.into_events(events);
            events.push(Event::SequenceItemEnd);
        }

        events.push(Event::SequenceEnd);
    }
}
