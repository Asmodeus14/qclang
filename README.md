# QCLang - A Quantum Programming Language

<p align="center">
  <strong>Quantum-C Language: Combining C's performance with quantum computing capabilities</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust-orange" alt="Built with Rust">
  <img src="https://img.shields.io/badge/Quantum-Programming-blue" alt="Quantum Programming">
  <img src="https://img.shields.io/badge/Semantic-Checking-green" alt="Semantic Checking">
  <img src="https://img.shields.io/badge/Version-0.4.0-yellow" alt="Version 0.4.0">
  <img src="https://img.shields.io/badge/License-MIT-brightgreen" alt="MIT License">
  <img src="https://img.shields.io/badge/OpenQASM-2.0-red" alt="OpenQASM 2.0">
</p>

## 🚀 **What's New in v0.4.0**

**Professional CLI & Complete Quantum Pipeline!** QCLang v0.4.0 introduces a comprehensive command-line interface and full quantum compilation pipeline with advanced features:

### ✅ **New Professional CLI**
- **Multiple Commands**: `compile`, `run`, `test`, `check`, `capabilities`
- **Colorful Output**: Emojis and ANSI colors for better UX
- **Progress Indicators**: Real-time compilation progress
- **Verbose/Silent Modes**: Detailed or minimal output

### ✅ **Advanced Features**
- **Quantum Registers**: `qreg q[5] = |00000>` multi-qubit support
- **Mutable Variables**: `mut int counter = 0` with compound assignments
- **Enhanced Gates**: RX, RY, RZ, T, S, SWAP gates
- **Quantum Control Flow**: `qif`, `qfor` for quantum-aware loops
- **JSON Output**: Option to export compilation metadata

### ✅ **Quantum Correctness at Compile Time**
Unlike Python quantum frameworks, QCLang enforces:
- **No-cloning theorem** via affine type system
- **No-use-after-measurement** via linear types  
- **Complete qubit lifecycle management**
- **Array bounds checking** for quantum registers

---

## 🎯 **Overview**

QCLang is a **quantum systems programming language** designed to combine the performance philosophy of C with native quantum computing capabilities. It features C-like syntax, seamless classical-quantum integration, and compiles to optimized OpenQASM 2.0 for execution on quantum hardware.

**Core Philosophy**: *"Quantum constraints should be enforced at compile time, not discovered at runtime."*

---

## ✨ **Key Features**

### **Language Features**
- **C-like Syntax**: Familiar syntax for classical programmers transitioning to quantum
- **Quantum Native Types**: Built-in `qubit`, `cbit`, `qreg` types with affine semantics
- **Mutable Classical Variables**: `mut` keyword with compound assignments (`+=`, `-=`, `*=`, `/=`)

### **Quantum Operations**
- **Standard Gates**: H, X, Y, Z, CNOT
- **Phase 3 Gates**: RX, RY, RZ, T, S, SWAP
- **Quantum Registers**: Multi-qubit arrays with bit-string initialization
- **Measurement**: Direct to classical bits with proper ownership tracking

### **Compiler Features**
- **Complete Pipeline**: Lexer → Parser → Semantic Check → IR → OpenQASM
- **Semantic Safety**: Compile-time prevention of quantum bugs
- **Hardware Optimized**: Direct compilation to quantum hardware operations
- **OpenQASM 2.0 Output**: Compatible with all major quantum computing platforms
- **Professional CLI**: Clean, informative compiler output with multiple verbosity levels

---

## 📦 **Installation**

### **From Source (Recommended)**

```bash
# Clone the repository
git clone https://github.com/Asmodeus14/qclang.git
cd qclang/compiler

# Build and install
cargo build --release
sudo cp target/release/qclang /usr/local/bin/

# Verify installation
qclang --version
```

### **Using Cargo**

```bash
# Install from crates.io (coming soon)
cargo install qclang-compiler

# Or install directly from git
cargo install --git https://github.com/Asmodeus14/qclang.git
```

### **Binary Releases**

```bash
# Download for Linux x86_64
wget https://github.com/Asmodeus14/qclang/releases/download/v0.4.0/qclang-linux-x86_64
chmod +x qclang-linux-x86_64
sudo mv qclang-linux-x86_64 /usr/local/bin/qclang
```

---

## 🚀 **Quick Start**

### **1. Create your first quantum program**

```rust
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
```

### **2. Compile with the new CLI**

```bash
# Basic compilation
qclang compile hello_quantum.qc

# With verbose output
qclang compile hello_quantum.qc --verbose --show

# Run with detailed statistics
qclang run hello_quantum.qc
```

