// Corrected Teleportation Protocol
fn teleport(msg: qubit, alice_entangled: qubit, bob_entangled: qubit) -> int {
    // 1. Entangle Alice's msg and her half of the pair
    CNOT(msg, alice_entangled);
    H(msg);
    
    // 2. Measure Alice's qubits to get classical bits
    // Use 'let' to store measurement results
    let m1: cbit = measure(msg);
    let m2: cbit = measure(alice_entangled);
    
    // 3. Bob applies corrections based on Alice's classical outcomes
    // Classical 'if' uses results from 'measure'
    if (m2 == 1) { 
        X(bob_entangled); 
    }
    if (m1 == 1) { 
        Z(bob_entangled); 
    }
    
    return 0; // Return an int as defined in the signature
}

fn main() -> int {
    // Initialize quantum resources
    qubit q_msg = |1>;
    qubit q_alice = |0>;
    qubit q_bob = |0>;
    let q12 : string ="q_alice";
    // Execute protocol
    teleport(q_msg, q_alice, q_bob);
    
    return 0;
}