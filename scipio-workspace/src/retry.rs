use reqwest::{Response, StatusCode};
use reqwest_retry::{default_on_request_failure, Retryable, RetryableStrategy};

/// The default retry strategy for the Google Workspace API client.
pub(crate) struct DefaultRetryStrategy;

impl RetryableStrategy for DefaultRetryStrategy {
    fn handle(&self, res: &Result<Response, reqwest_middleware::Error>) -> Option<Retryable> {
        match res {
            // retry if 412 or 429
            Ok(success)
                if success.status() == StatusCode::PRECONDITION_FAILED
                    || success.status() == StatusCode::TOO_MANY_REQUESTS =>
            {
                println!("Retrying request because of status code: {}", success.status());
                dbg!(success.status());
                log::info!("Retrying request because of status code: {}", success.status());
                Some(Retryable::Transient)
            }
            // otherwise do not retry a successful request
            Ok(_) => None,
            // but maybe retry a request failure
            Err(error) => default_on_request_failure(error),
        }
    }
}
