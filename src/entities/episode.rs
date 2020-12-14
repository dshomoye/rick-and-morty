use crate::entity::entity::*;
use serde::Deserialize;

pub mod episode {
  use super::*;

  #[derive(Deserialize, Debug)]
  pub struct Episode {
      id: i64,
      name: String,
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
