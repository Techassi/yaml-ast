//! This module contains types for YAML mappings. A [`Mapping`] consists of
//! multiple entries. Each entry is called [`MappingPair`]. Such a pair consists
//! of a unique key and a value. Both key and value can be any valid [`Node`].
//! The AST additionally wraps the key node in [`MappingKey`] and the value node
//! in [`MappingValue`].
//!
//! ### AST Structure
//!
//! ```plain
//! Node (
//!   Mapping [
//!     Mapping Pair (
//!       Mapping Key (Node)
//!       Mapping Value (Node)
//!     )
//!     ...
//!   ]
//! )
//! ```

use std::ops::Deref;

use crate::{
    events::{Event, IntoEvents},
    nodes::Node,
};

// TODO (Techassi): Ensure keys are unique in mappings
/// A mapping is a list of mapping key/value pairs.
///
/// ### AST Structure
///
/// ```plain
/// Mapping [
///   Mapping Pair (
///     Mapping Key (Node)
///     Mapping Value (Node)
///   )
///   ...
/// ]
/// ```
#[derive(Debug)]
pub struct Mapping(Vec<MappingPair>);

impl Deref for Mapping {
    type Target = Vec<MappingPair>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for Mapping {
    type IntoIter = std::vec::IntoIter<MappingPair>;
    type Item = MappingPair;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const N: usize> From<[MappingPair; N]> for Mapping {
    fn from(pairs: [MappingPair; N]) -> Self {
        Self(Vec::from(pairs))
    }
}

impl<const N: usize> From<[(MappingKey, MappingValue); N]> for Mapping {
    fn from(pairs: [(MappingKey, MappingValue); N]) -> Self {
        let mut mappings = Vec::new();

        for pair in pairs {
            mappings.push(MappingPair::from(pair))
        }

        Self(mappings)
    }
}

impl<const N: usize> From<[(Node, Node); N]> for Mapping {
    fn from(pairs: [(Node, Node); N]) -> Self {
        let mut mappings = Vec::new();

        for pair in pairs {
            mappings.push(MappingPair::from(pair))
        }

        Self(mappings)
    }
}

/// A mapping key/value pair. The AST structure looks like this:
///
/// ```plain
/// Mapping Pair (
///   Mapping Key (Node)
///   Mapping Value (Node)
/// )
/// ```
#[derive(Debug)]
pub struct MappingPair((MappingKey, MappingValue));

impl IntoEvents for MappingPair {
    fn into_events(self) -> Vec<Event> {
        todo!()
    }
}

impl From<(MappingKey, MappingValue)> for MappingPair {
    fn from(pair: (MappingKey, MappingValue)) -> Self {
        Self(pair)
    }
}

impl From<(Node, Node)> for MappingPair {
    fn from(pair: (Node, Node)) -> Self {
        Self((MappingKey(pair.0), MappingValue(pair.1)))
    }
}

#[derive(Debug)]
pub struct MappingKey(Node);

#[derive(Debug)]
pub struct MappingValue(Node);
