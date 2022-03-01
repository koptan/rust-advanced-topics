use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;

use futures::FutureExt;
use tokio::fs::File;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::time::Instant;
use tokio::time::Sleep;

struct SlowRead<R> {
    reader: R,
    sleep: Pin<Box<Sleep>>,
}

impl<R> SlowRead<R> {
    fn new(reader: R) -> Self {
        Self {
            reader: reader,
            sleep: Box::pin(tokio::time::sleep(Default::default())),
        }
    }
}

impl<R> AsyncRead for SlowRead<R>
where
    R: AsyncRead + Unpin,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.sleep.poll_unpin(cx) {
            Poll::Ready(_) => {
                self.sleep
                    .as_mut()
                    .reset(Instant::now() + Duration::from_millis(25));
                Pin::new(&mut self.reader).poll_read(cx, buf)
            }
            Poll::Pending => Poll::Pending,
        }

        //self.reader.as_mut().poll_read(cx, buf)
    }
}

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    let mut buf = vec![0u8; 128 * 1024];
    let mut f = File::open("/dev/urandom").await?;
    let before = Instant::now();
    f.read_exact(&mut buf).await?;
    println!("Read {} bytes in {:?}", buf.len(), before.elapsed());

    let mut buf = vec![0u8; 128 * 1024];
    let mut f = SlowRead::new(File::open("/dev/urandom").await?);
    let before = Instant::now();
    f.read_exact(&mut buf).await?;
    println!("Read {} bytes in {:?}", buf.len(), before.elapsed());

    Ok(())
}
