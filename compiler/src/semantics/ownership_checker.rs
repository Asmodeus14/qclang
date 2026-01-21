use crate::ast::{Program, Function, Stmt, Expr, Type};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
enum QubitState {
    Uninitialized,
    Alive,
    Measured,
    Consumed,
}

#[derive(Debug)]
pub struct OwnershipChecker {
    errors: Vec<String>,
    warnings: Vec<String>,
    qubit_env: HashMap<String, QubitState>,
    quantum_functions: HashSet<String>,
    current_function: String,
    current_return_type: Type, // Track the current function's return type
}

impl OwnershipChecker {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            qubit_env: HashMap::new(),
            quantum_functions: HashSet::new(),
            current_function: String::new(),
            current_return_type: Type::Unit,
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<String>> {
        // First pass: collect quantum function signatures
        for func in &program.functions {
            if self.is_quantum_function(func) {
                self.quantum_functions.insert(func.name.clone());
            }
        }
        
        // Second pass: check each function
        for func in &program.functions {
            self.current_function = func.name.clone();
            self.current_return_type = func.return_type.clone();
            self.qubit_env.clear();
            
            // Check each statement
            for stmt in &func.body {
                self.check_statement(stmt)?;
            }
            
            // At function end, enforce quantum resource cleanup
            self.check_function_exit(func)?;
        }
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
    
    fn is_quantum_function(&self, func: &Function) -> bool {
        // A function is quantum if:
        // 1. It returns a quantum type (qubit, qreg)
        // 2. It takes quantum parameters
        match &func.return_type {
            Type::Qubit | Type::Qreg(_, _) => true,
            _ => {
                // Check parameters
                for param in &func.params {
                    if matches!(param.ty, Type::Qubit | Type::Qreg(_, _)) {
                        return true;
                    }
                }
                false
            }
        }
    }
    
    fn check_statement(&mut self, stmt: &Stmt) -> Result<(), Vec<String>> {
        match stmt {
            Stmt::Let(name, ty, expr) => {
                match ty {
                    Type::Qubit => {
                        // Qubit declaration - must be initialized
                        self.qubit_env.insert(name.clone(), QubitState::Uninitialized);
                        self.check_expr(expr)?;
                        
                        // After initialization, mark as alive
                        if self.is_qubit_initializer(expr) {
                            self.qubit_env.insert(name.clone(), QubitState::Alive);
                        }
                    }
                    Type::Cbit => {
                        // Classical bit from measurement
                        self.check_expr(expr)?;
                        
                        // If this is a measurement, consume the qubit
                        if let Expr::Measure(qubit_expr) = expr {
                            if let Expr::Variable(qubit_name) = &**qubit_expr {
                                self.consume_qubit(qubit_name)?;
                            }
                        }
                    }
                    _ => {
                        // Classical variables - no quantum constraints
                        self.check_expr(expr)?;
                    }
                }
            }
            
            Stmt::Assign(var, expr) => {
                // Special quantum assignment rules
                if let Some(state) = self.qubit_env.get(var) {
                    match state {
                        QubitState::Measured | QubitState::Consumed => {
                            self.errors.push(format!(
                                "Cannot assign to qubit '{}' after it has been {}",
                                var,
                                match state {
                                    QubitState::Measured => "measured",
                                    QubitState::Consumed => "consumed",
                                    _ => unreachable!()
                                }
                            ));
                        }
                        QubitState::Alive => {
                            // Gate application consumes and produces
                            if let Expr::GateApply(_, args) = expr {
                                // Check all argument qubits are alive
                                for arg in args {
                                    if let Expr::Variable(arg_name) = arg {
                                        self.use_qubit(arg_name)?;
                                    }
                                }
                                // The LHS qubit is now re-alive
                                self.qubit_env.insert(var.clone(), QubitState::Alive);
                            }
                        }
                        _ => {}
                    }
                }
                self.check_expr(expr)?;
            }
            
            Stmt::Expr(expr) => {
                // Expression statement (like bare measurement)
                self.check_expr(expr)?;
                
                // If it's a measurement without assignment, qubit is still consumed
                if let Expr::Measure(qubit_expr) = expr {
                    if let Expr::Variable(qubit_name) = &**qubit_expr {
                        self.consume_qubit(qubit_name)?;
                    }
                }
            }
            
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.check_expr(expr)?;
                    
                    // If returning a qubit, mark it as passed out
                    if self.is_qubit_expression(expr) {
                        if let Expr::Variable(qubit_name) = expr {
                            self.consume_qubit(qubit_name)?;
                        }
                    }
                }
                
                // Check for unconsumed qubits when returning
                let unconsumed: Vec<_> = self.qubit_env.iter()
                    .filter(|(_, state)| **state == QubitState::Alive)
                    .map(|(name, _)| name.clone())
                    .collect();
                    
                if !unconsumed.is_empty() && self.current_return_type == Type::Unit {
                    self.errors.push(format!(
                        "Function '{}' returns but has unconsumed qubits: {:?}. \
                         All qubits must be measured or explicitly passed.",
                        self.current_function, unconsumed
                    ));
                }
            }
            
            _ => {} // Other statements not implemented yet
        }
        
