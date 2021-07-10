use hyper::client::Client;
use hyper::client::connect::HttpConnector as default;
use hyper_tls::HttpsConnector;
use hyper::Method;
use hyper::Body;
use hyper::body::HttpBody as _;
use hyper::Response;
use serde::Deserialize;
use crate::objects::error::Error;
use crate::objects::user::{User, Users};
use crate::objects::block::{Blocks, Block, AppendBlocks};
use crate::objects::page::{Page, PageTemplate, UpdatePage};
use crate::objects::database::Database;
use crate::misc::Result;

pub struct API<'a> {
    client: Client<HttpsConnector<default>, Body>,
    token: &'a str
}

impl API<'_>  {
    async fn post(&self, path: &str, body: Vec<u8>) -> Result<Response<Body>>{
       let req = hyper::Request::builder()
            .method(Method::POST) 
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Notion-Version", "2021-05-13")
            .uri(format!("https://api.notion.com/v1/{}", path))
            .body(Body::from(body))?;
 
        let resp = self.client.request(req).await?;
        
        Ok(resp)        
    }

    async fn patch(&self, path: &str, body: Vec<u8>) -> Result<Response<Body>> {
        let req = hyper::Request::builder()
            .method(Method::PATCH) 
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Notion-Version", "2021-05-13")
            .uri(format!("https://api.notion.com/v1/{}", path))
            .body(Body::from(body))?;
            
        let resp = self.client.request(req).await?;

        Ok(resp)
    }

    async fn read<T: for<'de> Deserialize<'de>>(&self, mut resp: Response<Body>) -> Result<T> {
        let mut vec: Vec<u8> = Vec::new();

        while let Some(chunk) = resp.body_mut().data().await {
            for byte in chunk?.iter() {
                vec.push(*byte);
            } 
        }
        
        let seq = String::from_utf8(vec)?;
        
        if let Ok(value) = serde_json::from_str::<T>(&seq) {
            return Ok(value);
        }
        
        let err = serde_json::from_str::<Error>(&seq)?;

        Err(err.into())
    }
    
    async fn get(&self, path: &str) -> Result<Response<Body>> {
        let req = hyper::Request::builder()
            .method(Method::GET)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Notion-Version", "2021-05-13")
            .uri(format!("https://api.notion.com/v1/{}", path))
            .body(Body::empty()).unwrap();

        let resp = self.client.request(req).await?;
        
        Ok(resp)
    }

    /*
    pub async fn query_database(&self) -> Result<()> {
        Ok(())
    }
    */

    pub async fn append_block_children(&self, id: &str, blocks: AppendBlocks) -> Result<Block> {
        let path = format!("blocks/{}/children", id);
        let vec = serde_json::to_vec(&blocks)?;
        let resp = self.patch(&path, vec).await?;

        let res = self.read::<Block>(resp).await?;

        Ok(res)
    }

    pub async fn update_page(&self, id: &str, update: UpdatePage) -> Result<Page> {
        let path = format!("pages/{}", id);
        let vec = serde_json::to_vec(&update)?;
        let resp = self.patch(&path, vec).await?;

        let res = self.read::<Page>(resp).await?;

        Ok(res)
    }

    pub async fn create_page(&self, page: PageTemplate) -> Result<Page> {
        let vec = serde_json::to_vec(&page)?;
        let resp = self.post("pages", vec).await?;

        let res = self.read::<Page>(resp).await?;
        
        Ok(res)
    }

    pub async fn get_blocks(&self, id: &str) -> Result<Blocks> {
        let path = format!("blocks/{}/children", id);
        let resp = self.get(&path).await?;

        let res = self.read::<Blocks>(resp).await?;

        Ok(res)
    }

    pub async fn get_user(&self, id: &str) -> Result<User> {
        let path = format!("users/{}", id);
        let resp = self.get(&path).await?;
        
        let res = self.read::<User>(resp).await?;

        Ok(res)
    }

    pub async fn get_users(&self) -> Result<Users> {
        let resp = self.get("users").await?;

        let res = self.read::<Users>(resp).await?;

        Ok(res)
    }

    pub async fn get_page(&self, id: &str) -> Result<Page> {
        let path = format!("pages/{}", id);
        let resp = self.get(&path).await?;
        
        let res = self.read::<Page>(resp).await?;

        Ok(res)
    }

    pub async fn get_database(&self, id: &str) -> Result<Database>{
        let path = format!("databases/{}", id);
        let resp = self.get(&path).await?;

        let res = self.read::<Database>(resp).await?;

        Ok(res)
    }

    pub fn new(token: &str) -> API {
        API {
            client: hyper::Client::builder()
                .build(HttpsConnector::new()),
            token: token
        }
    }
}
