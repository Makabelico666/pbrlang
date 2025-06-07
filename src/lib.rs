pub mod ast;
pub mod lexer;
pub mod parser;
pub mod transpiler;
pub mod caixotes;

// Re-export commonly used items
pub use ast::{Declaracao, Expressao, Operador, Programa, Tipo};
pub use parser::{analisar_codigo, PBRParser};
pub use lexer::tokenizar;
pub use transpiler::gerar_codigo_rust;