        Ok(())
    }
    
    fn check_expr(&mut self, expr: &Expr) -> Result<(), Vec<String>> {
        match expr {
            Expr::Variable(name) => {
                if let Some(state) = self.qubit_env.get(name) {
                    match state {
                        QubitState::Uninitialized => {
                            self.errors.push(format!(
                                "Use of uninitialized qubit '{}'",
                                name
                            ));
                        }
                        QubitState::Measured | QubitState::Consumed => {
                            self.errors.push(format!(
                                "Use of {} qubit '{}'",
                                match state {
                                    QubitState::Measured => "measured",
                                    QubitState::Consumed => "consumed",
                                    _ => unreachable!()
                                },
                                name
                            ));
                        }
                        _ => {} // Alive qubits can be used
                    }
                }
            }
            
            Expr::GateApply(gate_name, args) => {
                for arg in args {
                    self.check_expr(arg)?;
                    
                    // Check gate-specific constraints
                    if gate_name == "CNOT" && args.len() == 2 {
                        // CNOT control and target must be different qubits
                        if let (Expr::Variable(a), Expr::Variable(b)) = (&args[0], &args[1]) {
                            if a == b {
                                self.errors.push(format!(
                                    "CNOT gate cannot have same qubit as control and target: '{}'",
                                    a
                                ));
                            }
                        }
                    }
                }
            }
            
            Expr::Measure(qubit_expr) => {
                self.check_expr(qubit_expr)?;
            }
            
            Expr::Call(func_name, args) => {
                // Check if this is a quantum function call
                if self.quantum_functions.contains(func_name) {
                    // Quantum function consumes its quantum arguments
                    for arg in args {
                        if let Expr::Variable(arg_name) = arg {
                            if self.qubit_env.contains_key(arg_name) {
                                self.consume_qubit(arg_name)?;
                            }
                        }
                        self.check_expr(arg)?;
                    }
                } else {
                    // Classical function - no special quantum rules
                    for arg in args {
                        self.check_expr(arg)?;
                    }
                }
            }
            
            _ => {} // Literals don't affect quantum state
        }
        
        Ok(())
    }
    
fn check_function_exit(&mut self, func: &Function) -> Result<(), Vec<String>> {
    // Check for unconsumed qubits at function exit
    let unconsumed: Vec<_> = self.qubit_env.iter()
        .filter(|(_, state)| **state == QubitState::Alive)
        .map(|(name, _)| name.clone())
        .collect();
    
    if !unconsumed.is_empty() {
        // Always error if there are unconsumed qubits, regardless of return type
        // The only exception would be if the function returns qubits, 
        // but we handle that in the Return statement
        self.errors.push(format!(
            "Function '{}' ends with unconsumed qubits: {:?}. \
             All qubits must be measured, returned, or passed to another function.",
            func.name, unconsumed
        ));
    }
    
    Ok(())
}
    
    fn use_qubit(&mut self, name: &str) -> Result<(), Vec<String>> {
        match self.qubit_env.get(name) {
            Some(QubitState::Alive) => Ok(()),
            Some(QubitState::Uninitialized) => {
                self.errors.push(format!("Use of uninitialized qubit '{}'", name));
                Err(self.errors.clone())
            }
            Some(QubitState::Measured) => {
                self.errors.push(format!("Use of measured qubit '{}'", name));
                Err(self.errors.clone())
            }
            Some(QubitState::Consumed) => {
                self.errors.push(format!("Use of consumed qubit '{}'", name));
                Err(self.errors.clone())
            }
            None => {
                // Not a qubit (classical variable) - that's OK
                Ok(())
            }
        }
    }
    
    fn consume_qubit(&mut self, name: &str) -> Result<(), Vec<String>> {
        self.use_qubit(name)?;
        self.qubit_env.insert(name.to_string(), QubitState::Consumed);
        Ok(())
    }
    
    fn is_qubit_initializer(&self, expr: &Expr) -> bool {
        matches!(expr, Expr::LiteralQubit(_))
    }
    
    fn is_qubit_expression(&self, expr: &Expr) -> bool {
        matches!(expr, Expr::Variable(name) if self.qubit_env.contains_key(name))
    }
    
    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }
    
    pub fn get_warnings(&self) -> &[String] {
        &self.warnings
    }
}