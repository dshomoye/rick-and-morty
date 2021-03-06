use crate::entity::entity::*;
use crate::character::Character;
use serde::{Deserialize, Serialize};
use futures::future::try_join_all;

/// `location` module contains struct and functions for managing locations in the Rick And Morty Universe.
pub mod location {
    use super::*;

    /// The `Location` struct closely matches [location object](https://rickandmortyapi.com/documentation/#location-schema) from API.
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
    pub struct Location {
        ///  location id
        pub id: i64,
        /// location name
        pub name: String,

        /// location type. this is a rename from `type` in the json object.
        #[serde(rename = "type")]
        pub location_type: String,

        /// location dimension
        pub dimension: String,

        /// location residents. Array of urls to characters.
        pub residents: Vec<String>,

        /// location url.
        pub url: String,

        /// string of created
        pub created: String,
    }

    impl Location {
        /// Returns `Characters` that are residents of the location.
        pub async fn residents(&self) -> Result<Vec<Character>, Error> {
            let mut work = vec![];
            for res_url in self.residents.iter() {
                work.push(get_url::<Character>(res_url));
            }
            let results = try_join_all(work).await?;
            Ok(results)
        }
    }

    /// `get_all` locations
    pub async fn get_all() -> Result<Vec<Location>, Error> {
        API::new(EntityTypes::Location).get_all::<Location>().await
    }

    /// `get` a single location by id
    pub async fn get(id: i64) -> Result<Location, Error> {
        API::new(EntityTypes::Location).get::<Location>(id).await
    }

    /// get locations in page `page`. 
    /// 
    /// Example, calling `get_page(1)` calls `"https://rickandmortyapi.com/api/location?page=1"`
    pub async fn get_page(page: i64) -> Result<PageResponse<Location>, Error> {
        API::new(EntityTypes::Location)
            .get_page::<Location>(page)
            .await
    }

    /// get all locations with id in slice `ids`
    /// 
    /// Example call `get_multiple([2,3,4])` calls `"https://rickandmortyapi.com/api/location/[2,3,4]"`
    pub async fn get_multiple(ids: &Vec<i64>) -> Result<Vec<Location>, Error> {
        API::new(EntityTypes::Location)
            .get_multiple::<Location>(ids)
            .await
    }
}