### **3. Generated Output**

```openqasm
OPENQASM 2.0;
include "qelib1.inc";

qreg q[1];
creg c[1];

h q[0];
measure q[0] -> c[0];
```

---

## 📚 **QCLang Language Syntax**

### **Basic Structure**

```rust
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
```

### **Type System**

#### **Classical Types**
```rust
int x = 42;                 // 64-bit integer
float pi = 3.14159;         // 64-bit floating point
bool flag = true;           // Boolean
string name = "quantum";    // String

// Mutable variables (new in v0.4.0)
mut int counter = 0;
counter += 1;               // Compound assignment
```

#### **Quantum Types**
```rust
qubit q = |0>;              // Single qubit (|0⟩ state)
qubit q1 = |1>;             // Single qubit (|1⟩ state)
cbit result;                // Classical bit (measurement result)
qreg q[5] = |00000>;        // Quantum register (5 qubits, new in v0.4.0)
```

### **Quantum Operations**

#### **Quantum Registers (New!)**
```rust
qreg q[3] = |000>;          // 3-qubit register initialized to |000>
q[0] = H(q[0]);             // Apply H to qubit 0
q[1] = X(q[1]);             // Apply X to qubit 1
cbit result = measure(q[2]); // Measure qubit 2
```

#### **Single-Qubit Gates**
```rust
q = H(q);                   // Hadamard gate (creates superposition)
q = X(q);                   // Pauli-X (bit flip)
q = Y(q);                   // Pauli-Y
q = Z(q);                   // Pauli-Z (phase flip)
q = RX(1.57, q);            // Rotation X gate (new)
q = T(q);                   // T gate (π/8 phase, new)
```

#### **Two-Qubit Gates**
```rust
// CNOT (controlled-NOT) gate
target = CNOT(control, target);

// SWAP gate (new)
(a, b) = SWAP(a, b);

// Example: Create Bell pair
qubit alice = |0>;
qubit bob = |0>;
alice = H(alice);
bob = CNOT(alice, bob);
```

#### **Measurement**
```rust
// Measure qubit, store result in classical bit
cbit result = measure(qubit);

// Example: Measure and use result
qubit q = |0>;
q = H(q);
cbit measurement = measure(q);
```

### **Control Flow**

#### **Classical Control Flow**
```rust
// Conditional statements
if (x > 5) {
    // Do something
} else {
    // Do something else
}

// For loops with range (new in v0.4.0)
for i in range(0, 10) {
    qubit q = |0>;
    q = H(q);
    cbit result = measure(q);
}

// While loops
while (condition) {
    // Loop body
}
```

#### **Quantum Control Flow (New!)**
```rust
// Quantum if (qif)
qif (condition) {
    // Quantum operations
} qelse {
    // Alternative quantum operations
}

// Quantum for loop (qfor)
qfor i in range(0, n) {
    // Quantum operations that can be superposed
}
```

### **Functions with Parameters**

```rust
// Function that takes mutable parameters
fn increment(mut int x) -> int {
    x += 1;
    return x;
}

// Function with quantum parameters
fn create_bell_pair(qubit a, qubit b) -> (cbit, cbit) {
    a = H(a);
    b = CNOT(a, b);
    return (measure(a), measure(b));
}
```

---

## 🔬 **Compiler Architecture**

### **Complete Pipeline**
```
Lexical Analysis → Token stream with quantum-specific tokens
       ↓
Parsing → Abstract Syntax Tree (AST) with error recovery
       ↓
Semantic Analysis → Quantum correctness checking (affine types)
       ↓
IR Generation → Intermediate representation with qubit tracking
       ↓
Code Generation → Optimized OpenQASM 2.0 output
```

### **Semantic Analysis (Advanced in v0.4.0)**
QCLang enforces quantum constraints at compile time:

1. **Affine Types**: Qubits can be used exactly once
2. **No Cloning**: Prevents violations of no-cloning theorem  
3. **Resource Management**: All qubits must be consumed (measured or returned)
4. **Quantum Immutability**: Quantum types cannot be `mut`
5. **Array Bounds Checking**: Quantum register indices verified at compile time

---

## 🛠️ **Usage**

### **CLI Commands (New in v0.4.0)**

```bash
# Show help
qclang --help

# Show version
qclang version

# Show capabilities
qclang capabilities

# Compile single file
qclang compile circuit.qc

# Compile multiple files with progress bar
qclang compile *.qc -o build/

# Compile with JSON output
qclang compile circuit.qc -f json

# Run with detailed statistics
qclang run bell_state.qc --verbose

# Check syntax without compilation
qclang check *.qc

# Run test suite
qclang test
```

