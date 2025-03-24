let rng = OsRng; // Secure system RNG

// Add constant-time comparison
if verify_constant_time(signature, expected) { 