#[cfg(not(windows))]
use std::os::unix::fs::{self, PermissionsExt};
#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;

use std::fs::OpenOptions;

pub fn create_file(path: String) {
    let mut options = OpenOptions::new();
    options.create(true).write(true);

    #[cfg(windows)]
    {
        options.access_mode(0o666); // Permissions for Windows
    }

    let _ = options.open(path.clone());

    #[cfg(not(windows))]
    {
        let permissions = fs::Permissions::from_mode(0o777); // Permissions for Unix-based platforms
        let _ = fs::set_permissions(path, permissions);
    }
}