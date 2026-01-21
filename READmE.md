QCLang - A Quantum Programming Language
<p align="center"> <strong>Quantum-C Language: Combining C's performance with quantum computing capabilities</strong> </p><p align="center"> <img src="https://img.shields.io/badge/Language-Rust-orange" alt="Built with Rust"> <img src="https://img.shields.io/badge/Quantum-Programming-blue" alt="Quantum Programming"> <img src="https://img.shields.io/badge/Semantic-Checking-green" alt="Semantic Checking"> <img src="https://img.shields.io/badge/Version-0.2.0-yellow" alt="Version 0.2.0"> <img src="https://img.shields.io/badge/License-MIT-brightgreen" alt="MIT License"> </p>
🎉 What's New in v0.2.0
Quantum Correctness at Compile Time! QCLang now features semantic analysis that prevents quantum bugs before they reach hardware. Unlike Python quantum frameworks, QCLang enforces:

✅ No-cloning theorem via affine type system

✅ No-use-after-measurement via linear types

✅ Complete qubit lifecycle management

✅ Professional CLI with verbose/silent modes

🚀 Overview
QCLang is a quantum systems programming language designed to combine the performance philosophy of C with native quantum computing capabilities. It features C-like syntax, seamless classical-quantum integration, and compiles to optimized OpenQASM 2.0 for execution on quantum hardware.

Core Philosophy: "Quantum constraints should be enforced at compile time, not discovered at runtime."

✨ Key Features
C-like Syntax: Familiar syntax for classical programmers transitioning to quantum

Quantum Native Types: Built-in qubit, cbit, qreg types with affine semantics

Hardware Optimized: Direct compilation to quantum hardware operations

Semantic Safety: Compile-time prevention of quantum bugs

OpenQASM Output: Compatible with all major quantum computing platforms

Professional CLI: Clean, informative compiler output with multiple verbosity levels

📦 Installation
Quick Install (Linux/macOS)
bash
# One-line install
curl -fsSL https://raw.githubusercontent.com/yourusername/qclang/main/install.sh | bash

# Or using cargo (requires Rust)
cargo install qclang-compiler
From Source
bash
# Clone the repository
git clone https://github.com/yourusername/qclang.git
cd qclang/compiler

# Build and install
cargo build --release
sudo cp target/release/qclang /usr/local/bin/

# Verify installation
qclang --version
🚀 Quick Start
1. Create your first quantum program
rust
// hello_quantum.qc
fn main() -> int {
    // Initialize a qubit in |0⟩ state
    qubit q = |0>;
    
    // Apply Hadamard gate to create superposition
    q = H(q);
    
    // Measure the qubit
    cbit result = measure(q);
    
    return 0;
}
2. Compile to QASM
bash
qclang hello_quantum.qc
3. Generated Output
text
OPENQASM 2.0;
include "qelib1.inc";

qreg q[1];
creg c[1];

h q[0];
measure q[0] -> c[0];
📚 QCLang Language Syntax
Basic Structure
rust
// Function declaration
fn function_name(param1: type, param2: type) -> return_type {
    // Statements
    return expression;
}

// Main entry point (returns int)
fn main() -> int {
    // Your quantum circuit here
    return 0;
}
Type System
Classical Types
rust
int x = 42;                 // 64-bit integer
float pi = 3.14159;         // 64-bit floating point
bool flag = true;           // Boolean
string name = "quantum";    // String
Quantum Types
rust
qubit q = |0>;              // Single qubit (|0⟩ state)
qubit q1 = |1>;             // Single qubit (|1⟩ state)
cbit result;                // Classical bit (measurement result)
qreg q[5];                  // Quantum register (5 qubits)
Variable Declarations
rust
// Type-first declaration (C-style)
qubit q = |0>;
cbit measurement_result;

// Let declaration (Rust-style)
let x: int = 5;
let flag: bool = true;
Quantum Operations
Initialization
rust
qubit zero = |0>;           // Initialize to |0⟩
qubit one = |1>;            // Initialize to |1⟩
qreg qreg5[5];              // 5-qubit register (all |0⟩)
Single-Qubit Gates
rust
q = H(q);                   // Hadamard gate (creates superposition)
q = X(q);                   // Pauli-X (bit flip)
q = Y(q);                   // Pauli-Y
q = Z(q);                   // Pauli-Z (phase flip)
Two-Qubit Gates
rust
// CNOT (controlled-NOT) gate
target = CNOT(control, target);  // control → target

