use crate::ast::AbstractSyntaxTree;
use crate::ast::symbol::Symbol;
use crate::resolver::exporttable::ExportTableMutex;
use crate::resolver::resolvedast::ResolvedAST;

pub mod resolutionselector;
pub mod exporttable;
pub mod resolvedast;
pub mod typeinfo;

pub struct Resolver {
    ast: Vec<Symbol>,
    exportTable: ExportTableMutex,
}

impl Resolver {
    fn new(ast: AbstractSyntaxTree, exportTable: ExportTableMutex) -> Self {
        return Self {
            ast: ast.take(),
            exportTable,
        };
    }

    pub fn resolve(ast: AbstractSyntaxTree, exportTable: ExportTableMutex) -> Self {
        let mut resolver = Self::new(ast, exportTable);
        resolver.resolveExports();
        return resolver;
    }

    // resolve public symbols
    // public symbols (across modules) must be resolved before private symbols
    fn resolveExports(&mut self) {
        todo!()
    }

    // resolve private symbols
    pub fn getResolvedAST(self) -> ResolvedAST {
        todo!()
    }
}
