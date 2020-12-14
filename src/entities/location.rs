use crate::entity::entity::*;
use serde::Deserialize;

pub mod location {
  use super::*;

  #[derive(Deserialize, Debug)]
  pub struct Location {
      id: i64,
      name: String,
  }

  pub async fn get_all() -> Result<Vec<Location>, Error> {
      API::new(EntityTypes::Location).get_all::<Location>().await
  }

  pub async fn get(id: i64) -> Result<Location, Error> {
      API::new(EntityTypes::Location).get::<Location>(id).await
  }

  pub async fn get_page(page: i64) -> Result<PageResponse<Location>, Error> {
      API::new(EntityTypes::Location)
          .get_page::<Location>(page)
          .await
  }

  pub async fn get_multiple(pages: Vec<i64>) -> Result<PageResponse<Location>, Error> {
      API::new(EntityTypes::Location)
          .get_multiple::<Location>(pages)
          .await
  }
}