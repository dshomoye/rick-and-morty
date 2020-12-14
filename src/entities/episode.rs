use crate::entity::entity::*;
use serde::Deserialize;

pub mod episode {
    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct Episode {
        pub id: i64,
        pub name: String,
        pub air_date: String,
        pub episode: String,
        pub characters: Vec<String>,
        pub url: String,
        pub created: String,
    }

    pub async fn get_all() -> Result<Vec<Episode>, Error> {
        API::new(EntityTypes::Episode).get_all::<Episode>().await
    }

    pub async fn get(id: i64) -> Result<Episode, Error> {
        API::new(EntityTypes::Episode).get::<Episode>(id).await
    }

    pub async fn get_page(page: i64) -> Result<PageResponse<Episode>, Error> {
        API::new(EntityTypes::Episode)
            .get_page::<Episode>(page)
            .await
    }

    pub async fn get_multiple(pages: Vec<i64>) -> Result<PageResponse<Episode>, Error> {
        API::new(EntityTypes::Episode)
            .get_multiple::<Episode>(pages)
            .await
    }
}
