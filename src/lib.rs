mod dir;

pub use dir::{create_dir, create_dir_all, remove_dir};

use futures::Future;
use std::io::{self, ErrorKind};

fn blocking<F, I, E>(f: F) -> impl Future<Item = I, Error = io::Error>
where
    F: FnOnce() -> Result<I, E> + Send + 'static,
    I: Send + 'static,
    E: Send + std::fmt::Debug + 'static,
{
    actix_threadpool::run(f).map_err(|_| blocking_err())
}

fn blocking_err() -> io::Error {
    io::Error::new(
        ErrorKind::Other,
        "`blocking` annotated I/O must be called \
         from the context of the Actix runtime.",
    )
}
