use actix_fs::*;
use std::fs;
use tempfile::tempdir;

mod rt;

#[test]
fn create() {
    let base_dir = tempdir().unwrap();
    let new_dir = base_dir.path().join("foo");

    rt::run({ create_dir(new_dir.clone()) });

    assert!(new_dir.is_dir());
}

#[test]
fn create_all() {
    let base_dir = tempdir().unwrap();
    let new_dir = base_dir.path().join("foo").join("bar");

    rt::run({ create_dir_all(new_dir.clone()) });

    assert!(new_dir.is_dir());
}

#[test]
fn remove() {
    let base_dir = tempdir().unwrap();
    let new_dir = base_dir.path().join("foo");

    fs::create_dir(new_dir.clone()).unwrap();

    rt::run({ remove_dir(new_dir.clone()) });

    assert!(!new_dir.exists());
}
