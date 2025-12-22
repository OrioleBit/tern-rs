use std::hash::Hash;
use uuid::Uuid;

pub trait Entity: Sized + PartialEq + Eq + Hash {
    fn id(&self) -> Uuid;
    // fn new() -> Self;
    // fn from_existing(id: Uuid) -> Self;
}
