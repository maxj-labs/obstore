use std::sync::Arc;

use object_store::ObjectStore;
use pyo3::prelude::*;
use pyo3_object_store::{PyLocalStore, PyObjectStore};

/// Returning `()` from `future_into_py` returns an empty tuple instead of None
/// https://github.com/developmentseed/obstore/issues/240
pub(crate) struct PyNone;

impl<'py> IntoPyObject<'py> for PyNone {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(py.None().bind(py).clone())
    }
}

/// A store for the sync API, extracted together with whether it is a native `LocalStore`.
///
/// A native `LocalStore` needs no tokio context: without one, object_store's
/// `maybe_spawn_blocking` runs its blocking I/O inline instead of paying two
/// thread hops per call, which dominates small local reads.
pub(crate) struct SyncStore {
    store: PyObjectStore,
    local: bool,
}

impl<'py> FromPyObject<'_, 'py> for SyncStore {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        Ok(Self {
            local: obj.cast::<PyLocalStore>().is_ok(),
            store: obj.extract()?,
        })
    }
}

impl SyncStore {
    /// Run a store future to completion on the calling thread.
    pub(crate) fn block_on<F: std::future::Future>(&self, fut: F) -> F::Output {
        if self.local {
            futures::executor::block_on(fut)
        } else {
            pyo3_async_runtimes::tokio::get_runtime().block_on(fut)
        }
    }
}

impl AsRef<Arc<dyn ObjectStore>> for SyncStore {
    fn as_ref(&self) -> &Arc<dyn ObjectStore> {
        self.store.as_ref()
    }
}
