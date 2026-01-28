// Final comprehensive test
type CustomQubit = qubit;
struct CircuitResult {
    measured: cbit,
    success: bool
};

fn run_circuit(q: CustomQubit) -> CircuitResult {
    q = H(q);
    q = X(q);
    let result: cbit = measure(q);
    return CircuitResult { measured: result, success: true };
}

fn main() -> int {
    CustomQubit my_q = |0>;
    CircuitResult res = run_circuit(my_q);
    return 0;
}
