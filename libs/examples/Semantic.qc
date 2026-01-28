// error_test.qc - Tests for semantic error checking
// This should fail compilation with specific errors

// ERROR: Use after measurement
fn test_use_after_measure() -> int {
    qubit q = |0>;
    cbit r = measure(q);
    q = X(q);  // ERROR: Using qubit after measurement
    return 0;
}

// ERROR: Unconsumed qubit
fn test_unconsumed_qubit() -> int {
    qubit q = |0>;
    q = H(q);
    // ERROR: q is never measured!
    return 0;
}

// ERROR: Uninitialized qubit - FIXED: In QCLang, qubits must be initialized
// This is actually a syntax error, so let's remove this test or use different syntax
fn test_invalid_syntax() -> int {
    // qubit q; // This would be a syntax error in our language
    qubit q = |0>; // Valid
    q = H(q);
    return 0;
}

// ERROR: CNOT with same qubit
fn test_cnot_self() -> int {
    qubit q = |0>;
    q = CNOT(q, q);  // ERROR: Same qubit for control and target
    return 0;
}

// ERROR: SWAP with same qubit
fn test_swap_self() -> int {
    qubit q = |0>;
    q = SWAP(q, q);  // ERROR: Same qubit for SWAP
    return 0;
}