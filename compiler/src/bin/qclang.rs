// src/bin/qclang.rs - QUANTUM CAT VERSION
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use qclang_compiler::{Compiler, CompileStats};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Parser)]
#[command(name = "qclang")]
#[command(author = "QCLang Team")]
#[command(version = "0.4.1")]
#[command(about = "Quantum Computation Language Compiler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(long, global = true, help = "Disable colored output")]
    no_color: bool,
    
    #[arg(short, long, global = true, help = "Verbose output")]
    verbose: bool,
    
    #[arg(long, global = true, help = "Silent mode (no banners, minimal output)")]
    silent: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile QCLang source files to OpenQASM
    #[command(arg_required_else_help = true)]
    Compile {
        /// Input QCLang files
        #[arg(required = true, num_args = 1..)]
        input: Vec<PathBuf>,
        
        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Output format
        #[arg(short = 'f', long, default_value = "qasm")]
        format: OutputFormat,
        
        /// Show generated code
        #[arg(short, long)]
        show: bool,
        
        /// Optimize output
        #[arg(short, long)]
        optimize: bool,
    },
    
    /// Compile and show detailed statistics
    #[command(arg_required_else_help = true)]
    Run {
        /// Input QCLang file
        input: PathBuf,
        
        /// Simulate execution
        #[arg(long)]
        simulate: bool,
    },
    
    /// Run the test suite
    Test {
        /// Run specific test pattern
        #[arg(short, long)]
        pattern: Option<String>,
        
        /// Generate test report
        #[arg(long)]
        report: bool,
    },
    
    /// Show compiler capabilities
    Capabilities,
    
    /// Validate syntax without compilation
    Check {
        /// Input QCLang files
        #[arg(required = true, num_args = 1..)]
        input: Vec<PathBuf>,
        
        /// Show AST
        #[arg(long)]
        ast: bool,
    },
    
    /// Show compiler version and info
    Version,
    
    /// Benchmark compiler performance
    Benchmark {
        /// Number of iterations
        #[arg(short, long, default_value_t = 10)]
        iterations: usize,
    },
    
    /// Interactive REPL mode
    Repl,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Qasm,
    Json,
    Both,
    Qir,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Initialize colored output
    if cli.no_color {
        colored::control::set_override(false);
    }
    
    if !cli.silent && !matches!(cli.command, Commands::Version) {
        print_banner();
    }
    
    match cli.command {
        Commands::Compile { input, output, format, show, optimize } => {
            compile_files(input, output.as_deref(), format, show, optimize, cli.verbose)?;
        }
        Commands::Run { input, simulate } => {
            run_file(&input, simulate, cli.verbose)?;
        }
        Commands::Test { pattern, report } => {
            run_tests(pattern, report, cli.verbose)?;
        }
        Commands::Capabilities => {
            show_capabilities();
        }
        Commands::Check { input, ast } => {
            check_files(&input, ast, cli.verbose)?;
        }
        Commands::Version => {
            show_version(cli.verbose);
        }
        Commands::Benchmark { iterations } => {
            run_benchmark(iterations)?;
        }
        Commands::Repl => {
            start_repl()?;
        }
    }
    
    Ok(())
}

fn print_banner() {
    let cat_art = r#"
    ╔══════════════════════════════════════════════════════════════════╗
    ║                                                                  ║
    ║                /\_/\                                             ║
    ║               ( o.o )     ░░░░░░  ░░░░░░  ░░░░░░  ░░░░░░         ║
    ║                > ^ <      ░░      ░░  ░░  ░░      ░░  ░░         ║
    ║                           ░░░░░░  ░░░░░░  ░░░░░░  ░░░░░░         ║
    ║                               ░░  ░░  ░░      ░░  ░░  ░░         ║
    ║                           ░░░░░░  ░░  ░░  ░░░░░░  ░░  ░░         ║
    ║                                                                  ║
    ║                     Q U A N T U M   C A T                        ║
    ║                    Schrödinger's Companion                       ║
    ║                                                                  ║
    ║                  QCLang Compiler v0.4.1                          ║
    ║          A quantum systems programming language                  ║
    ║                                                                  ║
    ╚══════════════════════════════════════════════════════════════════╝
    "#;
    
    println!("{}", cat_art.cyan().bold());
}

