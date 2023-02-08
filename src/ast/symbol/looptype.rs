use label::Label;

pub mod r#loop;
pub mod whileloop;
pub mod forloop;
pub mod label;

pub trait LoopType {
    fn getLabel(&self) -> Option<&Label>;
}
