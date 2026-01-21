use qclang_compiler::semantics::OwnershipChecker;
use qclang_compiler::ast::{Program, Function, Stmt, Expr, Type};

fn create_test_program(name: &str, body: Vec<Stmt>) -> Program {
    Program {
        functions: vec![
            Function {
                name: name.to_string(),
                params: vec![],
                return_type: Type::Int,
                body,
            }
        ],
    }
}

#[test]
fn test_valid_qubit_lifecycle() {
    let program = create_test_program("main", vec![
        Stmt::Let("q".to_string(), Type::Qubit, Expr::LiteralQubit(0)),
        Stmt::Assign("q".to_string(), 
            Expr::GateApply("H".to_string(), 
                vec![Expr::Variable("q".to_string())])),
        Stmt::Let("r".to_string(), Type::CBit, 
            Expr::Measure(Box::new(Expr::Variable("q".to_string())))),
        Stmt::Return(Some(Expr::LiteralInt(0))),
    ]);
    
    let mut checker = OwnershipChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_ok(), "Valid qubit lifecycle should pass");
}

#[test]
fn test_error_use_after_measure() {
    let program = create_test_program("main", vec![
        Stmt::Let("q".to_string(), Type::Qubit, Expr::LiteralQubit(0)),
        Stmt::Let("r".to_string(), Type::CBit, 
            Expr::Measure(Box::new(Expr::Variable("q".to_string())))),
        Stmt::Assign("q".to_string(), 
            Expr::GateApply("X".to_string(), 
                vec![Expr::Variable("q".to_string())])),
        Stmt::Return(Some(Expr::LiteralInt(0))),
    ]);
    
    let mut checker = OwnershipChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err(), "Use after measure should fail");
    
    let errors = checker.get_errors();
    assert!(errors[0].contains("measured qubit") || 
            errors[0].contains("Use of consumed"),
            "Error message should mention measured qubit");
}

#[test]
fn test_error_unconsumed_qubit() {
    let program = create_test_program("main", vec![
        Stmt::Let("q".to_string(), Type::Qubit, Expr::LiteralQubit(0)),
        Stmt::Assign("q".to_string(), 
            Expr::GateApply("H".to_string(), 
                vec![Expr::Variable("q".to_string())])),
        // q is never measured or returned!
        Stmt::Return(Some(Expr::LiteralInt(0))),
    ]);
    
    let mut checker = OwnershipChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err(), "Unconsumed qubit should fail");
    
    let errors = checker.get_errors();
    assert!(errors[0].contains("unconsumed qubit") ||
            errors[0].contains("ends with unconsumed"),
            "Error message should mention unconsumed qubit");
}

#[test]
fn test_valid_bell_state() {
    let program = create_test_program("bell", vec![
        Stmt::Let("alice".to_string(), Type::Qubit, Expr::LiteralQubit(0)),
        Stmt::Let("bob".to_string(), Type::Qubit, Expr::LiteralQubit(0)),
        Stmt::Assign("alice".to_string(), 
            Expr::GateApply("H".to_string(), 
                vec![Expr::Variable("alice".to_string())])),
        Stmt::Assign("bob".to_string(), 
            Expr::GateApply("CNOT".to_string(), 
                vec![Expr::Variable("alice".to_string()), 
                     Expr::Variable("bob".to_string())])),
        Stmt::Let("a_res".to_string(), Type::CBit, 
            Expr::Measure(Box::new(Expr::Variable("alice".to_string())))),
        Stmt::Let("b_res".to_string(), Type::CBit, 
            Expr::Measure(Box::new(Expr::Variable("bob".to_string())))),
        Stmt::Return(Some(Expr::LiteralInt(0))),
    ]);
    
    let mut checker = OwnershipChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_ok(), "Bell state should pass");
}

#[test]
fn test_error_double_measure() {
    let program = create_test_program("main", vec![
        Stmt::Let("q".to_string(), Type::Qubit, Expr::LiteralQubit(0)),
        Stmt::Let("r1".to_string(), Type::CBit, 
            Expr::Measure(Box::new(Expr::Variable("q".to_string())))),
        Stmt::Let("r2".to_string(), Type::CBit, 
            Expr::Measure(Box::new(Expr::Variable("q".to_string())))),
        Stmt::Return(Some(Expr::LiteralInt(0))),
    ]);
    
    let mut checker = OwnershipChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err(), "Double measure should fail");
}