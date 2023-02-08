use crate::ast::ASTError;
use crate::ast::symbol::Symbol;

pub fn resolveConflict<'a>(options: impl Iterator<Item = &'a Symbol>) -> Result<usize, ASTError> {
    todo!()
}