fn compile_files(
    inputs: Vec<PathBuf>,
    output_dir: Option<&Path>,
    format: OutputFormat,
    show: bool,
    _optimize: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !verbose {
        println!("{} Starting compilation...", "🔧".cyan());
    }
    
    let total_files = inputs.len();
    let mut success_count = 0;
    let mut total_qubits = 0;
    let mut total_gates = 0;
    let mut total_measurements = 0;
    let mut total_time = Duration::default();
    
    // Create multi-progress bar
    let multi = MultiProgress::new();
    let main_pb = multi.add(ProgressBar::new(total_files as u64));
    main_pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} files | {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    main_pb.set_message("Compiling...");
    
    for (i, input_path) in inputs.iter().enumerate() {
        let file_pb = multi.add(ProgressBar::new(100));
        file_pb.set_style(
            ProgressStyle::with_template("  {spinner:.dim} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        file_pb.set_message(format!("Processing {}", input_path.display()));
        file_pb.enable_steady_tick(Duration::from_millis(100));
        
        let source = match fs::read_to_string(input_path) {
            Ok(source) => {
                file_pb.set_message(format!("Parsing {}...", input_path.display()));
                source
            }
            Err(e) => {
                file_pb.finish_and_clear();
                println!("{} Failed to read {}: {}", "✗".red(), input_path.display(), e);
                continue;
            }
        };
        
        let start_time = Instant::now();
        let result = Compiler::compile_with_stats(&source);
        let elapsed = start_time.elapsed();
        total_time += elapsed;
        
        file_pb.finish_and_clear();
        
        match result {
            Ok((qasm, stats)) => {
                success_count += 1;
                total_qubits += stats.qubits;
                total_gates += stats.gates;
                total_measurements += stats.measurements;
                
                // Determine output path
                let output_path = if let Some(dir) = output_dir {
                    let file_name = input_path.file_stem()
                        .unwrap_or_default()
                        .to_string_lossy();
                    dir.join(format!("{}.qasm", file_name))
                } else {
                    input_path.with_extension("qasm")
                };
                
                // Save output based on format
                match format {
                    OutputFormat::Qasm => {
                        fs::write(&output_path, &qasm)?;
                        if verbose {
                            println!("{} Wrote {}", "✓".green(), output_path.display());
                        }
                    }
                    OutputFormat::Json => {
                        let json_path = output_path.with_extension("json");
                        let json_data = serde_json::json!({
                            "qubits": stats.qubits,
                            "cbits": stats.cbits,
                            "gates": stats.gates,
                            "measurements": stats.measurements,
                            "qasm": qasm,
                            "compilation_time_ms": elapsed.as_millis(),
                        });
                        fs::write(json_path, serde_json::to_string_pretty(&json_data)?)?;
                    }
                    OutputFormat::Both => {
                        fs::write(&output_path, &qasm)?;
                        let json_path = output_path.with_extension("json");
                        let json_data = serde_json::json!({
                            "qubits": stats.qubits,
                            "cbits": stats.cbits,
                            "gates": stats.gates,
                            "measurements": stats.measurements,
                            "compilation_time_ms": elapsed.as_millis(),
                        });
                        fs::write(json_path, serde_json::to_string_pretty(&json_data)?)?;
                    }
                    OutputFormat::Qir => {
                        println!("{} QIR output not yet implemented", "⚠".yellow());
                        fs::write(&output_path, &qasm)?;
                    }
                }
                
                if verbose {
                    print_file_stats(&stats, elapsed);
                }
                
                // Show generated code if requested
                if show {
                    show_generated_code(&qasm, "OpenQASM 2.0");
                }
            }
            Err(errors) => {
                println!("{} {} failed:", "✗".red(), input_path.display());
                print_errors(&errors);
            }
        }
        
        main_pb.inc(1);
    }
    
    main_pb.finish_and_clear();
    multi.clear()?;
    
    print_summary(success_count, total_files, total_qubits, total_gates, total_measurements, total_time);
    
    Ok(())
}

fn print_file_stats(stats: &CompileStats, elapsed: Duration) {
    println!("  {} Qubits: {}", "⚛".blue(), stats.qubits);
    println!("  {} Gates: {}", "🔧".blue(), stats.gates);
    println!("  {} Measurements: {}", "📏".blue(), stats.measurements);
    println!("  {} Depth: {}", "📊".blue(), stats.gates.saturating_sub(1).max(1));
    println!("  {} Time: {:.2}ms", "⏱".blue(), elapsed.as_secs_f64() * 1000.0);
    println!();
}

fn print_errors(errors: &[String]) {
    let border = "─".repeat(50);
    println!("  {}", border.dimmed());
    for error in errors {
        let lines: Vec<&str> = error.split('\n').collect();
        for line in lines {
            if line.contains("error:") {
                println!("  {} {}", "✗".red(), line.red());
            } else if line.contains("warning:") {
                println!("  {} {}", "⚠".yellow(), line.yellow());
            } else if line.contains("note:") {
                println!("  {} {}", "ℹ".cyan(), line.cyan());
            } else {
                println!("  {}", line);
            }
        }
    }
    println!("  {}", border.dimmed());
}

fn print_summary(success: usize, total: usize, qubits: usize, gates: usize, measurements: usize, time: Duration) {
    let border = "─".repeat(50);
    println!("\n{}", border.dimmed());
    println!("{} COMPILATION SUMMARY", "📊".cyan());
    println!("{}", border.dimmed());
    
    println!("  {} Files:      {}/{} successful", "📁".blue(), success, total);
    println!("  {} Qubits:     {}", "⚛".blue(), qubits);
    println!("  {} Gates:      {}", "🔧".blue(), gates);
    println!("  {} Meas:       {}", "📏".blue(), measurements);
    println!("  {} Total time: {:.2}ms", "⏱".blue(), time.as_secs_f64() * 1000.0);
    
    if success == total {
        println!("\n{} All files compiled successfully!", "🎉".green());
    } else {
        println!("\n{} {} files failed", "⚠".yellow(), total - success);
    }
}

fn run_file(
    input_path: &Path,
    simulate: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Running cat ASCII art
    println!("{}", running_cat_art().cyan());
    println!("{} Running: {}", "🚀".green(), input_path.display());
    
    if !input_path.exists() {
        println!("{} File not found: {}", "✗".red(), input_path.display());
        return Ok(());
    }
    
    let source = fs::read_to_string(input_path)?;
    let start_time = Instant::now();
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["◐", "◓", "◑", "◒"]),
    );
    pb.set_message("Compiling quantum circuit...");
    pb.enable_steady_tick(Duration::from_millis(100));
    
    let result = Compiler::compile_with_stats(&source);
    pb.finish_and_clear();
    
    let elapsed = start_time.elapsed();
    
    match result {
        Ok((qasm, stats)) => {
            println!("{} Compilation successful!", "✓".green());
            println!();
            
            // Show circuit diagram
            print_circuit_diagram(&stats);
            
            // Show detailed statistics
            print_detailed_stats(&stats, elapsed);
            
            // Save QASM
            let output_path = input_path.with_extension("qasm");
            fs::write(&output_path, &qasm)?;
            println!("{} Output saved to: {}", "💾".blue(), output_path.display());
            
            if simulate {
                simulate_circuit(&qasm)?;
            }
            
            if verbose {
                println!("\n{} Generated circuit:", "📊".cyan());
                show_generated_code(&qasm, "OpenQASM 2.0");
            }
        }
        Err(errors) => {
            println!("{} Compilation failed:", "✗".red());
            print_errors(&errors);
        }
    }
    
    Ok(())
}

