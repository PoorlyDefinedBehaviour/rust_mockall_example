use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Credentials {
  pub username: String,
  pub password: String,
}

pub type Token = String;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AuthService {
  /// Registers a new user.
  async fn register(&self, credentials: &Credentials) -> bool;
  /// User can use its credentials to get a token.
  async fn login(&self, credentials: &Credentials) -> Option<Token>;
  async fn authenticate(&self, token: &Token) -> Option<String>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CredentialsRepo {
  async fn save_credentials(&self, credentials: &Credentials) -> bool;
  async fn credentials_exist(&self, credentials: &Credentials) -> bool;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TokenRepo {
  async fn generate_token(&self) -> Token;
  async fn save_token(&self, token: &Token, username: &String) -> bool;
  async fn get_username_by_token(&self, token: &Token) -> Option<String>;
}
