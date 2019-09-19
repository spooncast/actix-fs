use actix_rt::System;
use futures::Future;
use std::io;

pub fn run<F>(f: F)
where
    F: Future<Item = (), Error = io::Error> + Send + 'static,
{
    let mut sys = System::new("test");
    sys.block_on(f).unwrap()
}
