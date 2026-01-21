mod ownership_checker;

pub use ownership_checker::OwnershipChecker;

#[derive(Debug)]
pub struct SemanticAnalyzer {
    ownership_checker: OwnershipChecker,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            ownership_checker: OwnershipChecker::new(),
        }
    }
    
    pub fn analyze(&mut self, program: &crate::ast::Program) -> Result<(), Vec<String>> {
        self.ownership_checker.check_program(program)?;
        Ok(())
    }
}