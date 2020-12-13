pub mod character {
    use crate::entity::Entity::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Character {
        id: String,
        name: String,
    }

    pub async fn get_all() -> Result<Vec<Character>, Error> {
        API::new(EntityTypes::Character)
            .get_all::<Character>()
            .await
    }

    pub async fn get(id: i64) -> Result<Character, Error> {
        API::new(EntityTypes::Character).get::<Character>(id).await
    }

    pub async fn get_page(page: i64) -> Result<MultiPageResponse<Character>, Error> {
        API::new(EntityTypes::Character)
            .get_page::<Character>(page)
            .await
    }

    pub async fn get_multiple(pages: Vec<i64>) -> Result<MultiPageResponse<Character>, Error> {
        API::new(EntityTypes::Character)
            .get_multiple::<Character>(pages)
            .await
    }
}

pub mod location {
    use crate::entity::Entity::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Location {
        id: String,
        name: String,
    }

    pub async fn get_all() -> Result<Vec<Location>, Error> {
        API::new(EntityTypes::Location).get_all::<Location>().await
    }

    pub async fn get(id: i64) -> Result<Location, Error> {
        API::new(EntityTypes::Location).get::<Location>(id).await
    }

    pub async fn get_page(page: i64) -> Result<MultiPageResponse<Location>, Error> {
        API::new(EntityTypes::Location)
            .get_page::<Location>(page)
            .await
    }

    pub async fn get_multiple(pages: Vec<i64>) -> Result<MultiPageResponse<Location>, Error> {
        API::new(EntityTypes::Location)
            .get_multiple::<Location>(pages)
            .await
    }
}

pub mod episode {
    use crate::entity::Entity::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Episode {
        id: String,
        name: String,
    }

    pub async fn get_all() -> Result<Vec<Episode>, Error> {
        API::new(EntityTypes::Episode).get_all::<Episode>().await
    }

    pub async fn get(id: i64) -> Result<Episode, Error> {
        API::new(EntityTypes::Episode).get::<Episode>(id).await
    }

    pub async fn get_page(page: i64) -> Result<MultiPageResponse<Episode>, Error> {
        API::new(EntityTypes::Episode)
            .get_page::<Episode>(page)
            .await
    }

    pub async fn get_multiple(pages: Vec<i64>) -> Result<MultiPageResponse<Episode>, Error> {
        API::new(EntityTypes::Episode)
            .get_multiple::<Episode>(pages)
            .await
    }
}
