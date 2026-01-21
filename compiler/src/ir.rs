use crate::ast::{Expr, Stmt, Type};

#[derive(Debug, Clone)]
pub enum IROp {
    // Quantum operations
    QubitAlloc(usize),          // Allocate a qubit
    QubitInit(usize, i64),      // Initialize qubit to |0> or |1>
    GateH(usize),               // Hadamard gate
    GateX(usize),               // Pauli-X gate
    GateY(usize),               // Pauli-Y gate
    GateZ(usize),               // Pauli-Z gate
    GateCNOT(usize, usize),     // CNOT gate (control, target)
    Measure(usize, usize),      // Measure qubit -> store in cbit index
    
    // Classical operations
    Return,
}

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub qubits: Vec<String>,      // qubit variable names
    pub cbits: Vec<String>,       // classical bit variable names
    pub operations: Vec<IROp>,    // execution order
}

#[derive(Debug, Clone)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
}

pub struct IRGenerator {
    qubit_counter: usize,
    cbit_counter: usize,
    variable_map: std::collections::HashMap<String, usize>, // variable name -> index
}

impl IRGenerator {
    pub fn new() -> Self {
        Self {
            qubit_counter: 0,
            cbit_counter: 0,
            variable_map: std::collections::HashMap::new(),
        }
    }
    
    pub fn generate(&mut self, ast: &crate::ast::Program) -> IRProgram {
        let mut functions = Vec::new();
        
        for func in &ast.functions {
            if let Some(ir_func) = self.generate_function(func) {
                functions.push(ir_func);
            }
        }
        
        IRProgram { functions }
    }
    
    fn generate_function(&mut self, func: &crate::ast::Function) -> Option<IRFunction> {
        self.qubit_counter = 0;
        self.cbit_counter = 0;
        self.variable_map.clear();
        
        let mut ir_func = IRFunction {
            name: func.name.clone(),
            qubits: Vec::new(),
            cbits: Vec::new(),
            operations: Vec::new(),
        };
        
        // Process statements
        for stmt in &func.body {
            self.generate_statement(stmt, &mut ir_func);
        }
        
        // Add return operation if not present
        if !ir_func.operations.iter().any(|op| matches!(op, IROp::Return)) {
            ir_func.operations.push(IROp::Return);
        }
        
        Some(ir_func)
    }
    
    fn generate_statement(&mut self, stmt: &Stmt, ir_func: &mut IRFunction) {
        match stmt {
            Stmt::Let(name, ty, expr) => {
                match ty {
                    Type::Qubit => {
                        let qubit_id = self.qubit_counter;
                        self.qubit_counter += 1;
                        ir_func.qubits.push(name.clone());
                        self.variable_map.insert(name.clone(), qubit_id);
                        
                        // Initialize qubit
                        if let Expr::LiteralQubit(value) = expr {
                            ir_func.operations.push(IROp::QubitAlloc(qubit_id));
                            if *value == 1 {
                                ir_func.operations.push(IROp::GateX(qubit_id));
                            }
                        }
                    }
                    Type::Cbit => {
                        let cbit_id = self.cbit_counter;
                        self.cbit_counter += 1;
                        ir_func.cbits.push(name.clone());
                        self.variable_map.insert(name.clone(), cbit_id);
                        
                        // Handle measurement expressions
                        if let Expr::Measure(target_expr) = expr {
                            if let Expr::Variable(target_var) = &**target_expr {
                                if let Some(target_qubit) = self.variable_map.get(target_var) {
                                    ir_func.operations.push(IROp::Measure(*target_qubit, cbit_id));
                                }
                            }
                        }
                    }
                    _ => {
                        // Classical variable - not implemented yet
                    }
                }
            }
            Stmt::Assign(var_name, expr) => {
                // Find qubit ID
                if let Some(qubit_idx) = self.variable_map.get(var_name) {
                    self.generate_gate_application(expr, *qubit_idx, ir_func);
                }
            }
            Stmt::Return(_expr) => {
                ir_func.operations.push(IROp::Return);
            }
            _ => {
                // Other statements not implemented yet
            }
        }
    }
    
    fn generate_gate_application(&mut self, expr: &Expr, target_qubit: usize, ir_func: &mut IRFunction) {
        match expr {
            Expr::GateApply(gate_name, args) => {
                match gate_name.as_str() {
                    "H" => {
                        if args.len() == 1 {
                            ir_func.operations.push(IROp::GateH(target_qubit));
                        }
                    }
                    "X" => {
                        if args.len() == 1 {
                            ir_func.operations.push(IROp::GateX(target_qubit));
                        }
                    }
                    "Y" => {
                        if args.len() == 1 {
                            ir_func.operations.push(IROp::GateY(target_qubit));
                        }
                    }
                    "Z" => {
                        if args.len() == 1 {
                            ir_func.operations.push(IROp::GateZ(target_qubit));
                        }
                    }
                    "CNOT" => {
                        if args.len() == 2 {
                            // Find control qubit
                            if let Expr::Variable(control_var) = &args[0] {
                                if let Some(control_idx) = self.variable_map.get(control_var) {
                                    ir_func.operations.push(IROp::GateCNOT(*control_idx, target_qubit));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}