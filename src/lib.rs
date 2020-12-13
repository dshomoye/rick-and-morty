mod entities;
mod entity;

pub use self::entities::{character, episode, location};
pub use self::entity::entity::{PageResponse, Error};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
