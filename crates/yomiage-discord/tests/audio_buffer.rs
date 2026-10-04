use std::{
    io::{Cursor, Read, SeekFrom},
    pin::Pin,
    task::{Context, Poll},
};

use songbird::input::{AsyncAdapterStream, AsyncMediaSource};
use tokio::io::{AsyncRead, AsyncSeek, ReadBuf};

struct AudioBytes(Cursor<Vec<u8>>);

impl AsyncRead for AudioBytes {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

impl AsyncSeek for AudioBytes {
    fn start_seek(mut self: Pin<&mut Self>, position: SeekFrom) -> std::io::Result<()> {
        Pin::new(&mut self.0).start_seek(position)
    }

    fn poll_complete(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<u64>> {
        Pin::new(&mut self.0).poll_complete(cx)
    }
}

#[async_trait::async_trait]
impl AsyncMediaSource for AudioBytes {
    fn is_seekable(&self) -> bool {
        true
    }

    async fn byte_len(&self) -> Option<u64> {
        Some(self.0.get_ref().len() as u64)
    }
}

// A small capacity forces backpressure and wraparound during playback.
#[tokio::test(flavor = "multi_thread")]
async fn audio_buffer_preserves_bytes_across_wraparound_and_eof() {
    let expected: Vec<u8> = (0..=255).cycle().take(8192).collect();
    let source = AudioBytes(Cursor::new(expected.clone()));
    let mut stream = AsyncAdapterStream::new(Box::new(source), 32);

    tokio::task::spawn_blocking(move || {
        let mut prefix = vec![0; 1027];
        stream.read_exact(&mut prefix).unwrap();
        assert_eq!(prefix, expected[..1027]);

        let mut remaining = Vec::new();
        stream.read_to_end(&mut remaining).unwrap();
        assert_eq!(remaining, expected[1027..]);
        assert_eq!(stream.read(&mut [0; 1]).unwrap(), 0);
    })
    .await
    .unwrap();
}
