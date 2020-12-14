use crate::entity::entity::*;
use serde::{Deserialize, Serialize};

/// `episode` contains Struct and helper methods for episodes in the rick and morty api.
pub mod episode {
    use super::*;

    /// `Episode` struct closely matches episode json object fields.
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct Episode {
        /// episode id
        pub id: i64,

        /// episode name
        pub name: String,

        /// episode air date. e.g `"December 2, 2013"`
        pub air_date: String,

        /// episode code: e.g `"S01E01"`
        pub episode: String,

        /// slice of character urls
        pub characters: Vec<String>,

        /// location url
        pub url: String,

        /// created date
        pub created: String,
    }

    /// get all episodes from the api.
    pub async fn get_all() -> Result<Vec<Episode>, Error> {
        API::new(EntityTypes::Episode).get_all::<Episode>().await
    }

    /// get a single episode by its id.
    pub async fn get(id: i64) -> Result<Episode, Error> {
        API::new(EntityTypes::Episode).get::<Episode>(id).await
    }

    /// get all episodes in page. 
    /// 
    /// Example, calling `get_page(1)` calls `"https://rickandmortyapi.com/api/location?page=1"`
    pub async fn get_page(page: i64) -> Result<PageResponse<Episode>, Error> {
        API::new(EntityTypes::Episode)
            .get_page::<Episode>(page)
            .await
    }

    /// get multiple episodes with id matching id in `ids`
    /// 
    /// Example call get_multiple([2,3,4]) calls `"https://rickandmortyapi.com/api/episode/[2,3,4]"`
    pub async fn get_multiple(ids: Vec<i64>) -> Result<PageResponse<Episode>, Error> {
        API::new(EntityTypes::Episode)
            .get_multiple::<Episode>(ids)
            .await
    }
}
