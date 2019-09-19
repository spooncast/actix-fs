use futures::Future;

use std::fs;
use std::io;
use std::path::Path;

/// Creates a new, empty directory at the provided path
///
/// This is an async version of [`std::fs::create_dir`][std]
///
/// [std]: https://doc.rust-lang.org/std/fs/fn.create_dir.html
pub fn create_dir<P>(path: P) -> impl Future<Item = (), Error = io::Error>
where
    P: AsRef<Path> + Send + 'static,
{
    crate::blocking(move || fs::create_dir(path.as_ref()))
}

/// Recursively create a directory and all of its parent components if they
/// are missing.
///
/// This is an async version of [`std::fs::create_dir_all`][std]
///
/// [std]: https://doc.rust-lang.org/std/fs/fn.create_dir_all.html
pub fn create_dir_all<P>(path: P) -> impl Future<Item = (), Error = io::Error>
where
    P: AsRef<Path> + Send + 'static,
{
    crate::blocking(move || fs::create_dir_all(path.as_ref()))
}

/// Removes an existing, empty directory.
///
/// This is an async version of [`std::fs::remove_dir`][std]
///
/// [std]: https://doc.rust-lang.org/std/fs/fn.remove_dir.html
pub fn remove_dir<P>(path: P) -> impl Future<Item = (), Error = io::Error>
where
    P: AsRef<Path> + Send + 'static,
{
    crate::blocking(move || fs::remove_dir(path.as_ref()))
}
