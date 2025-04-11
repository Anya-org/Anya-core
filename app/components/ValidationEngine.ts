if (mergeConflicts.some(conflict => conflict.file.includes('.bip'))) {
    throw new Error('Merge contains unresolved BIP specification conflicts'); 
  }