fn running_cat_art() -> &'static str {
    r#"
        /\_/\
       ( •.• )  Running quantum circuit...
        > ^ <   |ψ⟩ = α|0⟩ + β|1⟩
       /  |  \
      /   |   \
    "#
}

fn print_circuit_diagram(stats: &CompileStats) {
    let width = 40;
    let qubit_line = "─".repeat(width);
    
    println!("{} Circuit Diagram:", "📐".cyan());
    println!("  {}", "┌".to_string() + &"─".repeat(width) + "┐");
    
    for i in 0..stats.qubits.min(5) {
        let line = format!("q{} ┤{}├", i, qubit_line);
        println!("  {}", line);
        if i < stats.qubits.min(5) - 1 {
            println!("  {}", "   │".to_string() + &" ".repeat(width) + "│");
        }
    }
    
    println!("  {}", "└".to_string() + &"─".repeat(width) + "┘");
    println!("  {} gates applied", stats.gates);
    println!();
}

fn print_detailed_stats(stats: &CompileStats, elapsed: Duration) {
    let border = "─".repeat(40);
    println!("{}", border.dimmed());
    println!("{} Statistics:", "📈".cyan());
    println!("{}", border.dimmed());
    
    println!("  {:15} : {}", "Qubits".blue(), stats.qubits);
    println!("  {:15} : {}", "Classical bits".blue(), stats.cbits);
    println!("  {:15} : {}", "Quantum gates".blue(), stats.gates);
    println!("  {:15} : {}", "Measurements".blue(), stats.measurements);
    println!("  {:15} : {}", "Circuit depth".blue(), stats.gates.saturating_sub(1).max(1));
    println!("  {:15} : {:.2} ms", "Compile time".blue(), elapsed.as_secs_f64() * 1000.0);
    let ops_per_sec = if elapsed.as_secs_f64() > 0.0 {
        stats.total_operations() as f64 / elapsed.as_secs_f64()
    } else {
        0.0
    };
    println!("  {:15} : {:.0}", "Operations/sec".blue(), ops_per_sec);
    println!("{}", border.dimmed());
}

