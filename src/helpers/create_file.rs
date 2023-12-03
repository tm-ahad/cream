use std::os::unix::fs::PermissionsExt;
use std::fs::{OpenOptions, self};

pub fn create_file(path: String) {
    let _ = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path);

    // Set file permissions after creating the file
    if let Err(e) = fs::set_permissions(&path, fs::Permissions::from_mode(0o777)) {
        panic!("{e}")
    }
}
