use crate::ir::{IRFunction, IROp, IRProgram};

pub struct QASMGenerator;

impl QASMGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate(&self, program: &IRProgram) -> String {
        let mut output = String::new();
        
        // OpenQASM 2.0 header
        output.push_str("OPENQASM 2.0;\n");
        output.push_str("include \"qelib1.inc\";\n\n");
        
        for func in &program.functions {
            output.push_str(&self.generate_function(func));
        }
        
        output
    }
    
    fn generate_function(&self, func: &IRFunction) -> String {
        let mut output = String::new();
        
        // Create quantum and classical registers
        if !func.qubits.is_empty() {
            output.push_str(&format!("qreg q[{}];\n", func.qubits.len()));
        }
        
        if !func.cbits.is_empty() {
            output.push_str(&format!("creg c[{}];\n", func.cbits.len()));
        }
        
        if !func.qubits.is_empty() || !func.cbits.is_empty() {
            output.push_str("\n");
        }
        
        // Generate operations
        for op in &func.operations {
            output.push_str(&self.generate_operation(op));
        }
        
        output
    }
    
    fn generate_operation(&self, op: &IROp) -> String {
        match op {
            IROp::QubitAlloc(_) => String::new(), // Already handled by qreg
            IROp::QubitInit(qubit_id, value) => {
                if *value == 1 {
                    format!("x q[{}];\n", qubit_id)
                } else {
                    String::new()  // |0> is default
                }
            }
            IROp::GateH(qubit_id) => {
                format!("h q[{}];\n", qubit_id)
            }
            IROp::GateX(qubit_id) => {
                format!("x q[{}];\n", qubit_id)
            }
            IROp::GateY(qubit_id) => {
                format!("y q[{}];\n", qubit_id)
            }
            IROp::GateZ(qubit_id) => {
                format!("z q[{}];\n", qubit_id)
            }
            IROp::GateCNOT(control, target) => {
                format!("cx q[{}], q[{}];\n", control, target)
            }
            IROp::Measure(qubit, cbit) => {
                format!("measure q[{}] -> c[{}];\n", qubit, cbit)
            }
            IROp::Return => String::new(),
        }
    }
}