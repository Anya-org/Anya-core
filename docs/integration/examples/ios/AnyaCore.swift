import Foundation

/**
 * Swift wrapper for Anya Core SDK
 *
 * This class provides a Swift-friendly interface for the native
 * Anya Core SDK functions exposed through Objective-C bridging.
 */
public class AnyaCore {
    
    // MARK: - Initialization
    
    /**
     * Initialize the wallet with a mnemonic phrase
     * @param mnemonic The BIP39 mnemonic phrase
     * @return true if successful, false otherwise
     */
    public static func initializeWallet(mnemonic: String) -> Bool {
        return anya_initialize_wallet(mnemonic) == 0
    }
    
    /**
     * Send a bitcoin transaction
     * @param recipient The recipient's bitcoin address
     * @param amount The amount to send in satoshis
     * @return Transaction ID if successful, empty string otherwise
     */
    public static func sendTransaction(recipient: String, amount: UInt64) -> String {
        guard let cString = anya_send_transaction(recipient, amount) else {
            return ""
        }
        
        let result = String(cString: cString)
        anya_free_string(cString)
        return result
    }
    
    /**
     * Synchronize the wallet with the blockchain
     * @return true if successful, false otherwise
     */
    public static func syncWallet() -> Bool {
        return anya_sync_wallet() == 0
    }
    
    /**
     * Get wallet information as a JSON string
     * @return JSON string containing wallet info (balance, address, last_sync, transaction_count)
     */
    public static func getWalletInfo() -> String {
        guard let cString = anya_get_wallet_info() else {
            return "{}"
        }
        
        let result = String(cString: cString)
        anya_free_string(cString)
        return result
    }
    
    /**
     * Authenticate using device biometrics
     * @return true if authenticated, false otherwise
     */
    public static func authenticateBiometric() -> Bool {
        let result = anya_authenticate_biometric()
        return result == 1
    }
    
    /**
     * Backup wallet to specified location
     * @param destination Path to backup location
     * @return true if successful, false otherwise
     */
    public static func backupWallet(destination: String) -> Bool {
        return anya_backup_wallet(destination) == 0
    }
    
    /**
     * Wipe all wallet data
     * @return true if successful, false otherwise
     */
    public static func wipeWallet() -> Bool {
        return anya_wipe_wallet() == 0
    }
    
    /**
     * Estimate fee for transaction
     * @param amount Amount to send in satoshis
     * @return Estimated fee in satoshis
     */
    public static func estimateFee(amount: UInt64) -> UInt64 {
        return anya_estimate_fee(amount)
    }
}

// MARK: - Bridging Header

// This would be in AnyaCore-Bridging-Header.h:
// 
// #ifndef AnyaCore_Bridging_Header_h
// #define AnyaCore_Bridging_Header_h
// 
// #include <stdint.h>
// 
// int32_t anya_initialize_wallet(const char *mnemonic);
// char* anya_send_transaction(const char *recipient, uint64_t amount);
// int32_t anya_sync_wallet(void);
// char* anya_get_wallet_info(void);
// int32_t anya_authenticate_biometric(void);
// int32_t anya_backup_wallet(const char *destination);
// int32_t anya_wipe_wallet(void);
// uint64_t anya_estimate_fee(uint64_t amount);
// void anya_free_string(char *ptr);
// 
// #endif /* AnyaCore_Bridging_Header_h */
