use crate::ast::symbol::Symbol;
use crate::ast::SymbolPos;
use crate::module::visibility::Visibility;

pub trait ExportHandler: 'static {
    fn isExported(&self, pos: &SymbolPos, visibility: Visibility) -> bool;
}

pub struct VisibilityExportHandler(pub Visibility);

impl ExportHandler for VisibilityExportHandler {
    fn isExported(&self, _: &SymbolPos, visibility: Visibility) -> bool {
        return visibility == self.0;
    }
}

pub struct IncompleteExportTable {
    symbolVec: Vec<SymbolPos>,
    exportHandler: Box<dyn ExportHandler>,
}

impl IncompleteExportTable {
    pub fn new(exportHandler: impl ExportHandler) -> Self {
        return Self {
            symbolVec: Vec::new(),
            exportHandler: Box::new(exportHandler),
        };
    }

    pub fn merge(&mut self, mut other: Self) {
        self.symbolVec.append(&mut other.symbolVec);
    }

    fn addSymbol(&mut self, symbolPos: SymbolPos) {
        debug_assert!(!self.symbolVec.contains(&symbolPos));
        self.symbolVec.push(symbolPos);
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
