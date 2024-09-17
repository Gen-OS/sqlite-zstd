// https://www.sqlite.org/loadext.html
// https://github.com/jgallagher/rusqlite/issues/524#issuecomment-507787350

use rusqlite::ffi;
use rusqlite::{Connection, Result};
use std::os::raw::{c_char, c_int};

#[no_mangle]
pub unsafe extern "C" fn sqlite3_sqlitezstd_init(
    db: *mut ffi::sqlite3,
    pz_err_msg: *mut *mut c_char,
    p_api: *mut ffi::sqlite3_api_routines,
) -> c_int {
    Connection::extension_init2(db, pz_err_msg, p_api, extension_init)
}

fn extension_init(db: Connection) -> Result<bool> {
    match crate::load(&db) {
        Ok(_) => Ok(true),
        Err(e) => panic!("failed to initialize sqlite-zstd extension: {:?}", e),
    }
}
