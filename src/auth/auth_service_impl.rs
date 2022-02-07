use async_trait::async_trait;

use super::ports::{self, Credentials, Token};

pub struct AuthServiceImpl<CredentialsRepo: ports::CredentialsRepo, TokenRepo: ports::TokenRepo> {
  pub credential_repo: CredentialsRepo,
  pub token_repo: TokenRepo,
}

#[async_trait]
impl<CredentialsRepo, TokenRepo> ports::AuthService for AuthServiceImpl<CredentialsRepo, TokenRepo>
where
  CredentialsRepo: ports::CredentialsRepo + Sync + Send,
  TokenRepo: ports::TokenRepo + Sync + Send,
{
  async fn register(self: &Self, credentials: &Credentials) -> bool {
    self.credential_repo.save_credentials(credentials).await
  }

  async fn login(&self, credentials: &Credentials) -> Option<Token> {
    if !self.credential_repo.credentials_exist(credentials).await {
      return None;
    }

    let token = self.token_repo.generate_token().await;

    if !self
      .token_repo
      .save_token(&token, &credentials.username)
      .await
    {
      return None;
    }

    Some(token)
  }

  async fn authenticate(&self, token: &Token) -> Option<String> {
    self.token_repo.get_username_by_token(token).await
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use mockall::predicate::*;
  use ports::*;

  #[actix_rt::test]
  async fn test_login_success() {
    let credentials = Credentials {
      username: String::from("username"),
      password: String::from("password"),
    };

    let token = String::from("token");

    let mut credential_repo = MockCredentialsRepo::new();

    credential_repo
      .expect_credentials_exist()
      .with(eq(credentials.clone()))
      .return_const(true);

    let mut token_repo = MockTokenRepo::new();

    token_repo
      .expect_generate_token()
      .return_const(token.clone());

    token_repo
      .expect_save_token()
      .with(eq(token.clone()), eq(credentials.username.clone()))
      .return_const(true);

    let sut = AuthServiceImpl {
      credential_repo,
      token_repo,
    };

    let actual = sut.login(&credentials).await;

    assert_eq!(Some(token.clone()), actual);
  }
}
