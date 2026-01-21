# QCLang Language Manifesto

## Core Philosophy

**"Quantum constraints should be enforced at compile time, not discovered at runtime."**

## What Makes QCLang Different

### 1. Compile-Time Quantum Correctness
Unlike Python quantum frameworks that allow invalid quantum operations at runtime, QCLang enforces:
- **No-cloning theorem** at the type system level
- **No-use-after-measurement** via affine types
- **Complete qubit lifecycle management**

### 2. Systems Programming Mindset
- **Zero-cost abstractions**: What you write is what runs on hardware
- **Explicit resource management**: No hidden allocations or garbage collection
- **Predictable performance**: No interpreter overhead, no JIT warmup

### 3. Education-First Design
- **Clear error messages**: "Cannot use qubit 'q' after measurement" not "Segmentation fault"
- **Gradual complexity**: Start with simple circuits, add complexity as needed
- **No magic**: Every quantum operation is explicit in the source code

## Target Audience

1. **Systems Programmers** exploring quantum computing
2. **Educators** teaching quantum programming principles
3. **Performance Engineers** optimizing quantum-classical workflows
4. **Tooling Developers** building quantum infrastructure

## Non-Goals

1. **Not another Python library** - We're a standalone language
2. **Not a pulse-level control language** - We target gate-level abstractions
3. **Not a quantum algorithm DSL** - We're general-purpose
4. **Not a research language** - We prioritize practical usability

## Success Metrics

A QCLang program is successful when:
1. It compiles without quantum errors
2. The generated circuit is optimal for the target hardware
3. The performance is predictable and reproducible
4. The code is understandable to both classical and quantum programmers

## The QCLang Promise

"Write quantum circuits with the safety of Rust and the performance of C."