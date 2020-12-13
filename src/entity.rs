pub mod Entity {
    use reqwest;
    pub use reqwest::Error;
    use serde::de::DeserializeOwned;
    use serde::Deserialize;

    const RMAPIROOT: &str = "https://rickandmortyapi.com/api";

    #[derive(Deserialize)]
    pub struct MultiPageResponse<T> {
        results: Vec<T>,
        info: Info,
    }

    #[derive(Deserialize)]
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

    impl API {
        pub fn new(e: EntityTypes) -> Self {
            API { entity_type: e }
        }

        fn base_url(&self) -> String {
            match self.entity_type {
                EntityTypes::Character => RMAPIROOT.to_owned() + "/character/",
                EntityTypes::Episode => RMAPIROOT.to_owned() + "/episode/",
                EntityTypes::Location => RMAPIROOT.to_owned() + "/location/",
            }
        }

        /// Get A single Entity by Id
        pub async fn get<T>(&self, id: i64) -> Result<T, Error>
        where
            T: DeserializeOwned,
        {
            let url = self.base_url() + &id.to_string();
            let resp = reqwest::get(&url).await?.json::<T>().await?;
            Ok(resp)
        }

        /// Get Vectory of all entities by calling all page urls.
        pub async fn get_all<T>(&self) -> Result<Vec<T>, Error>
        where
            T: DeserializeOwned,
        {
            let mut resArr: Vec<T> = vec![];
            let mut url = self.base_url();
            loop {
                let mut mr = self.get_url::<MultiPageResponse<T>>(&url).await?;
                resArr.append(&mut mr.results);
                match mr.info.next {
                    Some(n) => {
                        url = n;
                    }
                    None => break,
                }
            }
            Ok(resArr)
        }

        /// Get all entities in passed page
        pub async fn get_page<T>(&self, page: i64) -> Result<MultiPageResponse<T>, Error>
        where
            T: DeserializeOwned,
        {
            let url = self.base_url() + "/?page=" + &page.to_string();
            let resp = reqwest::get(&url)
                .await?
                .json::<MultiPageResponse<T>>()
                .await?;
            Ok(resp)
        }

        /// Get multiple entities in vector of provided ids
        pub async fn get_multiple<T>(&self, pages: Vec<i64>) -> Result<MultiPageResponse<T>, Error>
        where
            T: DeserializeOwned,
        {
            let mut page_query = String::from("[");
            for page in pages.iter() {
                page_query = page_query + &page.to_string() + ",";
            }
            page_query = page_query + "]";
            let url = self.base_url() + &page_query;
            let resp = reqwest::get(&url)
                .await?
                .json::<MultiPageResponse<T>>()
                .await?;
            Ok(resp)
        }

        async fn get_url<T>(&self, url: &str) -> Result<T, Error>
        where
            T: DeserializeOwned,
        {
            let resp = reqwest::get(url).await?.json::<T>().await?;
            Ok(resp)
        }
    }
}
