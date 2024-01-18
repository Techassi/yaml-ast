use crate::events::{Event, IntoEvents};

mod mapping;
mod sequence;

pub use mapping::*;
pub use sequence::*;

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
    Sequence(Sequence),

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

    /// Represents a comment.
    ///
    /// Providing this node differs from the official YAML spec. The spec
    /// states that comments have no effect on the serialization tree or
    /// representation graph (which this is). Per spec, they are a
    /// representation detail. To enable full control over the AST, we include
    /// it here. Access to the comments and their content are a valid use-case
    /// for some applications.
    Comment(Comment),
}

impl Default for Node {
    fn default() -> Self {
        Self::Null
    }
}

impl IntoEvents for Node {
    fn into_events(&self, events: &mut Vec<Event>) {
        match self {
            Node::Mapping(m) => m.into_events(events),
            Node::Sequence(s) => s.into_events(events),
            Node::String(_) => todo!(),
            Node::Null => todo!(),
            Node::Boolean(_) => todo!(),
            Node::Integer(_) => todo!(),
            Node::FloatingPoint(_) => todo!(),
            Node::Comment(c) => c.into_events(events),
        }
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
            Comment(_) => "",
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
            Comment(_) => Kind::Scalar,
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

#[derive(Debug)]
pub struct Comment {
    pub kind: CommentKind,
    pub content: String,
}

impl IntoEvents for Comment {
    fn into_events(&self, events: &mut Vec<Event>) {
        events.push(Event::Comment(self.content.clone()))
    }
}

/// Represents the kind of comment.
///
/// Inline comments are interleaved with other
#[derive(Debug)]
pub enum CommentKind {
    Inline,
    Block,
}
