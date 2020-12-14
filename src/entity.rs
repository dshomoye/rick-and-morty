pub mod entity {
    use reqwest;
    pub use reqwest::Error;
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};

    #[cfg(test)]
    use mockito;

    pub async fn get_url<T>(url: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let resp = reqwest::get(url).await?.json::<T>().await?;
        Ok(resp)
    }

    /// `PageResponse` is a helper struct matching the shape of the returned json object when a call returned paginated results.
    /// 
    /// Each object has a helper method `next()` that returns the next page until exhausted.
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct PageResponse<T> {
        results: Vec<T>,
        info: Info,
    }

    impl<T> PageResponse<T> {
        /// `next` gets the next `PageResponse` object, if any.
        /// 
        /// Can be a more efficent way of lazily fetching all entities for an endpoint instead of calling `get_all`.
        pub async fn next(&self) -> Result<Option<PageResponse<T>>, Error>
        where
            T: DeserializeOwned,
        {
            match self.info.next.clone() {
                Some(url) => {
                    let resp = get_url::<PageResponse<T>>(&url).await?;
                    Ok(Some(resp))
                }
                None => Ok(None),
            }
        }
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct Info {
        pages: i64,
        count: i64,
        next: Option<String>,
        prev: Option<String>,
    }

    pub enum EntityTypes {
        Character,
        Episode,
        Location,
    }

    pub struct API {
        entity_type: EntityTypes,
    }

    /// `Object` is a convenient wrapper for maps returned from API for linkable entities. 
    /// Example: a character has a location object with the shape:
    /// ```json
    /// {
    ///     "name": "{name}",
    ///     "url": "{url}"
    /// }
    /// ```
    #[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
    pub struct Object {
        /// `name` is the equivalent name from the api.
        pub name: String,
        /// `url` of the object
        pub url: String,
    }

    impl API {
        pub fn new(e: EntityTypes) -> Self {
            API { entity_type: e }
        }

        fn base_url(&self) -> String {
            #[cfg(not(test))]
            let url = "https://rickandmortyapi.com";

            #[cfg(test)]
            let url = &mockito::server_url();

            match self.entity_type {
                EntityTypes::Character => url.to_owned() + "/api/character",
                EntityTypes::Episode => url.to_owned() + "/api/episode",
                EntityTypes::Location => url.to_owned() + "/api/location",
            }
        }

        /// Get A single Entity by Id
        pub async fn get<T>(&self, id: i64) -> Result<T, Error>
        where
            T: DeserializeOwned,
        {
            let url = self.base_url() + "/" + &id.to_string();
            let resp = get_url::<T>(&url).await?;
            Ok(resp)
        }

        /// Get Vector of all entities by calling all page urls.
        pub async fn get_all<T>(&self) -> Result<Vec<T>, Error>
        where
            T: DeserializeOwned,
        {
            let mut result: Vec<T> = vec![];
            let mut url = self.base_url();
            loop {
                let mut mr = get_url::<PageResponse<T>>(&url).await?;
                result.append(&mut mr.results);
                match mr.info.next {
                    Some(n) => {
                        url = n;
                    }
                    None => break,
                }
            }
            Ok(result)
        }

        /// Get all entities in passed page
        pub async fn get_page<T>(&self, page: i64) -> Result<PageResponse<T>, Error>
        where
            T: DeserializeOwned,
        {
            let url = self.base_url() + "/?page=" + &page.to_string();
            println!("getting url {}", url);
            let resp = get_url::<PageResponse<T>>(&url).await?;
            Ok(resp)
        }

        /// Get multiple entities in vector of provided ids
        pub async fn get_multiple<T>(&self, pages: Vec<i64>) -> Result<PageResponse<T>, Error>
        where
            T: DeserializeOwned,
        {
            let mut page_query = String::from("[");
            for page in pages.iter() {
                page_query = page_query + &page.to_string() + ",";
            }
            page_query = page_query + "]";
            let url = self.base_url() + "/" + &page_query;
            let resp = get_url::<PageResponse<T>>(&url).await?;
            Ok(resp)
        }
    }
}
