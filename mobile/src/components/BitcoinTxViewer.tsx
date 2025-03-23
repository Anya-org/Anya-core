import BitcoinSDK from '../BitcoinSDK';
import { View, Button, Text } from 'react-native';
import { useState, useEffect } from 'react';

const BitcoinTxViewer: React.FC = () => {
  const [walletInfo, setWalletInfo] = useState<WalletInfo | null>(null);

  useEffect(() => {
    const loadWallet = async () => {
      const info = await BitcoinSDK.getWalletInfo();
      setWalletInfo(info);
    };
    loadWallet();
  }, []);

  const verifyTransaction = async (txHash: string) => {
    try {
      const isValid = await BitcoinSDK.verifySPVProof(txHash);
      console.log(`Transaction valid: ${isValid}`);
    } catch (error) {
      console.error('SPV Verification failed:', error);
    }
  };

  const handlePayment = async () => {
    try {
      const invoice = await BitcoinSDK.createInvoice(1000);
      const psbt = await BitcoinSDK.signTransaction(invoice);
      await BitcoinSDK.broadcastTransaction(psbt);
    } catch (err) {
      console.error('Payment failed:', err);
    }
  };

  return (
    <View>
      {walletInfo && (
        <Text>
          Balance: {walletInfo.balance} sats
          Last Sync: {walletInfo.lastSync}
        </Text>
      )}
      <Button 
        title="Create Lightning Invoice" 
        onPress={handlePayment}
      />
    </View>
  );
}; 