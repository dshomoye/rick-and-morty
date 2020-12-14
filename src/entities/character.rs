use crate::entity::entity::*;
use crate::location::Location;
use serde::Deserialize;

pub mod character {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq)]
    pub struct Character {
        pub id: i64,
        pub name: String,
        pub status: String,
        pub species: String,
        #[serde(rename = "type")]
        pub character_type: String,
        pub origin: Object,
        pub location: Object,
        pub gender: String,
        pub image: String,
        pub episode: Vec<String>,
        pub url: String,
        pub created: String,
    }

    impl Character {
        /// Create a new character object. Should not be required.
        ///
        /// Must use a mutable variable to update the fields.
        pub fn new(id: i64) -> Self {
            Character {
                id: id,
                name: "".to_string(),
                status: "".to_string(),
                species: "".to_string(),
                character_type: "".to_string(),
                gender: "".to_string(),
                image: "".to_string(),
                episode: vec![],
                url: "".to_string(),
                created: "".to_string(),
                location: Object {
                    name: "".to_string(),
                    url: "".to_string(),
                },
                origin: Object {
                    name: "".to_string(),
                    url: "".to_string(),
                },
            }
        }

        pub async fn get_location(&self) -> Result<Option<Location>, Error> {
            if self.location.url.is_empty() {
                Ok(None)
            } else {
                let resp = get_url::<Location>(&self.location.url).await?;
                Ok(Some(resp))
            }
        }
    }

    pub async fn get_all() -> Result<Vec<Character>, Error> {
        API::new(EntityTypes::Character)
            .get_all::<Character>()
            .await
    }

    pub async fn get(id: i64) -> Result<Character, Error> {
        API::new(EntityTypes::Character).get::<Character>(id).await
    }

    pub async fn get_page(page: i64) -> Result<PageResponse<Character>, Error> {
        API::new(EntityTypes::Character)
            .get_page::<Character>(page)
            .await
    }

    pub async fn get_multiple(pages: Vec<i64>) -> Result<PageResponse<Character>, Error> {
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

        let mut expected = character::Character::new(1);
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
