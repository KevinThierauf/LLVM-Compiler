use crate::module::modulepos::ModuleRange;
use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::function::FunctionDefinitionSym;
use crate::ast::symbol::SymbolType;
use crate::module::visibility::Visibility;

pub struct ClassFieldDefinition {
    name: ModuleRange,
    visibility: Visibility,
    // typeName or defaultValue (or both) MUST be Some
    typeName: Option<ModuleRange>,
    defaultValue: Option<Expr>,
}

pub struct ClassStaticFieldDefinition {
    name: ModuleRange,
    typeName: Option<ModuleRange>,
    visibility: Visibility,
    value: Expr,
}

pub struct ClassMethodDefinition {
    function: FunctionDefinitionSym,
    visibility: Visibility,
}

pub struct ClassStaticFunctionDefinition {
    function: FunctionDefinitionSym,
    visibility: Visibility,
}

pub struct ClassDefinitionSym {
    visibility: Visibility,
    range: ModuleRange,
    name: ModuleRange,
    fields: Vec<ClassFieldDefinition>,
    methods: Vec<ClassMethodDefinition>,
    staticMethods: Vec<ClassStaticFunctionDefinition>,
    inherited: Vec<ModuleRange>,
}

impl SymbolType for ClassDefinitionSym {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}
