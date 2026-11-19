use crate::domain::{LineId, StationId, TrackId};

pub trait Id<'a>:
    std::fmt::Debug
    + PartialEq
    + Eq
    + std::hash::Hash
    + Clone
    + serde::Serialize
    + serde::Deserialize<'a>
    + From<String>
    + From<&'a str>
{
    fn new(id: &str) -> Self;
}
