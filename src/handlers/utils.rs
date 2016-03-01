use error::Error;

pub fn error<E: 'static + ::std::error::Error>(err: E) -> Error {
    Error::Handler(Box::new(err))
}

macro_rules! handler_try {
    ($expr:expr) => (try!($expr.map_err($crate::handlers::utils::error)))
}
