#[cfg(not(windows))]
use std::os::unix::fs::PermissionsExt;
use std::fs;
#[cfg(windows)]
use crate::std_err::{ErrType, StdErr};
use std::os::windows::fs::OpenOptionsExt;
use std::fs::{File, OpenOptions};

pub fn create_file(path: String) -> File {
    let mut options = OpenOptions::new();
    options.create(true).write(true);

    #[cfg(windows)]
    {
        options.access_mode(0o666); // Permissions for Windows
    }

    #[cfg(not(windows))]
    {
        let permissions = fs::Permissions::from_mode(0o777); // Permissions for Unix-based platforms
        let _ = fs::set_permissions(path, permissions);
    }

    return options.open(path.clone()).unwrap_or_else(|e| {
        StdErr::exec(ErrType::OSError, &format!("Error create file {path}: {e}"));
        todo!()
    });
}