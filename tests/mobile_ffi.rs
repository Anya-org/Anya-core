#[cfg(test)]
mod tests {
    use crate::mobile::sdk::{MobileSDK, WalletInfo};

    #[cfg(feature = "ffi")]
    mod ffi_tests {
        use super::*;
        use crate::mobile::ffi;
        use std::ffi::{CStr, CString};
        use std::os::raw::c_char;

        #[test]
        fn test_initialize_wallet() {
            let mnemonic =
                CString::new("test test test test test test test test test test test junk")
                    .unwrap();
            let result = unsafe { ffi::anya_initialize_wallet(mnemonic.as_ptr()) };
            assert_eq!(result, 0);
        }

        #[test]
        fn test_send_transaction() {
            let recipient =
                CString::new("bc1qxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxqqqqqq").unwrap();
            let txid = unsafe { ffi::anya_send_transaction(recipient.as_ptr(), 1000) };
            let result = unsafe { CStr::from_ptr(txid).to_string_lossy().into_owned() };
            assert!(!result.is_empty());
            unsafe { ffi::anya_free_string(txid) };
        }

        #[test]
        fn test_get_wallet_info() {
            let info_json = unsafe { ffi::anya_get_wallet_info() };
            let json_str = unsafe { CStr::from_ptr(info_json).to_string_lossy().into_owned() };
            assert!(json_str.contains("balance"));
            assert!(json_str.contains("address"));
            assert!(json_str.contains("last_sync"));
            assert!(json_str.contains("transaction_count"));
            unsafe { ffi::anya_free_string(info_json) };
        }
    }

    #[tokio::test]
    async fn test_sdk_initialize() {
        let sdk = MobileSDK::new();
        let result = sdk
            .initialize_wallet("test test test test test test test test test test test junk")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sdk_send_transaction() {
        let sdk = MobileSDK::new();
        let result = sdk
            .send_transaction("bc1qxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxqqqqqq", 1000)
            .await;
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_sdk_get_wallet_info() {
        let sdk = MobileSDK::new();
        let result = sdk.get_wallet_info().await;
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.balance, 0);
        assert_eq!(info.address, "addr");
        assert_eq!(info.transaction_count, 1);
    }
}
