// FFI bindings for mobile platforms
// Separated from sdk.rs to avoid compilation issues with the full codebase

use crate::mobile::sdk::{MobileSDK, WalletInfo};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

#[no_mangle]
pub extern "C" fn anya_initialize_wallet(mnemonic: *const c_char) -> c_int {
    let sdk = MobileSDK::new();
    let mnemonic_str = unsafe { CStr::from_ptr(mnemonic).to_str().unwrap_or("") };
    match tokio::runtime::Runtime::new().unwrap().block_on(sdk.initialize_wallet(mnemonic_str)) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn anya_send_transaction(recipient: *const c_char, amount: u64) -> *mut c_char {
    let sdk = MobileSDK::new();
    let recipient_str = unsafe { CStr::from_ptr(recipient).to_str().unwrap_or("") };
    let txid = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(sdk.send_transaction(recipient_str, amount))
        .unwrap_or_default();
    CString::new(txid).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn anya_sync_wallet() -> c_int {
    let sdk = MobileSDK::new();
    match tokio::runtime::Runtime::new().unwrap().block_on(sdk.sync_wallet()) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn anya_get_wallet_info() -> *mut c_char {
    let sdk = MobileSDK::new();
    let info = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(sdk.get_wallet_info())
        .unwrap_or(WalletInfo {
            balance: 0,
            address: "".to_string(),
            last_sync: chrono::Utc::now(),
            transaction_count: 0,
        });

    let json = serde_json::json!({
        "balance": info.balance,
        "address": info.address,
        "last_sync": info.last_sync.to_rfc3339(),
        "transaction_count": info.transaction_count
    })
    .to_string();
    
    CString::new(json).unwrap().into_raw()
}

// Memory management helper for freeing strings returned by FFI functions
#[no_mangle]
pub extern "C" fn anya_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
