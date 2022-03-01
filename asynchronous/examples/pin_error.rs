use std::{mem::swap, pin::Pin, task::Poll, time::Duration};

use futures::{future::poll_fn, Future};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut sleep1 = sleep(Duration::from_secs(1));
    let mut sleep2 = sleep(Duration::from_secs(1));

    {
        let mut sleep1 = unsafe { Pin::new_unchecked(&mut sleep1) };

        poll_fn(|cx| {
            let _ = sleep1.as_mut().poll(cx);
            Poll::Ready(())
        })
        .await;
    }

    swap(&mut sleep1, &mut sleep2);

    sleep1.await;
    sleep2.await;
}
