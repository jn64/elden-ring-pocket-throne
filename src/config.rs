use std::{ffi::OsString, os::windows::ffi::OsStringExt, path::PathBuf};

use configparser::ini::Ini;
use windows::{
    Win32::{
        Foundation::HMODULE,
        System::LibraryLoader::{
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            GetModuleFileNameW, GetModuleHandleExW,
        },
    },
    core::{Error as WinError, PCWSTR},
};

// Get the path of our DLL
// <https://github.com/Dasaav-dsv/erfps2/blob/1a2eeef9884d3cd126aa312a661b76fd17842f2f/src/config/updater.rs#L127>
// <https://stackoverflow.com/questions/6924195/get-dll-path-at-runtime/6924332#6924332>
fn get_module_path() -> Result<PathBuf, WinError> {
    let module_handle = unsafe {
        let mut module_handle = HMODULE::default();
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT | GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
            PCWSTR(get_module_path as *const _),
            &mut module_handle,
        )?;
        module_handle
    };

    // Approx. reasonable max length:
    // https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation
    let mut module_filename = vec![0u16; 32767];

    unsafe {
        let len = GetModuleFileNameW(Some(module_handle), &mut module_filename);

        if len == 0 || len == 32767 {
            return Err(WinError::from_thread());
        }

        module_filename.truncate(len as usize);
    }

    let path = PathBuf::from(OsString::from_wide(&module_filename));
    Ok(path)
}

// i32 expected by eldenring::util::input::is_key_pressed()
pub fn get_key() -> i32 {
    // Default key is 0x48 = H
    // <https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes>
    const DEFAULT_KEY: i32 = 0x48;

    let config_path = {
        let mut path = get_module_path().unwrap();
        path.set_file_name("pocket_throne.ini");
        path
    };

    if config_path.exists() {
        let mut config = Ini::new();
        let _map = config.load(config_path);

        // Default section name in configparser is "default",
        // even for keys without a [section]
        let key = config.get("default", "key").unwrap();

        // Convert from hex string to number
        i32::from_str_radix(key.trim_start_matches("0x"), 16).unwrap_or(DEFAULT_KEY)
    } else {
        DEFAULT_KEY
    }
}
