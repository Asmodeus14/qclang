fn main() -> int {
    qubit q = |0>;
    q = H(q);
    cbit r = measure(q);
    return 0;
}