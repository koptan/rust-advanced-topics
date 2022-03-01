use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use futures::Future;

struct MyFuture {
    slept: bool,
}

impl MyFuture {
    fn new() -> Self {
        Self { slept: false }
    }
}

impl Future for MyFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("MyFuture::Poll()");
        println!("Slept value : {}", &self.slept);

        match self.slept {
            false => {
                let waker = cx.waker().clone();
                std::thread::spawn(move || {
                    println!("Inside thread");
                    std::thread::sleep(Duration::from_secs(10));
                    waker.wake();
                });
                println!("After thread");
                self.slept = true;
                Poll::Pending
            }
            true => Poll::Ready(()),
        }
    }
}

#[tokio::main]
async fn main() {
    let fut = MyFuture::new();
    println!("Awaiting fut...");
    fut.await;
    println!("Awaiting fut... done!");
}
