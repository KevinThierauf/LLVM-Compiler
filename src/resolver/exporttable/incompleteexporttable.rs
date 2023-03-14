use std::fmt::Debug;
use crate::ast::symbol::classdefinition::ClassDefinitionSym;
use crate::ast::symbol::function::FunctionDefinitionSym;

use crate::ast::symbol::Symbol;
use crate::ast::SymbolPos;
use crate::ast::visibility::Visibility;

pub trait ExportHandler: 'static + Debug + Send + Sync {
    fn isExported(&self, pos: &SymbolPos, visibility: Visibility) -> bool;
}

#[derive(Debug)]
pub struct VisibilityExportHandler(pub Visibility);

impl ExportHandler for VisibilityExportHandler {
    fn isExported(&self, _: &SymbolPos, visibility: Visibility) -> bool {
        return visibility == self.0;
    }
}

#[derive(Debug)]
struct IncompleteFunctionParameter {
    typeName: String,
    name: String,
}

#[derive(Debug)]
struct IncompleteFunction {
    name: String,
    returnType: String,
    parameters: Vec<IncompleteFunctionParameter>,
}

impl IncompleteFunction {
    fn new(functionDefinition: &FunctionDefinitionSym) -> Self {
        todo!()
    }
}

#[derive(Debug)]
struct IncompleteField {
    typeName: String,
    name: String,
}

#[derive(Debug)]
struct IncompleteClass {
    name: String,
    exportedFields: Vec<IncompleteField>,
    exportedFunctions: Vec<IncompleteFunction>,
}

impl IncompleteClass {
    fn new(classDefinition: &ClassDefinitionSym) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct IncompleteExportTable {
    classVec: Vec<IncompleteClass>,
    functionVec: Vec<IncompleteFunction>,
    exportHandler: Box<dyn ExportHandler>,
}

impl IncompleteExportTable {
    pub fn new(exportHandler: impl ExportHandler) -> Self {
        return Self {
            classVec: Vec::new(),
            functionVec: Vec::new(),
            exportHandler: Box::new(exportHandler),
        };
    }

    pub fn merge(&mut self, mut other: Self) {
        self.classVec.append(&mut other.classVec);
        self.functionVec.append(&mut other.functionVec);
    }

    fn addSymbol(&mut self, symbolPos: SymbolPos) {
        match symbolPos.getSymbol() {
            Symbol::ClassDefinition(definition) => {
                self.classVec.push(IncompleteClass::new(definition));
            }
            Symbol::FunctionDefinition(definition) => {
                self.functionVec.push(IncompleteFunction::new(definition));
            }
            _ if IncompleteExportTable::isExportable(&symbolPos).is_some() => unimplemented!("missing export handle for {:?}", symbolPos.getSymbol()),
            _ => panic!("cannot export symbol {:?}", symbolPos.getSymbol()),
        }
    }

    pub fn addSymbolIfExported(&mut self, symbolPos: SymbolPos) {
        if self.isExported(&symbolPos) {
            self.addSymbol(symbolPos);
        }
    }

    pub fn isExported(&self, pos: &SymbolPos) -> bool {
        return if let Some(visibility) = Self::isExportable(pos) {
            self.exportHandler.isExported(pos, visibility)
        } else {
            false
        };
    }

    pub fn isExportable(symbolPos: &SymbolPos) -> Option<Visibility> {
        return match symbolPos.getSymbol() {
            Symbol::ClassDefinition(definition) => {
                Some(definition.visibility)
            }
            Symbol::FunctionDefinition(definition) => {
                Some(definition.visibility)
            }
            _ => {
                None
            }
        };
    }
}
