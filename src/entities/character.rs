use crate::entity::entity::*;
use crate::location::Location;
use crate::episode::Episode;
use serde::{Deserialize, Serialize};

/// `character` mod provides struct and functions for querying characters
pub mod character {
    use super::*;

    /// `Character` closely matches character returned from [character endpoint](https://rickandmortyapi.com/documentation/#character-schema). 
    #[derive(Deserialize, Debug, PartialEq, Clone, Serialize, Default)]
    pub struct Character {
        /// character id
        pub id: i64,

        /// character name
        pub name: String,

        /// character status: e.g `"Alive"`
        pub status: String,

        /// character species
        pub species: String,
        #[serde(rename = "type")]

        /// `character.character_type` is equivalent to `character.type` in the json object of the api.
        pub character_type: String,

        /// character `origin` has `origin.name` and `origin.url`.
        pub origin: Object,

        /// character `location` has `location.name` and `location.url`.
        pub location: Object,

        /// gender
        pub gender: String,

        /// image url
        pub image: String,

        /// slices of episode urls
        pub episode: Vec<String>,
        
        /// url
        pub url: String,

        /// created date string
        pub created: String,
    }

    impl Character {

        /// Gets the `Location` associated with the `Character` object.
        pub async fn location(&self) -> Result<Option<Location>, Error> {
            if self.location.url.is_empty() {
                Ok(None)
            } else {
                let resp = get_url::<Location>(&self.location.url).await?;
                Ok(Some(resp))
            }
        }

        /// Gets the `Location` object for the origin of the character.
        pub async fn origin(&self) -> Result<Option<Location>, Error> {
            if self.origin.url.is_empty() {
                Ok(None)
            } else {
                let resp = get_url::<Location>(&self.origin.url).await?;
                Ok(Some(resp))
            }
        }

        /// Gets the episodes the character appears in. 
        /// 
        /// This makes multiple async calls, might take longer time to resolve. 
        pub async fn episodes(&self) -> Result<Vec<Episode>, Error> {
            if self.episode.len() < 1 {
                Ok(vec![])
            } else {
                let mut episodes = vec![];
                for e_url in self.episode.iter() {
                    let resp = get_url::<Episode>(e_url).await?;
                    episodes.push(resp);
                }
                Ok(episodes)
            }
        }
    }

    /// Returns slice of all characters from the API.
    /// 
    /// This makes synchronous calls so the entire call may take seconds to complete.
    pub async fn get_all() -> Result<Vec<Character>, Error> {
        API::new(EntityTypes::Character)
            .get_all::<Character>()
            .await
    }

    /// Get a single `Character` by its `id`
    pub async fn get(id: i64) -> Result<Character, Error> {
        API::new(EntityTypes::Character).get::<Character>(id).await
    }

    /// Get characters in `page`
    /// 
    /// Example, calling `get_page(1)` calls `"https://rickandmortyapi.com/api/character?page=1"`
    pub async fn get_page(page: i64) -> Result<PageResponse<Character>, Error> {
        API::new(EntityTypes::Character)
            .get_page::<Character>(page)
            .await
    }

    /// Get multiple characters by slice of `id`s.
    /// 
    /// Example call `get_multiple([2,3,4])` calls `"https://rickandmortyapi.com/api/character/[2,3,4]"`

    pub async fn get_multiple(pages: Vec<i64>) -> Result<Vec<Character>, Error> {
        API::new(EntityTypes::Character)
            .get_multiple::<Character>(pages)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[tokio::test]
    async fn it_gets_a_character() {
        let data = "{ \"id\": 1, \"name\": \"John Doe\", \"status\": \"Alive\", \"species\": \"Human\", \"type\": \"\", \"gender\": \"Male\", \"origin\": { \"name\": \"\", \"url\": \"\" }, \"location\": { \"name\": \"\", \"url\": \"\" }, \"image\": \"mock.jpeg\", \"episode\": [], \"url\": \"mock.mock\", \"created\": \"mock\" }";

        let _m = mock("GET", "/api/character/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(data)
            .create();

        let mut expected = character::Character::default();
        expected.id = 1;
        expected.name = "John Doe".to_string();
        expected.status = "Alive".to_string();
        expected.character_type = "".to_string();
        expected.gender = "Male".to_string();
        expected.image = "mock.jpeg".to_string();
        expected.url = "mock.mock".to_string();
        expected.created = "mock".to_string();
        expected.species = "Human".to_string();

        let req = character::get(1).await;
        match req {
            Ok(c) => {
                assert_eq!(c, expected)
            }
            Err(e) => {
                println!("request error: {:?}", e);
                panic!("request failed");
            }
        }
    }
}
