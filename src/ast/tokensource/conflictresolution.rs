use crate::ast::ASTError;
use crate::ast::symbol::SymbolType;

pub fn resolveConflict<'a>(options: impl Iterator<Item = &'a dyn SymbolType>) -> Result<usize, ASTError> {
    todo!()
}
