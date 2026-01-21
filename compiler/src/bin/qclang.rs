use clap::{Arg, ArgAction, Command};
use std::fs;
use std::path::Path;
use std::process;
use std::time::Instant;
use qclang_compiler::lexer::tokenize;
use qclang_compiler::parser::Parser;
use qclang_compiler::ir::IRGenerator;
use qclang_compiler::codegen::qasm::QASMGenerator;
use qclang_compiler::semantics::OwnershipChecker;

// ANSI color codes
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";

fn print_step(step: &str, status: &str) {
    println!("{:12} {}", step, status);
}

fn print_error(msg: &str) {
    eprintln!("{}{}error:{}{} {}", RED, BOLD, RESET, RED, msg);
}

fn print_warning(msg: &str) {
    println!("{}{}warning:{}{} {}", YELLOW, BOLD, RESET, YELLOW, msg);
}

fn print_info(msg: &str) {
    println!("{}{}info:{}{} {}", CYAN, BOLD, RESET, CYAN, msg);
}

fn print_success(msg: &str) {
    println!("{}{}success:{}{} {}", GREEN, BOLD, RESET, GREEN, msg);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    
    // Set up CLI with clap - Using standard conventions
    let matches = Command::new("qclang")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Quantum systems programming language compiler")
        .arg(
            Arg::new("input")
                .help("Input QCLang source file (.qc)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("Output QASM file")
                .index(2),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Verbose output"),
        )
        .arg(
            Arg::new("silent")
                .short('s')
                .long("silent")
                .action(ArgAction::SetTrue)
                .help("Silent mode (minimal output)"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("Quiet mode (no banner)")
        )
        .get_matches();
    
    let verbose = matches.get_flag("verbose");
    let silent = matches.get_flag("silent");
    let quiet = matches.get_flag("quiet");
    
    // Print banner (unless silent or quiet mode)
    if !silent && !quiet {
        println!("{}┌─────────────────────────────────────────────────────┐{}", DIM, RESET);
        println!("{}│{} {}{}QCLang Quantum Compiler v{}     {}│{}", 
            DIM, RESET, BOLD, CYAN, env!("CARGO_PKG_VERSION"), DIM, RESET);
        println!("{}│{} {}A quantum systems programming language{}        {}│{}", DIM, RESET, DIM, RESET, DIM, RESET);
        println!("{}└─────────────────────────────────────────────────────┘{}\n", DIM, RESET);
    }
    
    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").map(|s| s.as_str()).unwrap_or_else(|| {
        let input_path = Path::new(input_file);
        let stem = input_path.file_stem().unwrap_or_default();
        Box::leak(format!("{}.qasm", stem.to_string_lossy()).into_boxed_str())
    });
    
    // Check if input file exists
    if !Path::new(input_file).exists() {
        if !silent {
            print_error(&format!("Input file not found: '{}'", input_file));
        }
        process::exit(1);
    }
    
    if !silent {
        print_info(&format!("Compiling: {}", input_file));
        println!();
    }
    
    // 1. Read source file
    if !silent {
        print_step("Reading", input_file);
    }
    let source = match fs::read_to_string(input_file) {
        Ok(content) => {
            if !silent {
                print_step("", &format!("{}{}✓{} {} bytes", GREEN, BOLD, RESET, content.len()));
            }
            content
        }
        Err(err) => {
            if !silent {
                print_step("", &format!("{}{}✗{} {}", RED, BOLD, RESET, err));
            }
            process::exit(1);
        }
    };
    
    // 2. Lexical analysis
    if !silent {
        print_step("Lexical", "analyzing source...");
    }
    let tokens = tokenize(&source);
    if !silent {
        print_step("", &format!("{}{}✓{} {} tokens generated", GREEN, BOLD, RESET, tokens.len()));
    }
    
    // 3. Parsing
    if !silent {
        print_step("Parsing", "building AST...");
    }
    let mut parser = Parser::new(tokens.into_iter());
    let program = parser.parse_program();
    
    if !parser.errors.is_empty() {
        if !silent {
            print_step("", &format!("{}{}✗{} {} parsing error(s)", RED, BOLD, RESET, parser.errors.len()));
            println!();
        }
        for error in &parser.errors {
            if silent {
                eprintln!("error: {}", error);
            } else {
                eprintln!("  {}│{} {}", RED, RESET, error);
            }
        }
        process::exit(1);
    }
    
    if !silent {
        print_step("", &format!("{}{}✓{} AST built successfully", GREEN, BOLD, RESET));
    }
    
    // 4. Semantic analysis
    if !silent {
        print_step("Semantic", "checking quantum constraints...");
    }
    let mut checker = OwnershipChecker::new();
    match checker.check_program(&program) {
        Ok(_) => {
            if !silent {
                print_step("", &format!("{}{}✓{} All quantum constraints satisfied", GREEN, BOLD, RESET));
            }
        }
        Err(errors) => {
            if !silent {
                print_step("", &format!("{}{}✗{} {} semantic error(s)", RED, BOLD, RESET, errors.len()));
                println!();
            }
            for error in errors {
                if silent {
                    eprintln!("error: {}", error);
                } else {
                    eprintln!("  {}│{} {}", RED, RESET, error);
                }
            }
            process::exit(1);
        }
    }
    
    // 5. IR generation
    if !silent {
        print_step("IR", "generating intermediate representation...");
    }
    let mut ir_gen = IRGenerator::new();
    let ir_program = ir_gen.generate(&program);
    let qubit_count = if !ir_program.functions.is_empty() {
        ir_program.functions[0].qubits.len()
    } else {
        0
    };
    let cbit_count = if !ir_program.functions.is_empty() {
        ir_program.functions[0].cbits.len()
    } else {
        0
    };
    
    if !silent {
        print_step("", &format!("{}{}✓{} IR generated ({} qubits, {} cbits)", 
            GREEN, BOLD, RESET, qubit_count, cbit_count));
    }
    
    // 6. QASM generation
    if !silent {
        print_step("QASM", "generating OpenQASM 2.0 code...");
    }
    let qasm_gen = QASMGenerator::new();
    let qasm_code = qasm_gen.generate(&ir_program);
    let line_count = qasm_code.lines().count();
    
    if !silent {
        print_step("", &format!("{}{}✓{} {} lines generated", GREEN, BOLD, RESET, line_count));
    }
    
    // 7. Write output
    if !silent {
        print_step("Writing", &format!("to {}...", output_file));
    }
    match fs::write(&output_file, &qasm_code) {
        Ok(_) => {
            if !silent {
                print_step("", &format!("{}{}✓{} File written successfully", GREEN, BOLD, RESET));
            }
        }
        Err(err) => {
            if !silent {
                print_step("", &format!("{}{}✗{} {}", RED, BOLD, RESET, err));
            }
            process::exit(1);
        }
    }
    
    // Calculate gate count
    let gate_count = if let Some(func) = ir_program.functions.first() {
        func.operations.iter()
            .filter(|op| match op {
                qclang_compiler::ir::IROp::GateH(_)
                | qclang_compiler::ir::IROp::GateX(_)
                | qclang_compiler::ir::IROp::GateY(_)
                | qclang_compiler::ir::IROp::GateZ(_)
                | qclang_compiler::ir::IROp::GateCNOT(_, _) => true,
                _ => false
            })
            .count()
    } else {
        0
    };
    
    // Print summary (unless silent mode)
    if !silent {
        let elapsed = start_time.elapsed();
        println!();
        println!("{}Compilation Summary{}", BOLD, RESET);
        println!("{}───────────────────────{}", DIM, RESET);
        println!("  Input:        {}", input_file);
        println!("  Output:       {}", output_file);
        println!("  Qubits:       {}", qubit_count);
        println!("  Classical:    {}", cbit_count);
        println!("  Gates:        {}", gate_count);
        println!("  Time:         {:.2?}", elapsed);
        println!("  Status:       {}{}SUCCESS{}", GREEN, BOLD, RESET);
        
        // Show preview of generated QASM (verbose mode only)
        if verbose && line_count > 0 {
            println!();
            println!("{}Generated QASM Preview{}", BOLD, RESET);
            println!("{}───────────────────────{}", DIM, RESET);
            for (i, line) in qasm_code.lines().enumerate().take(15) {
                println!("  {:3} │ {}", i + 1, line);
            }
            if line_count > 15 {
                println!("  {}... {} more lines", DIM, line_count - 15);
            }
        }
    } else {
        // In silent mode, just output the filename
        println!("{}", output_file);
    }
    
    Ok(())
}