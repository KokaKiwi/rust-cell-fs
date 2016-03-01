use std::io;
use std::path::PathBuf;

pub type Result<T> = ::std::result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// I/O error.
        Io(err: io::Error) {
            cause(err)
            description("I/O error")
            display("I/O error: {}", err)
            from()
        }
        /// Handler error, usually a lib-specific boxed error.
        Handler(err: Box<::std::error::Error>) {
            description("Handler error")
            display("Handler error: {}", err)
        }
        /// Entity not found
        NotFound(path: PathBuf) {
            description("Entity not found")
            display("Entity not found at `{}`", path.display())
        }
    }
}
