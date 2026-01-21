pub mod lexer;
pub mod ast;
pub mod parser;
pub mod ir;
pub mod codegen;
pub mod semantics;

use lexer::tokenize;
use parser::Parser;
use ir::IRGenerator;
use codegen::qasm::QASMGenerator;
use semantics::OwnershipChecker;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Handle build info that might not be available
pub fn build_timestamp() -> &'static str {
    option_env!("BUILD_TIMESTAMP").unwrap_or("unknown")
}

pub fn git_commit_hash() -> &'static str {
    option_env!("GIT_COMMIT_HASH").unwrap_or("unknown")
}

pub struct Compiler {
    ownership_checker: OwnershipChecker,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            ownership_checker: OwnershipChecker::new(),
        }
    }
    
    pub fn version() -> &'static str {
        VERSION
    }
    
    pub fn compile(&mut self, source: &str) -> Result<String, Vec<String>> {
        // 1. Lexical analysis
        let tokens = tokenize(source);
        
        // 2. Parsing
        let mut parser = Parser::new(tokens.into_iter());
        let program = parser.parse_program();
        
        if !parser.errors.is_empty() {
            return Err(parser.errors);
        }
        
        // 3. Semantic analysis
        self.ownership_checker.check_program(&program)?;
        
        // 4. IR generation
        let mut ir_gen = IRGenerator::new();
        let ir_program = ir_gen.generate(&program);
        
        // 5. QASM generation
        let qasm_gen = QASMGenerator::new();
        let qasm_code = qasm_gen.generate(&ir_program);
        
        Ok(qasm_code)
    }
}