// src/qir/optimizer.rs - QIR Optimizations - FIXED
use super::*;
use std::collections::{HashMap, HashSet};

pub struct QirOptimizer {
    pub enable_gate_cancellation: bool,
    pub enable_dead_qubit_elimination: bool,
    pub enable_constant_folding: bool,
    pub enable_common_subexpression_elimination: bool,
}

impl QirOptimizer {
    pub fn new() -> Self {
        Self {
            enable_gate_cancellation: true,
            enable_dead_qubit_elimination: true,
            enable_constant_folding: true,
            enable_common_subexpression_elimination: true,
        }
    }
    
    pub fn optimize_module(&self, module: &mut QirModule) {
        for func in &mut module.functions {
            self.optimize_function(func);
        }
    }
    
    pub fn optimize_function(&self, func: &mut QirFunction) {
        // Run optimizations in sequence
        if self.enable_constant_folding {
            self.constant_folding(func);
        }
        
        if self.enable_dead_qubit_elimination {
            self.dead_qubit_elimination(func);
        }
        
        if self.enable_gate_cancellation {
            self.gate_cancellation(func);
        }
        
        if self.enable_common_subexpression_elimination {
            self.common_subexpression_elimination(func);
        }
        
        // Clean up empty blocks
        self.remove_empty_blocks(func);
    }
    
    fn constant_folding(&self, func: &mut QirFunction) {
        for block in func.blocks.values_mut() {
            let mut new_ops = Vec::new();
            
            for op in &block.ops {
                // Always push the original operation for now
                // TODO: Implement actual constant folding
                new_ops.push(op.clone());
            }
            
            block.ops = new_ops;
        }
    }
    
    fn dead_qubit_elimination(&self, func: &mut QirFunction) {
        // This is a simplified version that doesn't actually track qubit usage
        // TODO: Implement proper qubit usage tracking
        // For now, just pass through
    }
    
    fn gate_cancellation(&self, func: &mut QirFunction) {
        // Look for consecutive gates on the same qubit that cancel each other
        for block in func.blocks.values_mut() {
            let mut i = 0;
            while i < block.ops.len() {
                if i + 1 < block.ops.len() {
                    if let (QirOp::ApplyGate { gate: gate1, args: args1, result: _ }, 
                           QirOp::ApplyGate { gate: gate2, args: args2, result: _ }) = 
                           (&block.ops[i], &block.ops[i + 1]) {
                        
                        // Check if gates are inverses on the same qubits
                        if self.gates_cancel(gate1, gate2, args1, args2) {
                            // Remove both gates
                            block.ops.remove(i + 1);
                            block.ops.remove(i);
                            continue; // Don't increment i
                        }
                    }
                }
                i += 1;
            }
        }
    }
    
    fn gates_cancel(&self, gate1: &QirGate, gate2: &QirGate, args1: &[QirValue], args2: &[QirValue]) -> bool {
        // Basic gate cancellations
        match (gate1, gate2) {
            (QirGate::H, QirGate::H) => args1 == args2,
            (QirGate::X, QirGate::X) => args1 == args2,
            (QirGate::Y, QirGate::Y) => args1 == args2,
            (QirGate::Z, QirGate::Z) => args1 == args2,
            (QirGate::S, QirGate::Sdg) => args1 == args2,
            (QirGate::Sdg, QirGate::S) => args1 == args2,
            (QirGate::T, QirGate::Tdg) => args1 == args2,
            (QirGate::Tdg, QirGate::T) => args1 == args2,
            _ => false,
        }
    }
    
    fn common_subexpression_elimination(&self, func: &mut QirFunction) {
        // Simplified version that doesn't use QirValue as HashMap key
        // TODO: Implement proper CSE
    }
    
    fn remove_empty_blocks(&self, func: &mut QirFunction) {
        let mut to_remove = Vec::new();
        
        for (&block_id, block) in &func.blocks {
            if block.ops.is_empty() && block_id != func.entry_block {
                // Check if this block is just a jump to another block
                if block.successors.len() == 1 {
                    // We could redirect predecessors, but for now just mark for removal
                    to_remove.push(block_id);
                }
            }
        }
        
        for block_id in to_remove {
            func.blocks.remove(&block_id);
            
            // Update successors/predecessors in other blocks
            for block in func.blocks.values_mut() {
                block.predecessors.retain(|&id| id != block_id);
                block.successors.retain(|&id| id != block_id);
            }
        }
    }
}