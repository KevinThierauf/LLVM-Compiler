use std::rc::Rc;

use crate::ast::AbstractSyntaxTree;
use crate::resolver::exporttable::GlobalExportTable;
use crate::resolver::exporttable::incompleteexporttable::IncompleteExportTable;
use crate::resolver::resolutionerror::ResolutionError;
use crate::resolver::resolvedast::ResolvedAST;

pub mod typeresolutionselector;
pub mod exporttable;
pub mod resolvedast;
pub mod typeinfo;
pub mod function;
pub mod resolutionerror;
pub mod typefunctioninfo;

pub struct Resolver {
    ast: Rc<AbstractSyntaxTree>,
    exportTable: GlobalExportTable,
}

impl Resolver {
    pub fn new(ast: Rc<AbstractSyntaxTree>, exportTable: GlobalExportTable) -> Result<Self, Vec<ResolutionError>> {
        let mut resolver = Self {
            ast,
            exportTable,
        };
        resolver.collectExports()?;
        return Ok(resolver);
    }

    // collect exported symbols
    // exported symbols must be resolved before other symbols reference them
    fn collectExports(&mut self) -> Result<(), Vec<ResolutionError>> {
        let mut resolutionErrorVec = Vec::new();
        let mut exportTable = IncompleteExportTable::new();
        for index in 0..self.ast.getSymbols().len() {
            if let Err(err) = exportTable.addSymbolIfExported(self.ast.getPos(index)) {
                resolutionErrorVec.push(err);
            }
        }
        if !resolutionErrorVec.is_empty() {
            return Err(resolutionErrorVec);
        }
        self.exportTable.getIncompleteExportTable(|table| {
            table.merge(exportTable);
        });
        return Ok(());
    }

    // resolve symbols
    pub fn getResolvedAST(self) -> Result<ResolvedAST, Vec<ResolutionError>> {
        let exportTable = self.exportTable.getCompleteExportTableBlocking().ok_or_else(|| vec![])?;
        // todo!()
        return Ok(ResolvedAST::new());
    }
}
