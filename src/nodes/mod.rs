mod mapping;

pub use mapping::*;

use crate::events::{Event, IntoEvents};

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
#[derive(Debug)]
pub enum Node {
    /// Represents an associative container, where each key is unique in the
    /// association and mapped to exactly one value.
    ///
    /// See <https://yaml.org/spec/1.2.2/#10111-generic-mapping>
    Mapping(Mapping),

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

                for pair in mapping {
                    events.extend(pair.into_events())
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
