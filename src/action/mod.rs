use crate::MWAPIBuildable;

/// Marker trait for MediaWiki action module
pub trait MWAPIAction: MWAPIBuildable {}

// Common types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitOrMax {
    Limit(i32),
    Max,
}

impl core::fmt::Display for LimitOrMax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Limit(lmt) => write!(f, "{}", lmt),
            Self::Max => write!(f, "max"),
        }
    }
}

// Submodules
pub mod query;