fn simulate_circuit(_qasm: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{} Simulation mode not yet implemented", "⚠".yellow());
    println!("  {} Coming in v0.5.0!", "🚀".green());
    Ok(())
}

fn run_tests(
    pattern: Option<String>,
    report: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} Running QCLang Test Suite", "🧪".blue());
    
    let tests = vec![
        ("basic_circuit", "Basic quantum circuit", r#"
fn main() -> int {
    qubit q = |0>;
    q = H(q);
    cbit result = measure(q);
    return 0;
}
"#, (1, 1, 1)),
        ("bell_state", "Bell state", r#"
fn main() -> int {
    qubit a = |0>;
    qubit b = |0>;
    a = H(a);
    b = CNOT(a, b);
    cbit a_res = measure(a);
    cbit b_res = measure(b);
    return 0;
}
"#, (2, 2, 2)),
        ("loop_qubits", "Loop with 3 qubits", r#"
fn main() -> int {
    for i in range(0, 3) {
        qubit q = |0>;
        q = H(q);
        cbit result = measure(q);
    }
    return 0;
}
"#, (3, 3, 3)),
        ("multi_gate", "Multiple gate types", r#"
fn main() -> int {
    qubit q1 = |0>;
    qubit q2 = |0>;
    q1 = H(q1);
    q2 = X(q2);
    q1 = T(q1);
    q2 = S(q2);
    q1 = CNOT(q1, q2);
    cbit m1 = measure(q1);
    cbit m2 = measure(q2);
    return 0;
}
"#, (2, 5, 2)),
    ];
    
    let filtered_tests: Vec<_> = if let Some(pat) = &pattern {
        tests.into_iter()
            .filter(|(id, name, _, _)| id.contains(pat) || name.contains(pat))
            .collect()
    } else {
        tests
    };
    
    let mut passed = 0;
    let mut failed = 0;
    let mut test_results: Vec<(&str, &str, bool, Duration, Option<String>)> = Vec::new();
    
    let pb = ProgressBar::new(filtered_tests.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} tests")
            .unwrap()
            .progress_chars("█▓▒░"),
    );
    
    for (id, name, source, expected) in &filtered_tests {
        pb.set_message(format!("Testing: {}", name));
        
        let start_time = Instant::now();
        let result = Compiler::compile_with_stats(source);
        let elapsed = start_time.elapsed();
        
        match result {
            Ok((_, stats)) => {
                let (exp_qubits, exp_gates, exp_measurements) = expected;
                
                if stats.qubits == *exp_qubits && 
                   stats.gates == *exp_gates && 
                   stats.measurements == *exp_measurements {
                    passed += 1;
                    test_results.push((id, name, true, elapsed, None));
                    if verbose {
                        println!("{} {} ... PASS ({:.2}ms)", "✓".green(), name, elapsed.as_secs_f64() * 1000.0);
                    }
                } else {
                    failed += 1;
                    let error = format!("Expected: {}q/{}g/{}m, Got: {}q/{}g/{}m", 
                        exp_qubits, exp_gates, exp_measurements,
                        stats.qubits, stats.gates, stats.measurements);
                    test_results.push((id, name, false, elapsed, Some(error)));
                    println!("{} {} ... FAIL", "✗".red(), name);
                }
            }
            Err(errors) => {
                failed += 1;
                let error = errors.get(0).cloned().unwrap_or_else(|| "Unknown error".to_string());
                test_results.push((id, name, false, elapsed, Some(error)));
                println!("{} {} ... ERROR", "✗".red(), name);
            }
        }
        
        pb.inc(1);
    }
    
    pb.finish_and_clear();
    
    print_test_summary(passed, failed, &test_results);
    
    if report {
        generate_test_report(&test_results)?;
    }
    
    Ok(())
}

fn print_test_summary(passed: usize, failed: usize, results: &[(&str, &str, bool, Duration, Option<String>)]) {
    let total = passed + failed;
    let percentage = if total > 0 {
        (passed as f64 / total as f64 * 100.0) as usize
    } else {
        0
    };
    
    println!("\n{}", "─".repeat(50).dimmed());
    println!("{} TEST RESULTS", "📊".cyan());
    println!("{}", "─".repeat(50).dimmed());
    
    // Progress bar
    let bar_width = 40;
    let filled = if total > 0 {
        (passed * bar_width) / total
    } else {
        0
    };
    let bar = "█".repeat(filled) + &"░".repeat(bar_width - filled);
    println!("  [{}] {}%", bar.cyan(), percentage);
    println!();
    
    println!("  {} Total:     {}", "📁".blue(), total);
    println!("  {} Passed:    {}", "✓".green(), passed);
    println!("  {} Failed:    {}", "✗".red(), failed);
    
    if failed > 0 {
        println!("\n{} Failed tests:", "⚠".yellow());
        for (_, name, _, _, error) in results.iter().filter(|(_, _, s, _, _)| !*s) {
            if let Some(err) = error {
                println!("  • {}: {}", name.dimmed(), err.red());
            } else {
                println!("  • {}", name.dimmed());
            }
        }
    }
    
    if passed == total {
        println!("\n{} All tests passed!", "🎉".green());
    }
}

fn generate_test_report(results: &[(&str, &str, bool, Duration, Option<String>)]) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let report_path = format!("test_report_{}.md", timestamp);
    
    let mut report = String::new();
    report.push_str("# QCLang Test Report\n\n");
    report.push_str(&format!("Generated: {}\n\n", timestamp));
    
    let passed = results.iter().filter(|(_, _, s, _, _)| *s).count();
    let total = results.len();
    
    report.push_str("## Summary\n\n");
    report.push_str(&format!("- **Total Tests**: {}\n", total));
    report.push_str(&format!("- **Passed**: {} ({:.1}%)\n", passed, (passed as f64 / total as f64 * 100.0)));
    report.push_str(&format!("- **Failed**: {}\n\n", total - passed));
    
    report.push_str("## Details\n\n");
    report.push_str("| Test | Status | Time (ms) | Notes |\n");
    report.push_str("|------|--------|-----------|-------|\n");
    
    for (id, name, success, duration, error) in results {
        let status = if *success { "✅ PASS" } else { "❌ FAIL" };
        let time = duration.as_secs_f64() * 1000.0;
        let notes = error.as_ref().map_or("", String::as_str);
        report.push_str(&format!("| `{}` | {} | {:.2} | {} |\n", id, status, time, notes));
    }
    
    fs::write(&report_path, report)?;
    println!("{} Report saved to: {}", "📄".green(), report_path);
    
    Ok(())
}

fn show_capabilities() {
    println!("{} QCLang Compiler Capabilities", "🔧".cyan());
    println!("{}", "─".repeat(50).dimmed());
    
    let capabilities = Compiler::capabilities();
    
    println!("{} Language Features:", "📚".blue());
    for (i, cap) in capabilities.iter().enumerate() {
        println!("  {:2}. {}", i + 1, cap);
    }
    
    println!("\n{} Supported Gates:", "⚡".yellow());
    let gates = ["H", "X", "Y", "Z", "S", "T", "CNOT", "CZ", "SWAP", "RX", "RY", "RZ"];
    for gate in gates.chunks(4) {
        println!("  {}", gate.join("  "));
    }
    
    println!("\n{} Target Formats:", "🎯".green());
    println!("  • OpenQASM 2.0");
    println!("  • JSON Metadata");
    println!("  • QIR (planned)");
    
    println!("\n{} Version:      {}", "ℹ".blue(), Compiler::version());
    println!("{} Build:        {}", "🔨".blue(), qclang_compiler::build_timestamp());
    println!("{} License:      MIT/Apache-2.0", "⚖️".blue());
}

fn check_files(
    inputs: &[PathBuf],
    show_ast: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} Syntax Checking", "🔍".yellow());
    
    let mut errors_count = 0;
    let mut files_count = 0;
    
    for input_path in inputs {
        files_count += 1;
        println!("\n{} {}", "📄".blue(), input_path.display());
        
        let source = match fs::read_to_string(input_path) {
            Ok(source) => source,
            Err(e) => {
                println!("{} Failed to read: {}", "✗".red(), e);
                errors_count += 1;
                continue;
            }
        };
        
        // Lexical analysis
        let tokens = qclang_compiler::lexer::tokenize(&source);
        if verbose {
            println!("  {} Tokens: {}", "✓".green(), tokens.len());
        }
        
        // Parse
        let mut parser = qclang_compiler::parser::Parser::new(tokens.into_iter(), source.clone());
        let program = parser.parse_program();
        
        if parser.errors.is_empty() {
            println!("  {} Syntax OK", "✓".green());
            
            // Semantic analysis
            let mut checker = qclang_compiler::semantics::OwnershipChecker::new(&source);
            match checker.check_program(&program) {
                Ok(_) => println!("  {} Semantics OK", "✓".green()),
                Err(semantic_errors) => {
                    println!("  {} Semantic errors:", "⚠".yellow());
                    errors_count += semantic_errors.len();
                    for error in semantic_errors {
                        println!("    • {}", error);
                    }
                }
            }
            
            if show_ast {
                println!("\n  {} Abstract Syntax Tree:", "🌳".green());
                println!("    {}", "└─ Program".dimmed());
            }
        } else {
            println!("  {} Syntax errors:", "✗".red());
            errors_count += parser.errors.len();
            for error in &parser.errors {
                println!("    • {}", error);
            }
        }
    }
    
    println!("\n{}", "─".repeat(50).dimmed());
    println!("Checked {} files, found {} errors", files_count, errors_count);
    
    if errors_count == 0 {
        println!("{} All files are valid QCLang", "✓".green());
    }
    
    Ok(())
}

