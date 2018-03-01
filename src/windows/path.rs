use std::env::var_os;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;
use std::slice;

use winapi::shared::ntdef::PWSTR;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::winerror::S_OK;
use winapi::um::combaseapi::CoTaskMemFree;
use winapi::um::shlobj::SHGetKnownFolderPath;
use winapi::um::winbase::lstrlenW;

pub fn env_init_file() -> Option<PathBuf> {
    var_os("INPUTRC").map(PathBuf::from)
}

pub fn system_init_file() -> Option<PathBuf> {
    None
}

pub fn user_init_file() -> Option<PathBuf> {
    app_data().map(|p| p.join(r"linefeed\inputrc"))
}

#[allow(non_upper_case_globals)]
mod guid {
    DEFINE_GUID!{
        FOLDERID_RoamingAppData,
        0x3EB685DB,
        0x65F9, 0x4CF6,
        0xA0, 0x3A, 0xE3, 0xEF, 0x65, 0x72, 0x9F, 0x3D
    }
}

fn app_data() -> Option<PathBuf> {
    let mut path = ptr::null_mut();

    let res = unsafe { SHGetKnownFolderPath(
        &guid::FOLDERID_RoamingAppData,
        0,
        ptr::null_mut(),
        &mut path) };

    match res {
        S_OK => {
            let s = w_string(path);
            unsafe { CoTaskMemFree(path as LPVOID); }
            Some(PathBuf::from(s))
        }
        _ => None
    }
}

fn w_string(ptr: PWSTR) -> OsString {
    let len = unsafe { lstrlenW(ptr) };
    let slice = unsafe { slice::from_raw_parts(ptr, len as usize) };

    OsString::from_wide(slice)
}
