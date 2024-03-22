use pyo3::prelude::*;
use pythonize::depythonize;
use serde_json::{Map, Value};
use common::consumer::log::LogConsumer;
use common::log_error;

/// A Python module implemented in Rust.
#[pymodule]
fn dt_core_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(add_event, m)?)?;
    m.add_function(wrap_pyfunction!(flush, m)?)?;
    m.add_function(wrap_pyfunction!(close, m)?)?;
    m.add_function(wrap_pyfunction!(toggle_logger, m)?)?;
    Ok(())
}

#[pyfunction]
fn init(config: MyMap) -> PyResult<bool> {
    let mut config = config.0;
    let Some(Value::String(path)) = config.remove("path") else {
        log_error!("Failed to initialize: missing \"path\"!");
        return Ok(false);
    };

    let max_batch_len = config.remove("max_batch_len");
    let Some(Value::Number(max_batch_len)) = max_batch_len else {
        log_error!("Failed to initialize: missing \"max_batch_len\"!");
        return Ok(false);
    };
    let Some(max_batch_len) = max_batch_len.as_u64() else {
        log_error!("Failed to initialize: \"max_batch_len\" should be a positive number!");
        return Ok(false);
    };

    let name_prefix: Option<String> = if let Some(Value::String(name_prefix)) = config.remove("name_prefix") {
        Some(name_prefix)
    } else {
        None
    };
    let max_file_size_bytes = config.remove("max_file_size_bytes");
    let max_file_size_bytes: Option<u64> = if let Some(Value::Number(max_file_size_bytes)) = max_file_size_bytes {
        max_file_size_bytes.as_u64()
    } else {
        None
    };

    let consumer = LogConsumer::new(
        path, max_batch_len as u32, name_prefix, max_file_size_bytes
    );
    if let Err(e) = common::init_consumer(consumer) {
        log_error!("{e}");
        Ok(false)
    } else {
        Ok(true)
    }
}

#[pyfunction]
fn add_event(event: MyMap) -> PyResult<bool> {
    if let Err(e) = common::add(event.0) {
        log_error!("{e}");
        Ok(false)
    } else {
        Ok(true)
    }
}

#[pyfunction]
fn flush() -> PyResult<()> {
    if let Err(e) = common::flush() {
        log_error!("{e}");
    }
    Ok(())
}

#[pyfunction]
fn close() -> PyResult<()> {
    if let Err(e) = common::close() {
        log_error!("{e}");
    }
    Ok(())
}

#[pyfunction]
fn toggle_logger(enable: bool) -> PyResult<()> {
    unsafe {
        common::util::logger::LOG_ENABLED = enable;
    }
    Ok(())
}

#[derive(Debug)]
struct MyMap(Map<String, Value>);

impl<'py> FromPyObject<'py> for MyMap {
    fn extract(ob: &'py PyAny) -> PyResult<Self> {
        match depythonize(ob.downcast().unwrap()) {
            Ok(map) => Ok(MyMap(map)),
            Err(e) => Err(PyErr::from(e))
        }
    }
}
