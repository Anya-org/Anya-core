package org.anya.mobile.core

/**
 * Kotlin wrapper for Anya Core SDK
 *
 * This class provides a Kotlin-friendly interface for the native
 * Anya Core SDK functions exposed through JNI.
 */
class AnyaCore {
    companion object {
        init {
            System.loadLibrary("anya_mobile_sdk")
        }

        // JNI function declarations
        @JvmStatic external fun anya_initialize_wallet(mnemonic: String): Int
        @JvmStatic external fun anya_send_transaction(recipient: String, amount: Long): String
        @JvmStatic external fun anya_sync_wallet(): Int
        @JvmStatic external fun anya_get_wallet_info(): String
        @JvmStatic external fun anya_authenticate_biometric(): Int
        @JvmStatic external fun anya_backup_wallet(destination: String): Int
        @JvmStatic external fun anya_wipe_wallet(): Int
        @JvmStatic external fun anya_estimate_fee(amount: Long): Long
        @JvmStatic external fun anya_free_string(ptr: Long): Void
    }

    /**
     * Initialize the wallet with a mnemonic phrase
     * @param mnemonic The BIP39 mnemonic phrase
     * @return true if successful, false otherwise
     */
    fun initializeWallet(mnemonic: String): Boolean {
        return anya_initialize_wallet(mnemonic) == 0
    }

    /**
     * Send a bitcoin transaction
     * @param recipient The recipient's bitcoin address
     * @param amount The amount to send in satoshis
     * @return Transaction ID if successful, empty string otherwise
     */
    fun sendTransaction(recipient: String, amount: Long): String {
        return anya_send_transaction(recipient, amount)
    }

    /**
     * Synchronize the wallet with the blockchain
     * @return true if successful, false otherwise
     */
    fun syncWallet(): Boolean {
        return anya_sync_wallet() == 0
    }

    /**
     * Get wallet information as a JSON string
     * @return JSON string containing wallet info (balance, address, last_sync, transaction_count)
     */
    fun getWalletInfo(): String {
        return anya_get_wallet_info()
    }

    /**
     * Authenticate using device biometrics
     * @return true if authenticated, false otherwise
     */
    fun authenticateBiometric(): Boolean {
        val result = anya_authenticate_biometric()
        return result == 1
    }

    /**
     * Backup wallet to specified location
     * @param destination Path to backup location
     * @return true if successful, false otherwise
     */
    fun backupWallet(destination: String): Boolean {
        return anya_backup_wallet(destination) == 0
    }

    /**
     * Wipe all wallet data
     * @return true if successful, false otherwise
     */
    fun wipeWallet(): Boolean {
        return anya_wipe_wallet() == 0
    }

    /**
     * Estimate fee for transaction
     * @param amount Amount to send in satoshis
     * @return Estimated fee in satoshis
     */
    fun estimateFee(amount: Long): Long {
        return anya_estimate_fee(amount)
    }
}
