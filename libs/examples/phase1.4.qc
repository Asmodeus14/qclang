// phase1.4.qc - Corrected version
struct Point {
    x: int,
    y: int,
};

struct Complex {
    real: float,
    imag: float,
};

struct QubitPair {
    q1: qubit,
    q2: qubit,
};

fn main() -> unit {
    // Struct initialization
    let p1: Point = Point { x: 10, y: 20 };
    
    // Old-style declaration (if supported)
    Point p2 = Point { x: 30, y: 40 };
    
    // Access struct fields
    p1.x = 15;
    
    // Struct with quantum types
    let qp: QubitPair = QubitPair { q1: |0>, q2: |1> };
    
    // Apply gates to struct members
    h(qp.q1);
    cnot(qp.q1, qp.q2);
    
    // Complex number example
    let c1: Complex = Complex { real: 1.5, imag: 2.5 };
    let c2: Complex = Complex { real: 0.5, imag: -1.5 };
    
    return;
}

fn distance(p1: Point, p2: Point) -> float {
    // Function using structs - FIXED: No float() casting function
    let dx: int = p1.x - p2.x;
    let dy: int = p1.y - p2.y;
    
    // Calculate Euclidean distance - use explicit conversion
    // Convert int to float by multiplying by 1.0 or using as_float() if available
    let dx_f: float = 1.0 * dx;  // This converts int to float
    let dy_f: float = 1.0 * dy;
    
    // If you have a sqrt function
    let dist: float = dx_f * dx_f + dy_f * dy_f;  // Actually squared distance
    
    // If you want actual sqrt, you might need to implement it or use approximation
    return dist;
}

fn entangle_qubits() -> QubitPair {
    // Create and entangle qubits
    let qp: QubitPair = QubitPair { q1: |0>, q2: |0> };
    h(qp.q1);
    cnot(qp.q1, qp.q2);
    return qp;
}

// Example function to show int to float conversion
fn int_to_float(n: int) -> float {
    // Convert int to float - multiply by 1.0
    return 1.0 * n;
}