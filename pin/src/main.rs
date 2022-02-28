use std::{
    future::Future,
    task::Poll,
    time::{Duration, Instant},
};

#[derive(Default)]
#[pin_project::pin_project]
pub struct TimedWrapper<Fut: Future> {
    start: Option<Instant>,
    #[pin]
    future: Fut,
}

impl<Fut: Future> TimedWrapper<Fut> {
    pub fn new(future: Fut) -> Self {
        Self {
            future,
            start: None,
        }
    }
}

impl<Fut: Future> Future for TimedWrapper<Fut> {
    type Output = (Fut::Output, Duration);

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        let start = this.start.get_or_insert_with(Instant::now);
        let inner_poll = this.future.as_mut().poll(cx);
        let elapsed = start.elapsed();

        match inner_poll {
            // The inner future needs more time, so this future needs more time too
            Poll::Pending => {
                println!("Pending : {:?}", elapsed);
                Poll::Pending
            }
            // Success!
            Poll::Ready(output) => {
                println!("Ready : {:?}", elapsed);
                Poll::Ready((output, elapsed))
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (resp, time) = TimedWrapper::new(reqwest::get("http://rust-lang.com")).await;
    println!(
        "Got a HTTP {} in {}ms",
        resp.unwrap().status(),
        time.as_millis()
    );
}
