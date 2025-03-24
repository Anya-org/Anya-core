// [BIP-370] PSBTv2 Validation
function validatePSBTv2(psbt) {
  const requiredFields = [
    'globalTx',
    'inputCount',
    'outputCount',
    'txVersion=2'
  ];
  
  return requiredFields.every(field => 
    psbt.metadata.includes(field)
  ) && psbt.inputs.length === psbt.inputCount;
} 