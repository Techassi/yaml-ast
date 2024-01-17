use crate::{
    events::{Event, IntoEvents},
    nodes::{Mapping, Node},
};

pub mod emitter;
pub mod events;
pub mod nodes;

pub enum Error {}

/// A stream represents one or more [`Document`]s separated by `---`
/// (triple dash) and `...` (triple dot).
#[derive(Debug, Default)]
pub struct Stream(Vec<Document>);

impl Stream {
    /// Creates a new (empty) stream of YAML [`Document`]s.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends one YAML [`Document`] at the end of the stream.
    pub fn push_document(&mut self, document: Document) -> &mut Self {
        self.0.push(document);
        self
    }
}

impl IntoEvents for Stream {
    fn into_events(self) -> Vec<Event> {
        let mut events = Vec::new();
        events.push(Event::StreamStart);

        for doc in self.0 {
            events.extend(doc.into_events())
        }

        events.push(Event::StreamEnd);
        events
    }
}

/// A document is part (or chunk) of a larger [`Stream`].
///
/// Each document can have zero or more directives attached to it. These
/// directives influence the behavior of the YAML processor. The content of the
/// document is stored in zero or more [`Node`]s.
#[derive(Debug, Default)]
pub struct Document {
    pub directives: Vec<String>,
    pub nodes: Vec<Node>,
}

impl IntoEvents for Document {
    fn into_events(self) -> Vec<Event> {
        let mut events = Vec::new();
        events.push(Event::DocumentStart);

        for node in self.nodes {
            events.extend(node.into_events())
        }

        events.push(Event::DocumentEnd);
        events
    }
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    /// Convenience function to create a new document from a mapping. Most YAML
    /// documents have a mapping node as their root node.
    pub fn from_mapping(mapping: Mapping) -> Self {
        Self {
            nodes: Vec::from([Node::Mapping(mapping)]),
            ..Default::default()
        }
    }

    pub fn push_directive(&mut self, directive: String) -> &mut Self {
        self.directives.push(directive);
        self
    }

    pub fn push_node(&mut self, node: Node) -> &mut Self {
        self.nodes.push(node);
        self
    }
}

#[derive(Debug)]
pub enum ScopedTag {
    Global(Node),

    // TODO (Techassi): Let's see how we can deal with custom tags
    Local(Node),
}

impl Default for ScopedTag {
    fn default() -> Self {
        Self::Global(Node::default())
    }
}

/// Type alias for a [`Vec<Node>`].
pub type Sequence = Vec<Node>;

#[cfg(test)]
mod test {
    use crate::emitter::{Emitter, EmitterOptions};

    use super::*;

    #[test]
    fn basic() {
        let map = Mapping::from([
            (
                Node::String("clusterName".into()),
                Node::String("opensearch-cluster".into()),
            ),
            (
                Node::String("nodeGroup".into()),
                Node::String("master".into()),
            ),
            (Node::String("singleNode".into()), Node::Boolean(false)),
            (
                Node::String("roles".into()),
                Node::Sequence(Sequence::from([
                    Node::String("master".into()),
                    Node::String("ingest".into()),
                ])),
            ),
            (Node::String("replicas".into()), Node::Integer(3)),
            (
                Node::String("global".into()),
                Node::Mapping(Mapping::from([(
                    Node::String("dockerRegistry".into()),
                    Node::String("test".into()),
                )])),
            ),
        ]);

        let doc = Document::from_mapping(map);

        let mut stream = Stream::new();
        stream.push_document(doc);

        let events = stream.into_events();
        let mut output = String::new();

        let emitter = Emitter::new(events, EmitterOptions::default());
        emitter.emit(&mut output).unwrap();

        // println!("{events:?}");
        println!("{output}")
    }
}
