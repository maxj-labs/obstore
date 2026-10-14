use object_store::ObjectStoreExt;
use pyo3::prelude::*;
use pyo3_object_store::{PyLocalStore, PyObjectStore, PyObjectStoreError, PyObjectStoreResult, PyPath};

use crate::list::PyObjectMeta;
use crate::utils::block_on;

#[pyfunction]
pub fn head(py: Python, store: &Bound<PyAny>, path: PyPath) -> PyObjectStoreResult<PyObjectMeta> {
    let local = store.cast::<PyLocalStore>().is_ok();
    let store = store.extract::<PyObjectStore>()?.into_inner();

    py.detach(|| {
        let meta = block_on(local, store.head(path.as_ref()))?;
        Ok::<_, PyObjectStoreError>(PyObjectMeta::new(meta))
    })
}

#[pyfunction]
pub fn head_async(py: Python, store: PyObjectStore, path: PyPath) -> PyResult<Bound<PyAny>> {
    let store = store.into_inner().clone();
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let meta = store
            .head(path.as_ref())
            .await
            .map_err(PyObjectStoreError::ObjectStoreError)?;
        Ok(PyObjectMeta::new(meta))
    })
}
