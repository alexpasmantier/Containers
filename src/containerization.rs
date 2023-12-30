use libc;
use std::{ffi, fs};
use tempfile::{tempdir, TempDir};

use crate::registry::layers::decompress;

/// Creates a temporary directory, creates /dev/null inside it, and copies the given binaries as
/// well as the image layers into it
pub fn prepare_temp_dir(bin_paths: &[&str], img_layers: Vec<bytes::Bytes>) -> TempDir {
    let temp_dir = tempdir().expect("Failed to create temporary directory for the container");
    let temp_dir_path = temp_dir.path();
    // create /dev/null inside temp dir
    fs::create_dir(temp_dir_path.join("dev")).expect("Failed to create dev/ inside temp dir");
    fs::write(temp_dir_path.join("dev/null"), b"")
        .expect("Failed to create dev/null inside temp dir");

    // copy all we need
    for path in bin_paths {
        let p = temp_dir_path.join(path.strip_prefix("/").expect("Paths should be absolute"));
        // create intermediary directories
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::copy(path, p).expect(&format!("Unable to copy {:?} to temp dir", path));
    }

    // write image layers to disk
    img_layers.iter().for_each(|l| {
        decompress(l, temp_dir_path).unwrap();
    });

    temp_dir
}

pub fn chroot(destination_path: &str) -> i32 {
    let temp_dir_cstring = ffi::CString::new(destination_path).unwrap();
    unsafe { libc::chroot(temp_dir_cstring.as_ptr()) }
}

pub fn unshare() -> i32 {
    let unshare_flags = libc::CLONE_NEWPID;
    unsafe { libc::unshare(unshare_flags) }
}
