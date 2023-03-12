use std::rc::Rc;

use crate::ast::AbstractSyntaxTree;
use crate::module::visibility::Visibility::Private;
use crate::resolver::exporttable::ModuleExportTable;
use crate::resolver::exporttable::incompleteexporttable::{IncompleteExportTable, VisibilityExportHandler};
use crate::resolver::resolvedast::ResolvedAST;

pub mod resolutionselector;
pub mod exporttable;
pub mod resolvedast;
pub mod typeinfo;
pub mod function;
pub mod resolutionerror;

pub struct Resolver {
    ast: Rc<AbstractSyntaxTree>,
    exportTable: ModuleExportTable,
    privateExportTable: IncompleteExportTable,
}

impl Resolver {
    pub fn new(ast: Rc<AbstractSyntaxTree>, exportTable: ModuleExportTable) -> Self {
        let mut resolver = Self {
            ast,
            exportTable,
            privateExportTable: IncompleteExportTable::new(VisibilityExportHandler(Private)),
        };
        resolver.collectExports();
        return resolver;
    }

    // collect exported symbols
    // exported symbols must be resolved before other symbols reference them
    fn collectExports(&mut self) {
        for index in 0..self.ast.getSymbols().len() {
            self.privateExportTable.addSymbolIfExported(self.ast.getPos(index));
        }
        self.exportTable.getIncompleteExportTable(|table| {
            for index in 0..self.ast.getSymbols().len() {
                table.addSymbolIfExported(self.ast.getPos(index));
            }
        });
    }

    // resolve private symbols
    pub fn getResolvedAST(self) -> ResolvedAST {
        let table = self.exportTable.getCompleteExportTableBlocking();
        todo!()
    }
}
