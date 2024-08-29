use std::sync::atomic::Ordering;
use pyo3::prelude::*;
use pythonize::depythonize;
use serde_json::{Map, Value};
use common::util::result::{dissolve, dissolve_bool};

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name="dt_core_base_py")]
fn dt_core_python(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(add_event, m)?)?;
    m.add_function(wrap_pyfunction!(flush, m)?)?;
    m.add_function(wrap_pyfunction!(close, m)?)?;
    m.add_function(wrap_pyfunction!(toggle_logger, m)?)?;
    m.add_function(wrap_pyfunction!(set_static_common_properties, m)?)?;
    m.add_function(wrap_pyfunction!(clear_static_common_properties, m)?)?;
    Ok(())
}

#[pyfunction]
fn init(config: MyMap) -> PyResult<bool> {
    Python::with_gil(|py| {
        assert!(py.version_info() >= (3, 7, 0), "Only supports Python version 3.7.0 and up!")
    });

    dissolve_bool(common::init_by_config(config.0))
}

#[pyfunction]
fn add_event(event: MyMap) -> PyResult<bool> {
    dissolve_bool(common::add(event.0))
}

#[pyfunction]
fn flush() -> PyResult<()> {
    dissolve(common::flush())
}

#[pyfunction]
fn close() -> PyResult<()> {
    dissolve(common::close())
}

#[pyfunction]
fn toggle_logger(enable: bool) -> PyResult<()> {
    common::util::logger::LOG_ENABLED.store(enable, Ordering::Relaxed);
    Ok(())
}

#[pyfunction]
fn set_static_common_properties(props: MyMap) -> PyResult<()> {
    dissolve(common::set_static_common_props(props.0))
}

#[pyfunction]
fn clear_static_common_properties() -> PyResult<()> {
    dissolve(common::clear_static_common_props())
}

#[derive(Debug)]
struct MyMap(Map<String, Value>);

impl<'py> FromPyObject<'py> for MyMap {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        match depythonize(ob.downcast().unwrap()) {
            Ok(map) => Ok(MyMap(map)),
            Err(e) => Err(PyErr::from(e))
        }
    }
}
