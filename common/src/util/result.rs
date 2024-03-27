use crate::log_error;

pub fn dissolve_bool<T, E>(result: crate::util::error::Result<T>) -> Result<bool, E> {
    match result {
        Ok(_) => Ok(true),
        Err(e) => {
            log_error!("{e}");
            Ok(false)
        }
    }
}

pub fn dissolve<T, E>(result: crate::util::error::Result<T>) -> Result<(), E> {
    if let Err(e) = result {
        log_error!("{e}");
    }
    Ok(())
}