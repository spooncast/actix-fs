use futures::Future;
use std::convert::From;
use std::fs::{self, OpenOptions as StdOpenOptions, File as StdFile};
use std::io;
use std::path::Path;

/// A reference to an open file on the filesystem.
///
/// This is a specialized version of [`std::fs::File`][std] for usage from the
/// Actix runtime.
///
/// An instance of a `File` can be read and/or written depending on what options
/// it was opened with. Files also implement Seek to alter the logical cursor
/// that the file contains internally.
///
/// Files are automatically closed when they go out of scope.
///
/// [std]: https://doc.rust-lang.org/std/fs/struct.File.html
#[derive(Debug)]
pub struct File {
    std: Option<StdFile>,
}

impl File {
    /// Attempts to open a file in read-only mode.
    ///
    /// See [`OpenOptions`] for more details.
    ///
    /// [`OpenOptions`]: struct.OpenOptions.html
    ///
    /// # Errors
    ///
    /// `OpenFuture` results in an error if called from outside of the Actix
    /// runtime or if the underlying [`open`] call results in an error.
    ///
    /// [`open`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
    pub fn open<P>(path: P) -> impl Future<Item = File, Error = io::Error>
    where
        P: AsRef<Path> + Send + 'static,
    {
        OpenOptions::new().read(true).open(path)
    }

    /// Opens a file in write-only mode.
    ///
    /// This function will create a file if it does not exist, and will truncate
    /// it if it does.
    ///
    /// See [`OpenOptions`] for more details.
    ///
    /// [`OpenOptions`]: struct.OpenOptions.html
    ///
    /// # Errors
    ///
    /// `CreateFuture` results in an error if called from outside of the Actix
    /// runtime or if the underlying [`create`] call results in an error.
    ///
    /// [`create`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.create
    pub fn create<P>(path: P) -> impl Future<Item = File, Error = io::Error>
    where
        P: AsRef<Path> + Send + 'static,
    {
        crate::blocking(move || -> io::Result<File> {
            let std = StdFile::create(path.as_ref())?;
            let file = File::from_std(std);
            Ok(file)
        })
    }

    /// Convert a [`std::fs::File`][std] to a [`actix_fs::File`][file].
    ///
    /// [std]: https://doc.rust-lang.org/std/fs/struct.File.html
    /// [file]: struct.File.html
    pub fn from_std(std: StdFile) -> File {
        File { std: Some(std) }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        if let Some(_std) = self.std.take() {
            // This is probably fine as closing a file *shouldn't* be a blocking
            // operation. That said, ideally `shutdown` is called first.
        }
    }
}

/// Options and flags which can be used to configure how a file is opened.
///
/// This is a specialized version of [`std::fs::OpenOptions`] for usage from
/// the Actix runtime.
///
/// `From<std::fs::OpenOptions>` is implemented for more advanced configuration
/// than the methods provided here.
///
/// [`std::fs::OpenOptions`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
#[derive(Clone, Debug)]
pub struct OpenOptions(StdOpenOptions);

impl OpenOptions {
    /// Creates a blank new set of options ready for configuration.
    ///
    /// All options are initially set to `false`.
    pub fn new() -> OpenOptions {
        OpenOptions(StdOpenOptions::new())
    }

    /// See the underlying [`read`] call for details.
    ///
    /// [`read`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.read
    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.0.read(read);
        self
    }

    /// See the underlying [`write`] call for details.
    ///
    /// [`write`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.write
    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.0.write(write);
        self
    }

    /// See the underlying [`append`] call for details.
    ///
    /// [`append`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.append
    pub fn append(&mut self, append: bool) -> &mut OpenOptions {
        self.0.append(append);
        self
    }

    /// See the underlying [`truncate`] call for details.
    ///
    /// [`truncate`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.truncate
    pub fn truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        self.0.truncate(truncate);
        self
    }

    /// See the underlying [`create`] call for details.
    ///
    /// [`create`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create
    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        self.0.create(create);
        self
    }

    /// See the underlying [`create_new`] call for details.
    ///
    /// [`create_new`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create_new
    pub fn create_new(&mut self, create_new: bool) -> &mut OpenOptions {
        self.0.create_new(create_new);
        self
    }

    /// Opens a file at `path` with the options specified by `self`.
    ///
    /// # Errors
    ///
    /// `OpenOptionsFuture` results in an error if called from outside of the
    /// Actix runtime or if the underlying [`open`] call results in an error.
    ///
    /// [`open`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.open
    pub fn open<P>(&self, path: P) -> impl Future<Item = File, Error = io::Error>
    where
        P: AsRef<Path> + Send + 'static,
    {
        let opt = self.0.clone();
        crate::blocking(move || -> io::Result<File> {
            let std = opt.open(path.as_ref())?;
            let file = File::from_std(std);
            Ok(file)
        })
    }
}

impl From<StdOpenOptions> for OpenOptions {
    fn from(options: StdOpenOptions) -> OpenOptions {
        OpenOptions(options)
    }
}

/// Removes a file from the filesystem.
///
/// Note that there is no
/// guarantee that the file is immediately deleted (e.g. depending on
/// platform, other open file descriptors may prevent immediate removal).
///
/// This is an async version of [`std::fs::remove_file`][std]
///
/// [std]: https://doc.rust-lang.org/std/fs/fn.remove_file.html
pub fn remove_file<P>(path: P) -> impl Future<Item = (), Error = io::Error>
where
    P: AsRef<Path> + Send + 'static,
{
    crate::blocking(move || fs::remove_file(path.as_ref()))
}

/// Rename a file or directory to a new name, replacing the original file if
/// `to` already exists.
///
/// This will not work if the new name is on a different mount point.
///
/// This is an async version of [`std::fs::rename`][std]
///
/// [std]: https://doc.rust-lang.org/std/fs/fn.rename.html
pub fn rename<P, Q>(from: P, to: Q) -> impl Future<Item = (), Error = io::Error>
where
    P: AsRef<Path> + Send + 'static,
    Q: AsRef<Path> + Send + 'static,
{
    crate::blocking(move || fs::rename(from.as_ref(), to.as_ref()))
}
