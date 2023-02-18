use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::function::FunctionDefinitionSym;
use crate::ast::symbol::SymbolType;
use crate::module::visibility::Visibility;

#[derive(Debug)]
pub struct ClassFieldDefinition {
    pub name: ModuleRange,
    pub visibility: Visibility,
    // typeName or defaultValue (or both) MUST be Some
    pub typeName: Option<ModuleRange>,
    pub defaultValue: Option<Expr>,
}

#[derive(Debug)]
pub struct ClassStaticFieldDefinition {
    pub name: ModuleRange,
    pub typeName: Option<ModuleRange>,
    pub visibility: Visibility,
    pub value: Expr,
}

#[derive(Debug)]
pub struct ClassMethodDefinition {
    pub function: FunctionDefinitionSym,
    pub visibility: Visibility,
}

#[derive(Debug)]
pub struct ClassStaticFunctionDefinition {
    pub function: FunctionDefinitionSym,
    pub visibility: Visibility,
}

#[derive(Debug)]
pub struct ClassDefinitionSym {
    pub visibility: Visibility,
    pub range: ModuleRange,
    pub name: ModuleRange,
    pub fields: Vec<ClassFieldDefinition>,
    pub methods: Vec<ClassMethodDefinition>,
    pub staticMethods: Vec<ClassStaticFunctionDefinition>,
    pub inherited: Vec<ModuleRange>,
}

impl SymbolType for ClassDefinitionSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