### **Command Options**

```bash
# Compile with options
qclang compile circuit.qc \
  --output optimized.qasm \
  --format both \
  --show \
  --verbose

# Run with different verbosity levels
qclang run circuit.qc              # Normal output
qclang run circuit.qc --verbose    # Detailed output
```

---

## 🧪 **Examples**

### **Example 1: Quantum Register with Mutable Counter**

```rust
// qreg_demo.qc - Quantum registers with classical computation
fn main() -> int {
    qreg q[3] = |000>;          // 3-qubit register
    mut int zero_count = 0;     // Mutable counter
    
    // Apply gates to each qubit
    q[0] = H(q[0]);
    q[1] = X(q[1]);
    q[2] = H(q[2]);
    
    // Measure and count zeros
    for i in range(0, 3) {
        cbit result = measure(q[i]);
        if result == 0 {
            zero_count += 1;    // Mutable operation
        }
    }
    
    return zero_count;
}
```

### **Example 2: Phase Gates and Rotations**

```rust
// phase_gates.qc - Demonstrating new gate set
fn main() -> int {
    qubit q = |0>;
    
    // Standard gates
    q = H(q);
    q = X(q);
    q = Y(q);
    q = Z(q);
    
    // Phase gates (new in v0.4.0)
    q = T(q);           // T gate (π/8)
    q = S(q);           // S gate (π/4)
    
    // Rotation gates with angles
    q = RX(1.57, q);    // Rotate X by π/2
    q = RY(0.785, q);   // Rotate Y by π/4
    q = RZ(3.14, q);    // Rotate Z by π
    
    cbit result = measure(q);
    return 0;
}
```

### **Example 3: Error Detection (Compile-Time Safety)**

```rust
// This code will NOT compile - demonstrates semantic checking
fn error_example() -> int {
    qubit q = |0>;
    cbit r = measure(q);  // q is consumed here
    
    // COMPILE ERROR: Use of consumed qubit 'q'
    // q = X(q);
    
    // COMPILE ERROR: Quantum types cannot be mutable
    // mut qubit mq = |0>;
    
    // COMPILE ERROR: Array index out of bounds
    // qreg q2[2] = |00>;
    // q[5] = H(q[5]);  // Only indices 0-1 are valid
    
    return 0;
}
```

### **Example 4: Complex Quantum Algorithm Structure**

```rust
// algorithm.qc - Shows full language capabilities
fn quantum_algorithm(mut int iterations) -> int {
    qreg data[4] = |0000>;      // Data register
    qubit ancilla = |0>;        // Ancilla qubit
    mut int success_count = 0;
    
    for i in range(0, iterations) {
        // Initialize superposition
        data[0] = H(data[0]);
        data[1] = H(data[1]);
        
        // Entangle qubits
        data[2] = CNOT(data[0], data[2]);
        data[3] = CNOT(data[1], data[3]);
        
        // Quantum operation with ancilla
        ancilla = H(ancilla);
        data[0] = CNOT(ancilla, data[0]);
        
        // Measurement and classical processing
        cbit results[4];
        for j in range(0, 4) {
            results[j] = measure(data[j]);
        }
        
        // Classical post-processing
        if (results[0] == results[1]) {
            success_count += 1;
        }
        
        // Reset for next iteration
        data = |0000>;
        ancilla = |0>;
    }
    
    return success_count;
}
```

---

## 🎯 **Current Features (v0.4.0)**

### **✅ Fully Implemented**
- **Complete Compiler Pipeline**: Lexer → Parser → Semantic Check → IR → OpenQASM
- **Quantum Types**: `qubit`, `cbit`, `qreg` with proper semantics
- **Extended Gate Set**: H, X, Y, Z, CNOT, RX, RY, RZ, T, S, SWAP
- **Quantum Registers**: Multi-qubit arrays with initialization
- **Mutable Variables**: `mut` keyword with compound assignments
- **Measurement**: `cbit result = measure(qubit);`
- **Classical Control**: Variables, assignments, control flow
- **Semantic Analysis**: Affine type system for qubits
- **Professional CLI**: Multiple commands with color output
- **JSON Output**: Compilation metadata export

### **🔄 In Progress**
- **Quantum Functions**: First-class quantum operations
- **Standard Library**: Common quantum algorithms
- **Circuit Optimization**: Gate cancellation, qubit reuse
- **Hardware Backends**: Direct IBM/AWS/Rigetti integration

