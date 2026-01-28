// src/lib.rs - UPDATED FOR PHASE 1.5 (FIXED)
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod ir;      // OLD IR (will be deprecated)
pub mod qir;     // NEW Quantum Intermediate Representation
pub mod codegen;
pub mod semantics;
pub mod error;

use lexer::tokenize;
use parser::Parser;
use qir::builder::QirBuilder;
use qir::optimizer::QirOptimizer;
use qir::analysis::QirAnalyzer;
use semantics::SemanticAnalyzer;
use codegen::QASMGenerator;  // Import the new QASM generator

pub const VERSION: &str = "0.6.0";  // Bumped version for Phase 1.5

pub fn build_timestamp() -> &'static str {
    "2024-01-27 10:00:00"
}

pub fn git_commit_hash() -> &'static str {
    "phase1.5-qir-implementation"
}

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }
    
    pub fn version() -> &'static str {
        VERSION
    }
    
    pub fn capabilities() -> Vec<&'static str> {
        vec![
            "Phase 1.5: Quantum Intermediate Representation (QIR)",
            "• New QIR module with SSA form and linear qubit tracking",
            "• Type-safe intermediate representation",
            "• Basic optimization passes: constant folding, dead qubit elimination",
            "• Control flow representation with basic blocks",
            "• QIR analysis and verification",
            "• Gate cancellation optimization",
            "• Complete QIR-to-QASM translation backend",
            "Phase 1.4: Semantic Analyzer with Type Registry",
            "• Multi-pass semantic analysis (collect + analyze)",
            "• Type Registry for aliases, structs, and built-ins",
            "• Symbol Table with hierarchical scope management",
            "• Complete type resolution and compatibility checking",
            "• Struct field access validation",
            "• Function signature and call validation",
            "• Quantum type detection for ownership rules",
            "• Type coercion (int → float)",
            "• Control flow statement validation",
            "• Break/continue statement checking",
            "• Enhanced error reporting with source context",
            "Phase 1.3: Enhanced Type System",
            "• Type aliases with 'type' keyword",
            "• Struct definitions with 'struct' keyword",
            "• Tuple types (qubit, qubit, qubit)",
            "• Member access with dot operator",
            "• Struct literals",
            "Standard gates: H, X, Y, Z, CNOT",
            "Phase 3 gates: RX, RY, RZ, T, S, SWAP",
            "Quantum control flow: qif, qfor",
            "Quantum type system with affine types",
            "OpenQASM 2.0 output",
            "Compile-time quantum safety",
            "Quantum registers (qreg)",
            "Mutable variables (mut keyword)",
            "Enhanced assignments (+=, -=, *=, /=)",
        ]
    }
    
// In lib.rs - Update the compile_with_stats function
pub fn compile_with_stats(source: &str) -> Result<(String, CompileStats), Vec<String>> {
    // LEXING
    let tokens = tokenize(source);
    
    // PARSING
    let mut parser = Parser::new(tokens.into_iter(), source.to_string());
    let program = parser.parse_program();
    
    if !parser.errors.is_empty() {
        let errors: Vec<String> = parser.errors
            .iter()
            .map(|e| e.to_string())
            .collect();
        return Err(errors);
    }
    
    // SEMANTIC ANALYSIS
    let mut semantic_analyzer = SemanticAnalyzer::new();
    
    match semantic_analyzer.analyze_program(&program) {
        Ok(_) => {
            // Report warnings
            for warning in semantic_analyzer.get_warnings() {
                eprintln!("Warning: {}", warning);
            }
        }
        Err(errors) => {
            let error_strings: Vec<String> = errors
                .iter()
                .map(|e| e.to_string())
                .collect();
            return Err(error_strings);
        }
    }
    
    // PHASE 1.5: QIR GENERATION
    println!("Phase 1.5: Generating Quantum Intermediate Representation...");
    let mut qir_builder = QirBuilder::new();
    let mut qir_module = qir_builder.build_from_program(&program);
    
    // QIR OPTIMIZATION
    println!("  Running QIR optimizations...");
    let optimizer = QirOptimizer::new();
    optimizer.optimize_module(&mut qir_module);
    
    // QIR ANALYSIS
    println!("  Analyzing QIR...");
    let mut analyzer = QirAnalyzer::new();
    if !analyzer.analyze_module(&qir_module) {
        for error in analyzer.get_errors() {
            eprintln!("QIR Error: {}", error);
        }
        return Err(analyzer.get_errors().iter().map(|s| s.clone()).collect());
    }
    
    // Generate QASM
    println!("  Generating QASM from QIR...");
    let mut qasm_generator = QASMGenerator::new();
    let qasm_code = qasm_generator.generate(&qir_module);
    
    // Get compilation statistics
    let stats = CompileStats {
        qubits: qasm_generator.qubit_count(),
        cbits: qasm_generator.cbit_count(),
        gates: qasm_generator.gate_count(),
        measurements: qasm_generator.measurement_count(),
    };
    
    Ok((qasm_code, stats))
}
    
    pub fn compile(source: &str) -> Result<String, Vec<String>> {
        match Self::compile_with_stats(source) {
            Ok((qasm, _)) => Ok(qasm),
            Err(errors) => Err(errors),
        }
    }
    
    pub fn compile_with_diagnostics(source: &str) -> (Result<String, Vec<String>>, CompileStats) {
        match Self::compile_with_stats(source) {
            Ok((qasm, stats)) => (Ok(qasm), stats),
            Err(errors) => (Err(errors), CompileStats::default()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CompileStats {
    pub qubits: usize,
    pub cbits: usize,
    pub gates: usize,
    pub measurements: usize,
}

impl CompileStats {
    pub fn new() -> Self {
        Self {
            qubits: 0,
            cbits: 0,
            gates: 0,
            measurements: 0,
        }
    }
    
    pub fn total_operations(&self) -> usize {
        self.gates + self.measurements
    }
    
    pub fn is_empty(&self) -> bool {
        self.qubits == 0 && self.cbits == 0 && self.gates == 0 && self.measurements == 0
    }
}

impl Default for CompileStats {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CompileStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "Qubits: {}, CBits: {}, Gates: {}, Measurements: {}, Total Operations: {}",
            self.qubits,
            self.cbits,
            self.gates,
            self.measurements,
            self.total_operations()
        )
    }
}