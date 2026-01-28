// bell.qc - Fixed with proper type alias syntax
// Type alias for quantum circuit
type QuantumCircuit = qreg[10];

// Struct definition
struct Experiment {
    circuit: QuantumCircuit,
    name: string,
}

// Helper function
fn initialize_circuit() -> QuantumCircuit {
    let q: QuantumCircuit = |0000000000>;
    for i in range(0, 10) {
        q[i] = H(q[i]);
    }
    return q;
}

fn run_experiment() -> Experiment {
    let circuit = initialize_circuit();
    
    return Experiment {
        circuit: circuit,
        name: "superposition_test",
    };
}

fn main() -> int {
    let exp = run_experiment();
    return 0;
}