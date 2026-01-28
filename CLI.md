---

# QCLang Command Line Interface (CLI)

The `qclang` CLI is the primary tool for interacting with the QCLang compiler. It handles compilation, syntax checking, benchmarking, and testing of Quantum source files (`.qc`).

## Global Options

These options apply to all subcommands.

| Option | Short | Description |
| --- | --- | --- |
| `--verbose` | `-v` | Enable verbose logging (shows detailed compilation stats and paths). |
| `--no-color` |  | Disable colored terminal output (useful for CI/CD logs). |
| `--silent` |  | Suppress all output except errors (no ASCII art banners). |

---

## Commands

### 1. `compile`

Compiles QCLang source files (`.qc`) into OpenQASM 2.0 or other target formats. This is the primary command.

**Usage:**

```bash
qclang compile [OPTIONS] <INPUT_FILES>...

```

**Arguments:**

* `<INPUT_FILES>`: One or more `.qc` source files to compile.

**Options:**

* `-o, --output <PATH>`: Specify the output directory. If omitted, files are saved alongside the source.
* `-f, --format <FORMAT>`: Output format. Default is `qasm`.
* `qasm`: Standard OpenQASM 2.0 file.
* `json`: Metadata JSON containing stats (qubit count, gate depth, etc.).
* `both`: Generates both `.qasm` and `.json` files.
* `qir`: (Experimental) Quantum Intermediate Representation.


* `-s, --show`: Print the generated code to stdout after compilation.
* `--optimize`: Enable Phase 1.5 QIR optimizations (dead code elimination, gate cancellation).

**Example:**

```bash
# Compile multiple files with optimization and show output
qclang compile main.qc lib.qc -o ./build --optimize --show

```

### 2. `check`

Performs lexical analysis, parsing, and semantic verification without generating output code. Use this for quick syntax validation during development.

**Usage:**

```bash
qclang check [OPTIONS] <INPUT_FILES>...

```

**Options:**

* `--ast`: Print the Abstract Syntax Tree (AST) structure if the file parses successfully.

**Example:**

```bash
# Verify syntax and view the AST structure
qclang check circuit.qc --ast

```

### 3. `run`

Compiles a single file and displays detailed circuit statistics. Can also simulate execution (feature pending in v0.5.0).

**Usage:**

```bash
qclang run [OPTIONS] <INPUT_FILE>

```

**Options:**

* `--simulate`: Trigger the simulation backend (currently a placeholder for v0.5.0).

### 4. `test`

Runs the internal compiler test suite or specific test patterns. Useful for verifying compiler integrity.

**Usage:**

```bash
qclang test [OPTIONS]

```

**Options:**

* `-p, --pattern <STRING>`: Run only tests matching the given string pattern.
* `--report`: Generate a Markdown file (`test_report_<timestamp>.md`) containing the results.

### 5. `benchmark`

Runs performance benchmarks on the compiler itself, measuring compilation time across circuits of varying complexity (Small, Medium, Large).

**Usage:**

```bash
qclang benchmark [OPTIONS]

```

**Options:**

* `-i, --iterations <NUM>`: Number of iterations per circuit (default: 10).

**Output:**

```text
  Circuit         Avg (ms)     Min (ms)     Max (ms)    Samples
  ─────────────────────────────────────────────────────────────────
  Small (3q)           0.15         0.12         0.25        10
  Medium (8q)          0.32         0.28         0.45        10
  Large (15q)          0.85         0.75         1.10        10

```

### 6. `repl`

Starts the interactive Read-Eval-Print Loop (REPL). Allows entering QCLang code line-by-line for immediate feedback.

**Usage:**

```bash
qclang repl

```

**Commands inside REPL:**

* `code`: Enter QCLang statements (e.g., `qubit q = |0>;`).
* `version`: Show compiler version.
* `clear`: Clear screen.
* `quit` / `exit`: Exit the REPL.

### 7. `capabilities`

Displays the current feature set of the compiler, including supported gates, active optimization passes, and enabled language features.

**Usage:**

```bash
qclang capabilities

```

---

## Exit Codes

* `0`: Success.
* `1`: General error (IO, file not found).
* `2`: Compilation failed (Syntax or Semantic errors).