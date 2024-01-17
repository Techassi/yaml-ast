/// This trait is used to turn higher level representations of a YAML stream
/// into an event stream. These event streams can be produced/consumed by
/// high and low-level components.
pub trait IntoEvents {
    /// Turns the stream of documents into a list of ordered events.
    ///
    /// These events are used by the emitter to write the event tree as a
    /// character stream in a human-friendly manner. This is the last step in
    /// the "dump" sequence.
    ///
    /// #### Reference
    ///
    /// - <https://yaml.org/spec/1.2.2/#serializing-the-representation-graph>
    /// - <https://yaml.org/spec/1.2.2/#presenting-the-serialization-tree>
    fn into_events(self) -> Vec<Event>;
}

pub trait FromEvents {
    fn from_events(events: Vec<Event>) -> Self;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    StreamStart,
    StreamEnd,
    DocumentStart,
    DocumentEnd,
    Alias(usize),
    Scalar(String),
    SequenceStart(usize),
    SequenceEnd,
    MappingStart(usize),
    MappingKey,
    MappingValue,
    MappingEnd,
}
