QCLang Language Reference
📖 Complete Syntax Specification
1. Program Structure
text
Program ::= Function*
Function ::= 'fn' Ident '(' Params? ')' '->' Type Block
Params ::= Param (',' Param)*
Param ::= Ident ':' Type
Type ::= 'int' | 'float' | 'bool' | 'string' 
        | 'qubit' | 'cbit' | 'qreg' '[' Int ']'
Block ::= '{' Statement* '}'
2. Statements
text
Statement ::= LetStmt | VarDeclStmt | AssignStmt 
            | ExprStmt | IfStmt | WhileStmt 
            | ReturnStmt | BlockStmt

LetStmt ::= 'let' Ident ':' Type '=' Expr ';'
VarDeclStmt ::= Type Ident ('=' Expr)? ';'
AssignStmt ::= Ident '=' Expr ';'
ExprStmt ::= Expr ';'
ReturnStmt ::= 'return' Expr? ';'
IfStmt ::= 'if' '(' Expr ')' Statement ('else' Statement)?
WhileStmt ::= 'while' '(' Expr ')' Statement
BlockStmt ::= '{' Statement* '}'
3. Expressions
text
Expr ::= AssignmentExpr
AssignmentExpr ::= OrExpr ('=' AssignmentExpr)?
OrExpr ::= AndExpr ('|' AndExpr)*
AndExpr ::= EqualityExpr ('&' EqualityExpr)*
EqualityExpr ::= RelationalExpr (('==' | '!=') RelationalExpr)*
RelationalExpr ::= AdditiveExpr (('<' | '>' | '<=' | '>=') AdditiveExpr)*
AdditiveExpr ::= MultiplicativeExpr (('+' | '-') MultiplicativeExpr)*
MultiplicativeExpr ::= UnaryExpr (('*' | '/') UnaryExpr)*
UnaryExpr ::= ('-' | '!') UnaryExpr | PrimaryExpr
PrimaryExpr ::= Literal | Ident | '(' Expr ')' | CallExpr | GateExpr | MeasureExpr

Literal ::= IntLiteral | FloatLiteral | BoolLiteral 
          | StringLiteral | QubitLiteral
QubitLiteral ::= '|' ('0' | '1') '>'
CallExpr ::= Ident '(' Args? ')'
GateExpr ::= GateName '(' Args ')'
MeasureExpr ::= 'measure' '(' Expr ')'
Args ::= Expr (',' Expr)*
4. Built-in Gates
text
GateName ::= 'H' | 'X' | 'Y' | 'Z' | 'CNOT'
5. Type System Semantics
Qubit Affine Types
Linear Usage: Qubits must be used exactly once in their scope

No Cloning: Cannot copy or duplicate qubit values

Consumption: Measurement consumes the qubit

Return Semantics: Functions can return qubits (transfers ownership)

Type Aliases
rust
// Built-in types
type Qubit = affine qubit;      // Single quantum bit
type CBit = cbit;               // Classical bit
type QReg = qreg[Size];         // Quantum register
type Int = i64;                 // 64-bit integer
type Float = f64;               // 64-bit float
type Bool = bool;               // Boolean
type String = str;              // String
6. Memory Model
Qubit Lifetime
rust
fn example() -> int {
    qubit q = |0>;          // [1] Allocation
    q = H(q);               // [2] Use (consumes q, produces new q)
    cbit r = measure(q);    // [3] Consumption (q is destroyed)
    return 0;               // [4] No dangling qubits
}
Invalid Patterns (Compile Errors)
rust
// ERROR: Double measurement
qubit q = |0>;
cbit r1 = measure(q);
cbit r2 = measure(q);  // ERROR: q already consumed

// ERROR: Unconsumed qubit
qubit q = |0>;
q = H(q);
// ERROR: q not measured or returned

// ERROR: Use after measurement
qubit q = |0>;
cbit r = measure(q);
q = X(q);  // ERROR: q already consumed
7. Standard Library (Planned)
rust
// Quantum algorithms
fn grover_search(qubit[] register, Oracle oracle) -> int;
fn qft(qreg q) -> qreg;
fn shor_factor(n: int) -> (int, int);

// Utility functions
fn random_qubit() -> qubit;
fn bell_state() -> (qubit, qubit);
fn swap(qubit a, qubit b) -> (qubit, qubit);
8. Compiler Directives (Future)
rust
// Target hardware specification
#target ibm_cairo
#optimize level=3
#shots 1024

// Error mitigation
#error_mitigation readout
#calibration auto