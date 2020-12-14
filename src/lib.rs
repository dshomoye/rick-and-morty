mod entities;
mod entity;

pub use self::entities::{character::character, episode::episode, location::location};
pub use self::entity::entity::{Error, Object, PageResponse};
