pub mod Entity {
  pub use reqwest::Error;

  pub struct MultiPageResponse<T> {
    results: Vec<T>,
    page: i64,
    total_pages: i64,
    count: i64
  }

  pub enum EntityTypes {
    Character,
    Episode,
    Location,
  }
  
  pub struct API {
    entityType: EntityTypes
  }

  
  impl API {
    pub fn new(e: EntityTypes) -> Self {
      match e {
        Character => API {entityType: EntityTypes::Character},
        Location => API {entityType: EntityTypes::Location},
        Episode => API {entityType: EntityTypes::Episode},
      }
    }

    pub async fn get<T>(&self, id: i64) -> Result<T, Error> {
  
    }
    pub async fn get_all<T>(&self) -> Result<Vec<T>, Error> {
  
    }
    pub async fn get_page<T>(&self, page: i64) -> Result<MultiPageResponse<T>, Error> {
  
    }
    pub async fn get_multiple<T>(&self, pages: Vec<i64>) -> Result<MultiPageResponse<T>, Error> {
  
    }
  }

}

