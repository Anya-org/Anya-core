import BitcoinSDK from '../BitcoinSDK';
import { View, Button } from 'react-native';

const BitcoinTxViewer: React.FC = () => {
  const verifyTransaction = async (txHash: string) => {
    try {
      const isValid = await BitcoinSDK.verifySPVProof(txHash);
      console.log(`Transaction valid: ${isValid}`);
    } catch (error) {
      console.error('SPV Verification failed:', error);
    }
  };

  return (
    <View>
      <Button 
        title="Create Lightning Invoice" 
        onPress={async () => {
          const invoice = await BitcoinSDK.createInvoice(1000);
          console.log('Lightning Invoice:', invoice);
        }}
      />
    </View>
  );
}; 