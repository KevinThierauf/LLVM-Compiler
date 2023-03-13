use std::rc::Rc;

use crate::ast::AbstractSyntaxTree;
use crate::module::visibility::Visibility::{Private, Public};
use crate::resolver::exporttable::completeexporttable::CompleteExportTable;
use crate::resolver::exporttable::completeexporttable::coreexporttable::CORE_EXPORT_TABLE;
use crate::resolver::exporttable::incompleteexporttable::{IncompleteExportTable, VisibilityExportHandler};
use crate::resolver::exporttable::GlobalExportTable;
use crate::resolver::resolutionerror::ResolutionError;
use crate::resolver::resolvedast::ResolvedAST;

pub mod typeresolutionselector;
pub mod exporttable;
pub mod resolvedast;
pub mod typeinfo;
pub mod function;
pub mod resolutionerror;

pub struct Resolver {
    ast: Rc<AbstractSyntaxTree>,
    exportTable: GlobalExportTable,
    privateExportTable: IncompleteExportTable,
}

impl Resolver {
    pub fn new(ast: Rc<AbstractSyntaxTree>, exportTable: GlobalExportTable) -> Self {
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
        let mut exportTable = IncompleteExportTable::new(VisibilityExportHandler(Public));
        for index in 0..self.ast.getSymbols().len() {
            exportTable.addSymbolIfExported(self.ast.getPos(index));
            self.privateExportTable.addSymbolIfExported(self.ast.getPos(index));
        }
        self.exportTable.getIncompleteExportTable(|table| {
            table.merge(exportTable);
        });
    }

    // resolve private symbols
    pub fn getResolvedAST(self) -> Result<ResolvedAST, Vec<ResolutionError>> {
        let table = CompleteExportTable::new(&self.privateExportTable, vec![
            CORE_EXPORT_TABLE.to_owned(), self.exportTable.getCompleteExportTableBlocking().ok_or_else(|| vec![])?,
        ])?;
        todo!()
    }
}