### **📅 Planned Features**
- **Quantum Modules**: Import/export of quantum circuits
- **Advanced Types**: Quantum channels, mixed states
- **Error Correction**: Built-in error correction circuits
- **Visualization**: Circuit diagram generation
- **Debugger**: Quantum state inspection

---

## 🏆 **Performance**

### **Compilation Speed**
- **Native Binary**: < 10ms for typical circuits
- **Memory Usage**: ~5MB binary size
- **Optimized Output**: Clean, minimal QASM code

### **Safety Advantages vs. Python Frameworks**

| Feature | QCLang | Qiskit/Cirq |
|---------|---------|-------------|
| **Compile-time errors** | ✅ Quantum bugs caught before runtime | ❌ Discovered at hardware execution |
| **No-cloning enforcement** | ✅ Affine type system | ❌ Possible to violate theorem |
| **Resource management** | ✅ All qubits tracked | ❌ Manual management required |
| **Performance** | ✅ Native binary | ⚠️ Python interpreter overhead |
| **Type safety** | ✅ Static typing | ❌ Dynamic typing |
| **Quantum registers** | ✅ Native multi-qubit support | ⚠️ Manual array management |

---

## 📁 **Project Structure**

```
qclang/
├── compiler/              # Main compiler implementation
│   ├── src/
│   │   ├── lexer.rs      # Tokenizer with quantum tokens
│   │   ├── parser.rs     # Parser with error recovery
│   │   ├── ast.rs        # Abstract Syntax Tree definitions
│   │   ├── ir.rs         # Intermediate Representation
│   │   ├── semantics/    # Semantic analysis
│   │   │   ├── mod.rs
│   │   │   └── ownership_checker.rs  # Affine type system
│   │   └── codegen/
│   │       ├── mod.rs
│   │       └── qasm.rs   # OpenQASM code generator
│   ├── Cargo.toml
│   └── src/bin/
│       └── qclang.rs     # Professional CLI interface
├── libs/
│   ├── examples/         # Example programs
│   │   ├── hello.qc      # Hello Quantum
│   │   ├── bell.qc       # Bell state
│   │   ├── teleport.qc   # Quantum teleportation
│   │   ├── qreg_demo.qc  # Quantum register demo
│   │   └── error_demo.qc # Compile-time error examples
│   └── stdlib/           # Standard library (future)
├── tests/                # Test suite
├── scripts/              # Build and install scripts
└── README.md             # This file
```

---

## 🤝 **Contributing**

We welcome contributions! The project is perfect for:

- **Quantum enthusiasts** wanting to learn compiler development
- **Rust developers** interested in quantum computing
- **Researchers** prototyping quantum algorithms
- **Educators** teaching quantum programming concepts

### **Getting Started**

```bash
# 1. Fork and clone
git clone https://github.com/Asmodeus14/qclang.git
cd qclang/compiler

# 2. Build and test
cargo build
cargo test

# 3. Run examples
qclang run ../libs/examples/bell.qc

# 4. Make changes and submit PR!
```

### **Contribution Areas**
- **Language Features**: New quantum operations, syntax improvements
- **Optimizations**: Circuit optimization algorithms
- **Backends**: Support for additional quantum hardware
- **Tooling**: IDE plugins, debuggers, visualizers
- **Documentation**: Tutorials, examples, API docs

---

## 📄 **License**

MIT License - see [LICENSE](LICENSE) file for details.

---

## 📞 **Support & Community**

- **GitHub Issues**: Report bugs or request features
- **GitHub Discussions**: Ask questions, share ideas
- **Contributing Guide**: [CONTRIBUTING.md](CONTRIBUTING.md)

---

## 🙏 **Acknowledgments**

- **Rust Community**: For excellent tools and libraries
- **OpenQASM**: Quantum assembly language standard
- **Qiskit, Cirq, Quil**: Inspiration from existing quantum frameworks
- **Logos**: Lexer implementation library

---

<p align="center">
  <em>Building the future of quantum computing with compile-time safety</em>
</p>

<p align="center">
  <a href="https://github.com/Asmodeus14/qclang/stargazers">
    <img src="https://img.shields.io/github/stars/Asmodeus14/qclang?style=social" alt="GitHub stars">
  </a>
  <a href="https://github.com/Asmodeus14/qclang/network/members">
    <img src="https://img.shields.io/github/forks/Asmodeus14/qclang?style=social" alt="GitHub forks">
  </a>
  <a href="https://github.com/Asmodeus14/qclang/watchers">
    <img src="https://img.shields.io/github/watchers/Asmodeus14/qclang?style=social" alt="GitHub watchers">
  </a>
</p>