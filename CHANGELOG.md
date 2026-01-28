# Changelog

All notable changes to the **QCLang** project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.1] - 2025-01-28

### Added
- **Phase 1.5 Pipeline**: Introduced Quantum Intermediate Representation (QIR) generation step.
- **Optimization Pass**: Added basic QIR optimizations (gate cancellation, constant folding).
- **Quantum Loop Syntax**: Support for `qfor` and `qif` constructs in the parser.
- **CLI Improvements**: Added `--show` flag to display compilation phases and final OpenQASM output.
- **ASCII Art**: "Quantum Cat" logo added to compiler startup sequence.

### Changed
- **Semantic Analysis**: Stricter enforcement of Affine Typing.
  - *Fix*: Reassigning a `qubit` variable now raises a hard semantic error.
  - *Fix*: Using a qubit after `measure()` is now correctly blocked.
- **Parser**: Updated gate application syntax to forbid assignment (e.g., `q = H(q)` is now invalid; use `H(q)`).

### Fixed
- Fixed semantic error hints for immutable variable assignment.
- Resolved issue where `measure()` return type was not correctly inferred as `cbit`.

## [0.3.0] - Previous Release
- Initial Parser and Lexer implementation.
- Basic AST structure.
- OpenQASM 2.0 code generation backbone.