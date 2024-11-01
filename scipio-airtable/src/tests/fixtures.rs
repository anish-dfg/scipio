use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::{env, thread};
use tracing_test::traced_test;

use anyhow::Result;
use rstest::fixture;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

use crate::Airtable;

type BoxedAsyncFn = Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync>;

pub struct AsyncTestContext {
    pub airtable: Airtable,
    pub cleanup: Arc<Mutex<Vec<BoxedAsyncFn>>>,
}

impl Drop for AsyncTestContext {
    fn drop(&mut self) {
        let cleanup = self.cleanup.clone();
        let handle = thread::spawn(move || {
            Runtime::new().expect("error creating runtime to handle cleanup").block_on(async move {
                let mut error_count = 0;
                for cleanup_fn in cleanup.lock().await.iter() {
                    if let Err(err) = cleanup_fn().await {
                        error_count += 1;
                tracing::error!("Error during cleanup: {:?}", err);
                    }
                }
                if error_count > 0 {
                    panic!("{} cleanup functions failed", error_count);
                }
            })
        });

        handle.join().expect("error joining cleanup thread");
    }
}

#[fixture]
pub fn context() -> AsyncTestContext {
    dotenvy::dotenv().expect("error loading environment variables");
    let api_token =
        env::var("TEST_AIRTABLE_API_TOKEN").expect("missing TEST_AIRTABLE_API_TOKEN variable");
    let client = Airtable::new(&api_token, 5).expect("error creating Airtable client");

    tracing::info!("Creating async test context");

    AsyncTestContext { airtable: client, cleanup: Arc::new(Mutex::new(vec![])) }
}
