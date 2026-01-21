fn main() -> int { 
        qubit q = |0>;
        q = H(q);
        cbit result = measure(q);
        return 0;
    }