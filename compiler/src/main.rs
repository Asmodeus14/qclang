mod lexer;
mod ast;
mod parser;
mod ir;
mod codegen;
mod semantics;

use lexer::tokenize;  // Use our tokenize function
use parser::Parser;
use ir::IRGenerator;
use codegen::qasm::QASMGenerator;
use semantics::OwnershipChecker;

fn compile_and_test(source: &str, name: &str) -> Result<(crate::ast::Program, String), Vec<String>> {
    println!("\n=== TEST: {} ===", name);
    println!("Source:\n```rust\n{}\n```", source);
    
    // 1. Lexical analysis
    let tokens = tokenize(source);
    
    // 2. Parsing
    let mut parser = Parser::new(tokens.into_iter());
    let program = parser.parse_program();
    
    if !parser.errors.is_empty() {
        println!("❌ Parsing errors:");
        for error in &parser.errors {
            println!("  - {}", error);
        }
        return Err(parser.errors);
    }
    println!("✅ Parsing successful");
    
    // 3. Semantic analysis
    println!("\n=== SEMANTIC ANALYSIS ===");
    let mut checker = OwnershipChecker::new();
    match checker.check_program(&program) {
        Ok(_) => {
            println!("✅ Semantic checks passed");
        }
        Err(errors) => {
            println!("❌ Semantic errors:");
            for error in &errors {
                println!("  - {}", error);
            }
            return Err(errors);
        }
    }
    
    // 4. IR generation
    println!("\n=== GENERATING IR ===");
    let mut ir_gen = IRGenerator::new();
    let ir_program = ir_gen.generate(&program);
    println!("✅ IR generated");
    
    // 5. QASM generation
    println!("\n=== GENERATING QASM ===");
    let qasm_gen = QASMGenerator::new();
    let qasm_code = qasm_gen.generate(&ir_program);
    println!("✅ QASM code generated");
    
    Ok((program, qasm_code))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 QCLang Compiler v0.2.0");
    println!("=======================\n");
    
    // Test 1: Valid quantum program
    let source1 = "fn main() -> int { 
        qubit q = |0>;
        q = H(q);
        cbit result = measure(q);
        return 0;
    }";
    
    match compile_and_test(source1, "Valid quantum program") {
        Ok((_, qasm)) => {
            println!("\n=== GENERATED QASM ===");
            println!("{}", qasm);
            
            // Save files
            std::fs::write("../libs/examples/valid.qasm", &qasm)?;
            std::fs::write("../libs/examples/valid.qc", source1)?;
            println!("\n📁 Files saved: valid.qc, valid.qasm");
        }
        Err(errors) => {
            eprintln!("Compilation failed with {} error(s)", errors.len());
        }
    }
    
    // Test 2: ERROR - Use after measurement
    let source2 = "fn main() -> int { 
        qubit q = |0>;
        cbit r = measure(q);
        q = X(q);  // ERROR: Use after measurement
        return 0;
    }";
    
    println!("\n\n=== EXPECTING ERROR: Use after measurement ===");
    match compile_and_test(source2, "Invalid: Use after measurement") {
        Ok(_) => {
            println!("❌ UNEXPECTED: Should have failed!");
        }
        Err(errors) => {
            println!("✅ CORRECT: Compilation failed as expected");
            for error in errors {
                println!("  - {}", error);
            }
        }
    }
    
    // Test 3: ERROR - Unconsumed qubit
    let source3 = "fn main() -> int { 
        qubit q = |0>;
        q = H(q);
        // q is never measured!
        return 0;
    }";
    
    println!("\n\n=== EXPECTING ERROR: Unconsumed qubit ===");
    match compile_and_test(source3, "Invalid: Unconsumed qubit") {
        Ok(_) => {
            println!("❌ UNEXPECTED: Should have failed!");
        }
        Err(errors) => {
            println!("✅ CORRECT: Compilation failed as expected");
            for error in errors {
                println!("  - {}", error);
            }
        }
    }
    
    // Test 4: Bell state
    let source4 = "fn main() -> int { 
        qubit a = |0>;
        qubit b = |0>;
        a = H(a);
        b = CNOT(a, b);
        cbit a_res = measure(a);
        cbit b_res = measure(b);
        return 0;
    }";
    
    match compile_and_test(source4, "Bell state") {
        Ok((_, qasm)) => {
            println!("\n=== GENERATED QASM ===");
            println!("{}", qasm);
        }
        Err(errors) => {
            eprintln!("Compilation failed with {} error(s)", errors.len());
        }
    }
    
    Ok(())
}