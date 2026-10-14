use pyo3::prelude::*;

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

/// Run a store future to completion on the calling thread.
///
/// A native `LocalStore` needs no tokio context: without one, object_store's
/// `maybe_spawn_blocking` runs its blocking I/O inline instead of paying two
/// thread hops per call, which dominates small local reads.
pub(crate) fn block_on<F: std::future::Future>(local: bool, fut: F) -> F::Output {
    if local {
        futures::executor::block_on(fut)
    } else {
        pyo3_async_runtimes::tokio::get_runtime().block_on(fut)
    }
}
