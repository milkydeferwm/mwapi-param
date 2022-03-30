use crate::MWAPIBuildable;

pub trait MWAPIFormat: MWAPIBuildable {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MWAPIFormatVersion {
    Version1,
    Version2,
    VersionLatest,
}

impl core::fmt::Display for MWAPIFormatVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Version1 => write!(f, "1"),
            Self::Version2 => write!(f, "2"),
            Self::VersionLatest => write!(f, "latest"),
        }
    }
}

pub mod json;
pub mod none;
pub mod php;
pub mod rawfm;
pub mod xml;