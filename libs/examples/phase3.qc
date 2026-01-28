fn basic_test() -> int {
    // Quantum registers with bit string initialization
    qreg q[5] = |00000>;
    qreg bell[2] = |00>;
    
    // Array indexing and gate application
    q[0] = H(q[0]);  // Apply H to first qubit
    q[1] = X(q[1]);  // Apply X to second qubit
    
    // Measurements into classical array
    cbit results[3];
    for i in range(0, 3) {
        results[i] = measure(q[i]);
    }
    
    // Multi-bit initialization
    qreg pattern[4] = |1010>;
    
    return 0;
}