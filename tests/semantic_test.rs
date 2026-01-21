use qclang_compiler::semantics::OwnershipChecker;
use qclang_compiler::ast::{Program, Function, Stmt, Expr, Type};

#[test]
fn test_valid_qubit_lifecycle() {
    let program = Program {
        functions: vec![
            Function {
                name: "main".to_string(),
                params: vec![],
                return_type: Type::Int,
                body: vec![
                    Stmt::Let("q".to_string(), Type::Qubit, Expr::LiteralQubit(0)),
                    Stmt::Assign("q".to_string(), Expr::GateApply("H".to_string(), 
                        vec![Expr::Variable("q".to_string())])),
                    Stmt::Let("r".to_string(), Type::CBit, 
                        Expr::Measure(Box::new(Expr::Variable("q".to_string())))),
                    Stmt::Return(Some(Expr::LiteralInt(0))),
                ],
            }
        ],
    };
    
    let mut checker = OwnershipChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_ok(), "Valid qubit lifecycle should pass");
}

#[test]
fn test_error_use_after_measure() {
    let program = Program {
        functions: vec![
            Function {
                name: "main".to_string(),
                params: vec![],
                return_type: Type::Int,
                body: vec![
                    Stmt::Let("q".to_string(), Type::Qubit, Expr::LiteralQubit(0)),
                    Stmt::Let("r".to_string(), Type::CBit, 
                        Expr::Measure(Box::new(Expr::Variable("q".to_string())))),
                    Stmt::Assign("q".to_string(), Expr::GateApply("X".to_string(), 
                        vec![Expr::Variable("q".to_string())])), // ERROR!
                    Stmt::Return(Some(Expr::LiteralInt(0))),
                ],
            }
        ],
    };
    
    let mut checker = OwnershipChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err(), "Use after measure should fail");
    assert!(checker.get_errors()[0].contains("measured qubit"));
}

#[test]
fn test_error_unconsumed_qubit() {
    let program = Program {
        functions: vec![
            Function {
                name: "main".to_string(),
                params: vec![],
                return_type: Type::Int,
                body: vec![
                    Stmt::Let("q".to_string(), Type::Qubit, Expr::LiteralQubit(0)),
                    Stmt::Assign("q".to_string(), Expr::GateApply("H".to_string(), 
                        vec![Expr::Variable("q".to_string())])),
                    // q is never measured or returned!
                    Stmt::Return(Some(Expr::LiteralInt(0))),
                ],
            }
        ],
    };
    
    let mut checker = OwnershipChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err(), "Unconsumed qubit should fail");
    assert!(checker.get_errors()[0].contains("unconsumed qubits"));
}