mod entities;
mod entity;

pub use self::entities::{ episode::episode, location::location, character::character};
pub use self::entity::entity::{Error, PageResponse, Object};
