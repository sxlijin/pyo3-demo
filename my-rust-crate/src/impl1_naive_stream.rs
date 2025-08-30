//! Demonstrate different approaches to attempting to build a Python async
//! iterator in Rust without cloning stuff.
//!
//! Couldn't make any of them work: wrapping a `Stream`, wrapping a `Receiver`,
//! or (not shown) using `StreamExt::into_future`.

use crate::get_http_stream;
use futures::pin_mut;
use pyo3::exceptions::PyStopAsyncIteration;
use pyo3::prelude::*;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_stream::StreamExt;

#[pyclass]
pub struct NaiveStream {
    stream: Pin<Box<dyn futures::Stream<Item = String> + Send + Sync>>,
}

#[pymethods]
impl NaiveStream {
    #[new]
    fn new() -> Self {
        Self {
            stream: Box::pin(get_http_stream("https://sse.dev/test")),
        }
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        unimplemented!()
        // pyo3_async_runtimes::tokio::future_into_py(py, async move {
        //     let event = self.stream.next().await;

        //     match event {
        //         Some(s) => Ok(s),
        //         None => Err(PyStopAsyncIteration::new_err("Stream ended")),
        //     }
        // })
    }
}

#[pyclass]
pub struct NaiveStreamWithArcMutex {
    stream: Arc<Mutex<Pin<Box<dyn futures::Stream<Item = String> + Send + Sync>>>>,
}

#[pymethods]
impl NaiveStreamWithArcMutex {
    #[new]
    fn new() -> Self {
        Self {
            stream: Arc::new(Mutex::new(Box::pin(get_http_stream(
                "https://sse.dev/test",
            )))),
        }
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let stream_ref = self.stream.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let event = stream_ref.lock().await.next().await;

            match event {
                Some(s) => Ok(s),
                None => Err(PyStopAsyncIteration::new_err("Stream ended")),
            }
        })
    }
}

#[pyclass]
pub struct MpscChannelStream {
    rx: tokio::sync::mpsc::UnboundedReceiver<String>,
}

#[pymethods]
impl MpscChannelStream {
    #[new]
    fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        tokio::spawn(async move {
            let stream = get_http_stream("https://sse.dev/test");
            pin_mut!(stream);
            while let Some(s) = stream.next().await {
                tx.send(s).unwrap();
            }
        });

        Self { rx }
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        unimplemented!()
        // pyo3_async_runtimes::tokio::future_into_py(py, async move {
        //     let event = self.rx.recv().await;

        //     match event {
        //         Some(s) => Ok(s),
        //         None => Err(PyStopAsyncIteration::new_err("Stream ended")),
        //     }
        // })
    }
}

#[pyclass]
pub struct BroadcastChannelStream {
    rx: tokio::sync::broadcast::Receiver<String>,
}

#[pymethods]
impl BroadcastChannelStream {
    #[new]
    fn new() -> Self {
        let (tx, rx) = tokio::sync::broadcast::channel(100);

        tokio::spawn(async move {
            let stream = get_http_stream("https://sse.dev/test");
            pin_mut!(stream);
            while let Some(s) = stream.next().await {
                tx.send(s).unwrap();
            }
        });

        Self { rx }
    }

    fn __aiter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __anext__<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let mut rx = self.rx.resubscribe();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let event = rx.recv().await;

            match event {
                Ok(s) => Ok(s),
                Err(_) => Err(PyStopAsyncIteration::new_err("Stream ended")),
            }
        })
    }
}
