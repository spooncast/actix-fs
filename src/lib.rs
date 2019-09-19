mod dir;
mod file;

pub use dir::{create_dir, create_dir_all, remove_dir};
pub use file::{remove_file, rename, File, OpenOptions};

use futures::Future;
use std::io::{self, ErrorKind};

fn blocking<F, I>(f: F) -> impl Future<Item = I, Error = io::Error>
where
    F: FnOnce() -> Result<I, io::Error> + Send + 'static,
    I: Send + 'static,
{
    actix_threadpool::run(f).map_err(|err| blocking_err(err))
}

fn blocking_err<E>(err: E) -> io::Error
where
    E: Send + std::fmt::Display + 'static,
{
    io::Error::new(ErrorKind::Other, format!("{}", err))
}