// Example: Create Bell pair
qubit alice = |0>;
qubit bob = |0>;
alice = H(alice);
bob = CNOT(alice, bob);
Measurement
rust
// Measure qubit, store result in classical bit
cbit result = measure(qubit);

// Example: Measure and use result
qubit q = |0>;
q = H(q);
cbit measurement = measure(q);
Control Flow
Conditional Statements
rust
// Classical if-else
if (x > 5) {
    // Do something
} else {
    // Do something else
}

// While loops
while (condition) {
    // Loop body
}
Blocks and Scoping
rust
{
    // Variables declared here are scoped to this block
    qubit local_q = |0>;
    // ... operations on local_q
} // local_q goes out of scope here
Functions with Quantum Parameters
rust
// Function that takes and returns qubits
fn create_bell_pair(qubit a, qubit b) -> (cbit, cbit) {
    a = H(a);
    b = CNOT(a, b);
    return (measure(a), measure(b));
}

// Function using quantum registers
fn qft(qreg q) {
    // Quantum Fourier Transform implementation
    // ... gate operations on register q
}
🔬 Compiler Architecture
Complete Pipeline
Lexical Analysis → Token stream with quantum-specific tokens

Parsing → Abstract Syntax Tree (AST) with error recovery

Semantic Analysis → Quantum correctness checking (affine types)

IR Generation → Intermediate representation with qubit tracking

Code Generation → Optimized OpenQASM 2.0 output

Semantic Analysis (New in v0.2.0)
QCLang enforces quantum constraints at compile time:

Affine Types: Qubits can be used exactly once

No Cloning: Prevents violations of no-cloning theorem

Resource Management: All qubits must be consumed (measured or returned)

📁 Project Structure
text
qclang/
├── compiler/              # Main compiler implementation
│   ├── src/
│   │   ├── lexer.rs      # Tokenizer with quantum tokens
│   │   ├── parser.rs     # Parser with error recovery
│   │   ├── ast.rs        # Abstract Syntax Tree definitions
│   │   ├── ir.rs         # Intermediate Representation
│   │   ├── semantics/    # Semantic analysis (NEW!)
│   │   │   ├── mod.rs
│   │   │   └── ownership_checker.rs  # Affine type system
│   │   └── codegen/
│   │       └── qasm.rs   # OpenQASM code generator
│   ├── Cargo.toml
│   └── src/bin/
│       └── qclang.rs     # CLI interface
├── libs/
│   ├── examples/         # Example programs
│   │   ├── hello.qc      # Hello Quantum
│   │   ├── bell.qc       # Bell state
│   │   ├── teleport.qc   # Quantum teleportation
│   │   └── semantic_demo.qc # Error examples
│   └── stdlib/           # Standard library (future)
├── tests/                # Test suite
├── scripts/              # Build and install scripts
└── README.md             # This file
🛠️ Usage
Basic Compilation
bash
# Compile to QASM (outputs to input.qasm)
qclang circuit.qc

# Specify output file
qclang circuit.qc output.qasm

# Verbose mode (shows QASM preview)
qclang -v circuit.qc

# Silent mode (minimal output)
qclang -s circuit.qc
Compiler Options
bash
qclang --help
Output:

text
QCLang Quantum Compiler v0.2.0

USAGE:
    qclang [FLAGS] <INPUT> [OUTPUT]

FLAGS:
    -v, --verbose      Verbose output
    -s, --silent       Silent mode (minimal output)
    -q, --quiet        Quiet mode (no banner)
    -h, --help         Print help information
    -V, --version      Print version information

ARGS:
    <INPUT>         Input QCLang source file (.qc)
    <OUTPUT>        Output QASM file [default: <input>.qasm]
