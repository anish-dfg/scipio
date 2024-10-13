use reqwest::{Response, StatusCode};
use reqwest_retry::{default_on_request_failure, Retryable, RetryableStrategy};

pub struct DefaultRetryStrategy;

impl RetryableStrategy for DefaultRetryStrategy {
    fn handle(&self, res: &Result<Response, reqwest_middleware::Error>) -> Option<Retryable> {
        match res {
            Ok(success) if success.status() == StatusCode::TOO_MANY_REQUESTS => {
                Some(Retryable::Transient)
            }

            Ok(_) => None,
            Err(error) => default_on_request_failure(error),
        }
    }
}
