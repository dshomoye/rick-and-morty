mod entities;
mod entity;

pub use self::entities::{character, location, episode};
pub use self::entity::Entity::{MultiPageResponse}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
