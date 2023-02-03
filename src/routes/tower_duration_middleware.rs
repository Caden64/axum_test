use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use pin_project::pin_project;
use tokio::time::Sleep;
use tower::Service;

#[derive(Debug, Clone)]
struct Timeout<S> {
    inner: S,
    timeout: Duration
}

impl<S> Timeout<S> {
    pub fn new(inner: S, timeout: Duration) -> Self {
        Timeout { inner, timeout }
    }
}

// forward everything to the inner service

impl<S, Request> Service<Request> for Timeout<S>
where
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let res_future = self.inner.call(request);

        let sleep = tokio::time::sleep(self.timeout);

        ResponseFuture {
            res_future,
            sleep
        }
    }
}

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    res_future: F,
    #[pin]
    sleep: Sleep,
}

impl<F, Response, Error> Future for ResponseFuture<F>
where
F: Future<Output = Result<Response, Error>>
{
    type Output = Result<Response, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let res_future: Pin<&mut F> = this.res_future;

        let sleep: Pin<&mut Sleep> = this.sleep;
    }
}