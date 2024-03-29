use serde::Serializer;
use std::fmt::{Debug, Formatter};

/// backend for BuildMeta or Prerelease
#[repr(C, align(8))]
#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) struct Identifier {
    vec: Option<Box<str>>,
}

impl Identifier {
    pub const EMPTY: Identifier = Identifier { vec: None };

    /// Creates new Identifier
    ///
    /// SAFETY: the string must be valid ASCII string.
    /// if it contain non-ASCII bytes, it will undefined behaviour
    pub(crate) fn new(string: &str) -> Self {
        Self {
            vec: Some(string.into()),
        }
    }

    /// Returns true if this is empty identifier
    pub(crate) fn is_empty(&self) -> bool {
        self.vec.as_ref().map(|x| x.is_empty()).unwrap_or(true)
    }

    pub fn as_str(&self) -> &str {
        self.vec.as_deref().unwrap_or("")
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.serialize_str(self.as_str())
    }
}
