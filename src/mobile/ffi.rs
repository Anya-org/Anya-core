// FFI bindings for mobile platforms
// Separated from sdk.rs to avoid compilation issues with the full codebase

use crate::mobile::sdk::{MobileSDK, WalletInfo};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

/// Initialize the wallet using the provided mnemonic.
///
/// # Safety
/// `mnemonic` must be a valid pointer to a null-terminated C string whose data
/// remains valid for the duration of this call.
#[no_mangle]
pub unsafe extern "C" fn anya_initialize_wallet(mnemonic: *const c_char) -> c_int {
    let sdk = MobileSDK::new();
    // SAFETY: Caller must ensure mnemonic is a valid, null-terminated C string pointer.
    let mnemonic_str = CStr::from_ptr(mnemonic).to_str().unwrap_or("");
    match tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(sdk.initialize_wallet(mnemonic_str))
    {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Send a transaction to the given recipient with the specified amount.
/// Returns a newly allocated C string containing the transaction id.
///
/// # Safety
/// `recipient` must be a valid pointer to a null-terminated C string. The
/// returned pointer must be freed by calling `anya_free_string`.
#[no_mangle]
pub unsafe extern "C" fn anya_send_transaction(
    recipient: *const c_char,
    amount: u64,
) -> *mut c_char {
    let sdk = MobileSDK::new();
    // SAFETY: Caller must ensure recipient is a valid, null-terminated C string pointer.
    let recipient_str = CStr::from_ptr(recipient).to_str().unwrap_or("");
    let txid = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(sdk.send_transaction(recipient_str, amount))
        .unwrap_or_default();
    CString::new(txid).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn anya_sync_wallet() -> c_int {
    let sdk = MobileSDK::new();
    match tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(sdk.sync_wallet())
    {
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
/// Free a C string previously returned by an Anya FFI function.
///
/// # Safety
/// `ptr` must have been allocated by an Anya FFI function and not previously freed.
#[no_mangle]
pub unsafe extern "C" fn anya_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        // SAFETY: Pointer must originate from anya_* FFI allocation and not be freed yet.
        let _ = CString::from_raw(ptr);
    }
}
