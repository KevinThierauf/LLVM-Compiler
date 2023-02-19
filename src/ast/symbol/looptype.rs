use label::Label;
use crate::ast::symbol::SymbolType;

pub mod r#loop;
pub mod whileloop;
pub mod forloop;
pub mod label;

pub trait LoopType: SymbolType {
    fn getLabel(&self) -> Option<&Label>;
}
