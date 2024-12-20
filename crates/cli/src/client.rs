use anyhow::{bail, Result};
use http::Method;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use std::sync::{Arc, LazyLock};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tokio::task::spawn;
use tokio::time::{sleep, Duration};

pub const CRATES_IO: &str = "https://crates.io";
pub const CRATES_IO_API: &str = "https://crates.io/api/v1/crates";

const USER_AGENT: &str = concat!(
  env!("CARGO_PKG_NAME"),
  "/",
  env!("CARGO_PKG_VERSION"),
  " (",
  env!("CARGO_PKG_REPOSITORY"),
  ")"
);

static HTTP_CLIENT: LazyLock<HttpClient> = LazyLock::new(HttpClient::new);

struct HttpClient {
  client: Client,
  semaphore: Arc<Semaphore>,
}

impl HttpClient {
  pub fn new() -> Self {
    let client = Client::builder()
      .use_rustls_tls()
      .user_agent(USER_AGENT)
      .brotli(true)
      .deflate(true)
      .gzip(true)
      .build()
      .expect("failed to build http client");

    Self {
      client,
      semaphore: Arc::new(Semaphore::new(1)),
    }
  }

  async fn acquire_permit(&self) -> OwnedSemaphorePermit {
    let semaphore = Arc::clone(&self.semaphore);
    semaphore
      .acquire_owned()
      .await
      .expect("semaphore wouldn't be closed")
  }
}

async fn request(method: Method, url: &str) -> Result<Response> {
  let permit = HTTP_CLIENT.acquire_permit().await;
  let (status, response) = HTTP_CLIENT
    .client
    .request(method, url)
    .timeout(Duration::from_secs(10))
    .send()
    .await
    .map(|response| (response.status(), response))?;

  if !status.is_success() {
    let url = response.url().to_owned();
    let message = response.text().await?;
    bail!("REQUEST FAILED ({status})\nurl: {url}\nreason: {message}");
  }

  spawn(async move {
    sleep(Duration::from_secs(1)).await;
    drop(permit);
  });

  Ok(response)
}

pub async fn get<T>(url: &str) -> Result<T>
where
  T: DeserializeOwned,
{
  request(Method::GET, url)
    .await?
    .json::<T>()
    .await
    .map_err(Into::into)
}
