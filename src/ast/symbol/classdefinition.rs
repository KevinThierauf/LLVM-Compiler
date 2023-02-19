use crate::module::modulepos::{ModulePos, ModuleRange};
use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::function::FunctionDefinitionSym;
use crate::ast::symbol::SymbolType;
use crate::module::visibility::Visibility;

#[derive(Debug)]
pub struct ClassFieldDefinition {
    pub name: ModulePos,
    pub visibility: Visibility,
    // typeName or defaultValue (or both) MUST be Some
    pub typeName: Option<ModulePos>,
    pub defaultValue: Option<Expr>,
}

#[derive(Debug)]
pub struct ClassStaticFieldDefinition {
    pub name: ModulePos,
    pub typeName: Option<ModulePos>,
    pub visibility: Visibility,
    pub defaultValue: Option<Expr>,
}

pub enum ClassMember {
    FieldDefinition(ClassFieldDefinition),
    FunctionDefinition(FunctionDefinitionSym),
    StaticFieldDefinition(ClassStaticFieldDefinition),
}

#[derive(Debug)]
pub struct ClassDefinitionSym {
    pub visibility: Visibility,
    pub range: ModuleRange,
    pub name: ModulePos,
    pub fields: Vec<ClassFieldDefinition>,
    pub staticFields: Vec<ClassStaticFieldDefinition>,
    pub methods: Vec<FunctionDefinitionSym>,
    pub inherited: Vec<ModulePos>,
}

impl SymbolType for ClassDefinitionSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
