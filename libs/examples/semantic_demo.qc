// Demonstrates QCLang's ownership system
fn bell_state() -> (cbit, cbit) {
    // Create two qubits
    qubit alice = |0>;
    qubit bob = |0>;
    
    // Create entanglement
    alice = H(alice);
    bob = CNOT(alice, bob);
    
    // Measure both qubits (consumes them)
    cbit a_result = measure(alice);
    cbit b_result = measure(bob);
    
    return (a_result, b_result);
}

// ERROR EXAMPLES (commented out):

// fn error_use_after_measure() {
//     qubit q = |0>;
//     cbit r = measure(q);
//     q = X(q); // COMPILE ERROR: Use of measured qubit 'q'
// }

// fn error_unconsumed_qubit() {
//     qubit q = |0>;
//     q = H(q);
//     // COMPILE ERROR: Function ends with unconsumed qubit 'q'
// }

// fn error_double_measure() {
//     qubit q = |0>;
//     cbit r1 = measure(q);
//     cbit r2 = measure(q); // COMPILE ERROR: Use of consumed qubit 'q'
// }