use redis::AsyncCommands;
use std::sync::Arc;
use tracing::error;
use tracing::instrument;

use super::ports::{Token, TokenRepo};
use async_trait::async_trait;
use uuid::Uuid;

pub struct RedisTokenRepoImpl {
  // TODO: does this need to be Arc?
  pub redis_client: Arc<redis::Client>,
}

#[async_trait]
impl TokenRepo for RedisTokenRepoImpl {
  async fn generate_token(&self) -> Token {
    Uuid::new_v4().to_string()
  }

  #[instrument(skip_all)]
  async fn save_token(&self, token: &Token, username: &String) -> bool {
    let redis_client = &*self.redis_client;

    match redis_client.get_async_connection().await {
      Err(e) => {
        error!("unable to get redis connection. error={:?}", e);
        false
      }
      Ok(mut conn) => {
        let key = format!("token:{}", token);

        conn
          .set(key, username)
          .await
          .map(|_: String| true)
          .unwrap_or(false)
      }
    }
  }

  async fn get_username_by_token(&self, token: &Token) -> Option<String> {
    let redis_client = &*self.redis_client;

    match redis_client.get_async_connection().await {
      Err(e) => {
        error!("unable to get redis connection. error={:?}", e);
        None
      }
      Ok(mut conn) => {
        let key = format!("token:{}", token);

        conn.get(key).await.ok()
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test_log::test(actix_rt::test)]
  async fn test_save_and_get() {
    let redis_client = redis::Client::open("redis://localhost:6378").unwrap();

    let sut = RedisTokenRepoImpl {
      redis_client: Arc::new(redis_client),
    };

    let token = sut.generate_token().await;

    let username = String::from("username");

    assert_eq!(None, sut.get_username_by_token(&token).await);
    assert_eq!(true, sut.save_token(&token, &username).await);
    assert_eq!(Some(username), sut.get_username_by_token(&token).await);
  }
}
