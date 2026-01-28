// quantum_affine.qc - Correct quantum programming pattern
type MyQubit = qubit;
type MyCbit = cbit;

fn main() -> int {
    // Create a qubit
    MyQubit q1 = |0>;
    
    // Apply H gate - must use NEW variable
    MyQubit q2 = H(q1);  // q1 is "consumed", q2 is the result
    
    // Apply X gate - again new variable
    MyQubit q3 = X(q2);  // q2 is consumed, q3 is the result
    
    // Measure - returns classical bit
    MyCbit c = measure(q3);  // q3 is consumed by measurement
    
    return 0;
}