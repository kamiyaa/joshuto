use std::io;
use std::sync::{Arc, Mutex};

/// An in-memory, shareable log sink. `tracing` writes into it instead of the terminal, so log
/// output doesn't corrupt the TUI; cloning shares the same underlying buffer.
#[derive(Clone, Debug, Default)]
pub struct LogBuffer {
    inner: Arc<Mutex<Vec<u8>>>,
}

impl LogBuffer {
    /// Returns everything logged so far.
    pub fn contents(&self) -> String {
        match self.inner.lock() {
            Ok(buf) => String::from_utf8_lossy(&buf).into_owned(),
            Err(_) => String::new(),
        }
    }
}

impl io::Write for LogBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| io::Error::other("log buffer lock poisoned"))?;
        inner.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::LogBuffer;

    #[test]
    fn tracing_events_are_captured() {
        let log = LogBuffer::default();
        let writer = log.clone();
        let subscriber = tracing_subscriber::fmt()
            .with_writer(move || writer.clone())
            .with_ansi(false)
            .finish();
        tracing::subscriber::with_default(subscriber, || tracing::error!("disk on fire"));
        assert!(log.contents().contains("disk on fire"));
    }
}
