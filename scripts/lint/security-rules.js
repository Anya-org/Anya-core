// [AIS-3] Add constant-time validation
function validateConstantTime(code) {
  const forbiddenPatterns = [
    /if\s*\(\s*secret\s*==\s*input\s*\)/,
    /memcmp\(.*,\s*.*,\s*LEN\)\s*!=\s*0/
  ];
  
  return forbiddenPatterns.every(pattern => 
    !pattern.test(code)
  );
} 