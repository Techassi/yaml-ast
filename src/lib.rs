use std::collections::BTreeMap;

use crate::events::{Event, IntoEvents};

pub mod emitter;
pub mod events;

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

/// Type alias for a [`BTreeMap<Node, Node>`].
pub type Mapping = BTreeMap<Node, Node>;

/// Type alias for a [`Vec<Node>`].
pub type Sequence = Vec<Node>;

/// A YAML schema is a combination of a set of tags and a mechanism for
/// resolving non-specific tags.
///
/// The tags are specified in different schemas. The spec defined three main
/// schemas: Failsafe, JSON, and Core. The tags are grouped in the following
/// way:
///
/// - Failsafe Schema:
///     - Mapping
///     - Sequence
///     - String
/// - JSON Schema:
///     - Null
///     - Boolean
///     - Integer
///     - Floating Point
///
/// #### Note
///
/// The YAML specification defines nodes and tags a two separate (but related)
/// concepts. Because Rust allows us to combine enums with structured data,
/// this crate decides to combine both these concepts into one.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    /// Represents an associative container, where each key is unique in the
    /// association and mapped to exactly one value.
    ///
    /// See <https://yaml.org/spec/1.2.2/#10111-generic-mapping>
    Mapping(BTreeMap<Node, Node>),

    /// Represents a collection indexed by sequential integers starting with
    /// zero.
    ///
    /// See <https://yaml.org/spec/1.2.2/#10112-generic-sequence>
    Sequence(Vec<Node>),

    /// Represents a Unicode string, a sequence of zero or more Unicode
    /// characters.
    ///
    /// See <https://yaml.org/spec/1.2.2/#0113-generic-string>
    String(String),

    /// Represents the lack of a value.
    ///
    /// See <https://yaml.org/spec/1.2.2/#10211-null>
    Null,

    /// Represents a true/false value.
    ///
    /// See <https://yaml.org/spec/1.2.2/#10212-boolean>
    Boolean(bool),

    /// Represents arbitrary sized finite mathematical integers.
    ///
    /// See <https://yaml.org/spec/1.2.2/#10213-integer>
    Integer(i64),

    /// Represents an approximation to real numbers.
    ///
    /// See <https://yaml.org/spec/1.2.2/#10214-floating-point>
    FloatingPoint(String),
}

impl Default for Node {
    fn default() -> Self {
        Self::Null
    }
}

impl IntoEvents for Node {
    fn into_events(self) -> Vec<Event> {
        let mut events = Vec::new();

        match self {
            Node::Mapping(mapping) => {
                events.push(Event::MappingStart(0));

                for (k, v) in mapping {
                    events.extend(k.into_events());
                    events.extend(v.into_events());
                }

                events.push(Event::MappingEnd);
            }
            Node::Sequence(sequence) => {
                events.push(Event::SequenceStart(0));

                for item in sequence {
                    events.extend(item.into_events());
                }

                events.push(Event::SequenceEnd);
            }
            Node::String(s) => events.push(Event::Scalar(s)),
            Node::Null => events.push(Event::Scalar("null".into())),
            Node::Boolean(b) => events.push(Event::Scalar(b.to_string())),
            Node::Integer(i) => events.push(Event::Scalar(i.to_string())),
            Node::FloatingPoint(_) => todo!(),
        }

        events
    }
}

impl Node {
    pub fn uri(&self) -> String {
        use Node::*;

        match self {
            Mapping(_) => "tag:yaml.org,2002:map",
            Sequence(_) => "tag:yaml.org,2002:seq",
            String(_) => "tag:yaml.org,2002:str",
            Null => "tag:yaml.org,2002:null",
            Boolean(_) => "tag:yaml.org,2002:bool",
            Integer(_) => "tag:yaml.org,2002:int",
            FloatingPoint(_) => "tag:yaml.org,2002:float",
        }
        .into()
    }

    pub fn kind(&self) -> Kind {
        use Node::*;

        match self {
            Mapping(_) => Kind::Mapping,
            Sequence(_) => Kind::Sequence,
            String(_) => Kind::Scalar,
            Null => Kind::Scalar,
            Boolean(_) => Kind::Scalar,
            Integer(_) => Kind::Scalar,
            FloatingPoint(_) => Kind::Scalar,
        }
    }

    pub fn as_name(&self) -> Option<&String> {
        use Node::*;

        match self {
            String(name) => Some(name),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Kind {
    Sequence,
    Mapping,
    Scalar,
}

#[cfg(test)]
mod test {
    use crate::emitter::Emitter;

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
        ]);

        let doc = Document::from_mapping(map);

        let mut stream = Stream::new();
        stream.push_document(doc);

        let mut output = String::new();
        let events = stream.into_events();
        let emitter = Emitter::new(events, 2);
        emitter.emit(&mut output).unwrap();

        // println!("{events:?}");
        println!("{output}")
    }
}
