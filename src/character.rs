pub mod Character {
  use crate::entity::Entity::*;
  use serde::Deserialize;

  #[derive(Deserialize)]
  pub struct Character {
    id: String,
    name: String,
  }
  
  pub async fn get_all() -> Result<Vec<Character>, Error> {
    API::new(EntityTypes::Character).get_all::<Character>().await
  }
  pub async fn get(id: i64) -> Result<Character, Error> {
    API::new(EntityTypes::Character).get::<Character>(id).await
  }
  pub async fn get_page(page: i64) -> Result<MultiPageResponse<Character>, Error> {
    API::new(EntityTypes::Character).get_page::<Character>(page).await
  }

  pub async fn get_multiple(pages: Vec<i64>) -> Result<MultiPageResponse<Character>, Error> {
    API::new(EntityTypes::Character).get_multiple::<Character>(pages).await
  }
}