🧪 Examples
Example 1: Bell State (Entanglement)
rust
// bell.qc - Create maximally entangled pair
fn create_bell_pair() -> (cbit, cbit) {
    qubit alice = |0>;
    qubit bob = |0>;
    
    // Create superposition on Alice's qubit
    alice = H(alice);
    
    // Entangle with Bob's qubit
    bob = CNOT(alice, bob);
    
    // Measure both qubits
    cbit a_result = measure(alice);
    cbit b_result = measure(bob);
    
    return (a_result, b_result);
}
Example 2: Quantum Teleportation
rust
// teleport.qc - Quantum teleportation protocol
fn teleport_state(qubit psi) -> cbit {
    // Create Bell pair between Alice and Bob
    qubit alice = |0>;
    qubit bob = |0>;
    alice = H(alice);
    bob = CNOT(alice, bob);
    
    // Bell measurement
    psi = CNOT(psi, alice);
    psi = H(psi);
    
    cbit m1 = measure(psi);
    cbit m2 = measure(alice);
    
    // Classical bits m1, m2 would be sent to Bob
    // Bob applies corrections based on these bits
    
    return m1;  // Return one of the measurement results
}
Example 3: Error Detection (Compile-Time Safety)
rust
// This code will NOT compile - demonstrates semantic checking
fn error_example() -> int {
    qubit q = |0>;
    cbit r = measure(q);  // q is consumed here
    
    // COMPILE ERROR: Use of consumed qubit 'q'
    // q = X(q);
    
    return 0;
}
🎯 Current Features
✅ Fully Implemented
Complete Compiler Pipeline: Lexer → Parser → Semantic Check → IR → QASM

Quantum Types: qubit, cbit, qreg with proper semantics

Basic Gates: H, X, Y, Z, CNOT

Measurement: cbit result = measure(qubit);

Classical Control: Variables, assignments, control flow

Semantic Analysis: Affine type system for qubits

Professional CLI: Clean output with progress tracking

🔄 Coming Soon
Extended Gate Set: RX, RY, RZ, T, S, SWAP

Quantum Control Flow: Conditional gates, quantum loops

Standard Library: Common quantum algorithms

Hardware Backends: Direct IBM/AWS/Rigetti integration

Optimization Passes: Gate cancellation, circuit optimization

🏆 Performance
Compilation Speed
Native Binary: < 10ms for typical circuits

Memory Usage: ~5MB binary size

Optimized Output: Clean, minimal QASM code

Safety Advantages vs. Python Frameworks
Feature	QCLang	Qiskit/Cirq
Compile-time errors	✅ Quantum bugs caught before runtime	❌ Discovered at hardware execution
No-cloning enforcement	✅ Affine type system	❌ Possible to violate theorem
Resource management	✅ All qubits tracked	❌ Manual management required
Performance	✅ Native binary	⚠️ Python interpreter overhead
🤝 Contributing
We welcome contributions! The project is perfect for:

Quantum enthusiasts wanting to learn compiler development

Rust developers interested in quantum computing

Researchers prototyping quantum algorithms

Educators teaching quantum programming concepts

Getting Started
bash
# 1. Fork and clone
git clone https://github.com/yourusername/qclang.git
cd qclang/compiler

# 2. Build and test
cargo build
cargo test

# 3. Run examples
cargo run -- ../libs/examples/hello.qc

# 4. Make changes and submit PR!
Contribution Areas
Language Features: New quantum operations, syntax improvements

Optimizations: Circuit optimization algorithms

Backends: Support for additional quantum hardware

Tooling: IDE plugins, debuggers, visualizers

Documentation: Tutorials, examples, API docs

📄 License
MIT License - see LICENSE file for details.

📞 Support & Community
GitHub Issues: Report bugs or request features

GitHub Discussions: Ask questions, share ideas

Contributing Guide: CONTRIBUTING.md

🙏 Acknowledgments
Rust Community: For excellent tools and libraries

OpenQASM: Quantum assembly language standard

Qiskit, Cirq, Quil: Inspiration from existing quantum frameworks

Logos: Lexer implementation library

🌟 Star History
If you find QCLang useful, please consider giving it a star! ⭐

<p align="center"> <em>Building the future of quantum computing with compile-time safety</em> </p><p align="center"> <a href="https://github.com/yourusername/qclang/stargazers"> <img src="https://img.shields.io/github/stars/yourusername/qclang?style=social" alt="GitHub stars"> </a> <a href="https://github.com/yourusername/qclang/network/members"> <img src="https://img.shields.io/github/forks/yourusername/qclang?style=social" alt="GitHub forks"> </a> <a href="https://github.com/yourusername/qclang/watchers"> <img src="https://img.shields.io/github/watchers/yourusername/qclang?style=social" alt="GitHub watchers"> </a> </p>