fn show_version(verbose: bool) {
    let version = Compiler::version();
    
    // Quantum Cat ASCII art for version
    let version_art = r#"
     ╔═══════════════════════════════════════╗
     ║  /\_/\   QCLang Quantum Compiler      ║
     ║ ( o.o )  Version: {:<15}              ║
     ║  > ^ <   Schrödinger's Companion      ║
     ╚═══════════════════════════════════════╝
    "#;
    
    println!("{}", version_art.replace("{:<15}", &format!("v{}", version)).cyan());
    
    if verbose {
        println!("{} Build:   {}", "🔨".blue(), qclang_compiler::build_timestamp());
        println!("{} Git:     {}", "🐙".blue(), qclang_compiler::git_commit_hash());
        println!("{} Target:  {}", "🎯".blue(), "OpenQASM 2.0");
        println!("{} License: {}", "⚖️".blue(), "MIT/Apache-2.0");
    }
}

fn run_benchmark(iterations: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} Running Compiler Benchmark", "🏃".cyan());
    
    let benchmark_circuits = vec![
        ("Small (3q)", r#"
fn main() -> int {
    qubit q1 = |0>;
    qubit q2 = |0>;
    qubit q3 = |0>;
    q1 = H(q1);
    q2 = CNOT(q1, q2);
    q3 = CNOT(q2, q3);
    cbit m1 = measure(q1);
    cbit m2 = measure(q2);
    cbit m3 = measure(q3);
    return 0;
}
"#),
        ("Medium (8q)", r#"
fn main() -> int {
    for i in range(0, 8) {
        qubit q = |0>;
        q = H(q);
        if i % 2 == 0 {
            q = X(q);
        }
        cbit r = measure(q);
    }
    return 0;
}
"#),
        ("Large (15q)", r#"
fn main() -> int {
    qubit[15] qs = |0>;
    for i in range(0, 15) {
        qs[i] = H(qs[i]);
    }
    for i in range(0, 14) {
        qs[i] = CNOT(qs[i], qs[i+1]);
    }
    cbit[15] results = measure(qs);
    return 0;
}
"#),
    ];
    
    let mut results = Vec::new();
    
    for (name, source) in benchmark_circuits {
        println!("\n{} Benchmarking: {}", "⏱".blue(), name);
        
        let mut times = Vec::new();
        let pb = ProgressBar::new(iterations as u64);
        
        for i in 0..iterations {
            pb.set_message(format!("Iteration {}/{}", i + 1, iterations));
            let start = Instant::now();
            let _ = Compiler::compile_with_stats(source);
            times.push(start.elapsed());
            pb.inc(1);
        }
        
        pb.finish_and_clear();
        
        let avg_time = if !times.is_empty() {
            times.iter().sum::<Duration>() / times.len() as u32
        } else {
            Duration::ZERO
        };
        let min_time = times.iter().min().copied().unwrap_or(Duration::ZERO);
        let max_time = times.iter().max().copied().unwrap_or(Duration::ZERO);
        
        results.push((name, avg_time, min_time, max_time, times.len()));
        
        println!("  {} Avg: {:.2}ms", "📊".green(), avg_time.as_secs_f64() * 1000.0);
        println!("  {} Min: {:.2}ms", "⚡".blue(), min_time.as_secs_f64() * 1000.0);
        println!("  {} Max: {:.2}ms", "🐢".yellow(), max_time.as_secs_f64() * 1000.0);
    }
    
    print_benchmark_summary(&results);
    Ok(())
}

fn print_benchmark_summary(results: &[(&str, Duration, Duration, Duration, usize)]) {
    println!("\n{}", "═".repeat(60).cyan());
    println!("{} BENCHMARK SUMMARY", "📈".cyan());
    println!("{}", "═".repeat(60).cyan());
    
    println!("\n{:<15} {:<12} {:<12} {:<12} {:<10}", 
        "Circuit", "Avg (ms)", "Min (ms)", "Max (ms)", "Samples");
    println!("{}", "─".repeat(65));
    
    for (name, avg, min, max, samples) in results {
        println!("{:<15} {:>11.2} {:>11.2} {:>11.2} {:>10}", 
            name, 
            avg.as_secs_f64() * 1000.0,
            min.as_secs_f64() * 1000.0,
            max.as_secs_f64() * 1000.0,
            samples);
    }
}

fn start_repl() -> Result<(), Box<dyn std::error::Error>> {
    // REPL ASCII art
    println!("{}", r#"
     ╔═══════════════════════════════════════╗
     ║  /\_/\   QCLang REPL                  ║
     ║ ( •.• )  Interactive Mode             ║
     ║  > ^ <   Type quantum code below!     ║
     ╚═══════════════════════════════════════╝
    "#.cyan());
    
    println!("{} Type 'quit' or 'exit' to exit", "ℹ".blue());
    println!("{} Type 'help' for available commands", "❓".blue());
    println!();
    
    // Simple REPL without external dependencies
    loop {
        print!("{} ", "qclang>".cyan().bold());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        match input {
            "quit" | "exit" => {
                println!("{} Goodbye!", "👋".green());
                break;
            }
            "help" => {
                print_repl_help();
            }
            "clear" => {
                // Simple clear by printing newlines
                print!("{}[2J", 27 as char);
                print!("{}[1;1H", 27 as char);
            }
            "version" => {
                show_version(false);
            }
            _ => {
                // Try to compile the input
                if input.starts_with("fn") || input.contains("qubit") {
                    match Compiler::compile_with_stats(input) {
                        Ok((qasm, stats)) => {
                            println!("{} Compiled successfully!", "✓".green());
                            println!("  Qubits: {}, Gates: {}", stats.qubits, stats.gates);
                            println!("{} Output:", "📋".blue());
                            println!("{}", qasm);
                        }
                        Err(errors) => {
                            println!("{} Compilation errors:", "✗".red());
                            for error in &errors {
                                println!("  • {}", error);
                            }
                        }
                    }
                } else {
                    println!("{} Not a valid QCLang expression", "⚠".yellow());
                    println!("  Try starting with 'qubit q = |0>;' or 'fn main() -> int {{ ... }}'");
                }
            }
        }
    }
    
    Ok(())
}

fn print_repl_help() {
    println!("\n{} Available commands:", "📚".cyan());
    println!("  {} ... enter QCLang code", "code".blue());
    println!("  {} ............ show this help", "help".blue());
    println!("  {} ............. show version", "version".blue());
    println!("  {} ............. clear screen", "clear".blue());
    println!("  {} ............. exit REPL", "quit/exit".blue());
    println!("\n{} Examples:", "💡".yellow());
    println!("  qubit q = |0>;");
    println!("  q = H(q);");
    println!("  cbit r = measure(q);");
    println!();
}

fn show_generated_code(code: &str, label: &str) {
    let border = "─".repeat(60);
    println!("\n{} {}:", "📋".cyan(), label);
    println!("{}", border.dimmed());
    
    for (i, line) in code.lines().enumerate().take(25) {
        let line_num = format!("{:3} │", i + 1);
        println!("{} {}", line_num.dimmed(), line);
    }
    
    let total_lines = code.lines().count();
    if total_lines > 25 {
        println!("{} ... {} more lines", "⋮".dimmed(), total_lines - 25);
    }
    println!("{}", border.dimmed());
}