use std::sync::Arc;

use hashbrown::hash_map::Entry;
use hashbrown::HashMap;

use crate::ast::SymbolPos;
use crate::resolver::exporttable::incompleteexporttable::IncompleteExportTable;
use crate::resolver::function::Function;
use crate::resolver::resolutionerror::ResolutionError;
use crate::resolver::typeinfo::Type;

pub mod coreexporttable;

#[derive(Debug)]
pub struct CompleteExportTable {
    exportTypes: HashMap<String, Type>,
    exportFunctions: HashMap<String, Function>,
}

impl CompleteExportTable {
    pub fn new(exportTable: &IncompleteExportTable, importedTables: Vec<Arc<CompleteExportTable>>) -> Result<Arc<CompleteExportTable>, Vec<ResolutionError>> {
        let mut builder = Self::newBuilder();

        // for symbol in &exportTable.symbolVec {
        //     match symbol.getSymbol() {
        //         Symbol::ClassDefinition(definition) => {
        //             // definition.
        //             todo!()
        //         }
        //         Symbol::FunctionDefinition(definition) => {
        //             todo!()
        //         }
        //         _ if IncompleteExportTable::isExportable(symbol).is_some() => unimplemented!("missing export handle for {:?}", symbol.getSymbol()),
        //         _ => panic!("cannot export symbol {:?}", symbol.getSymbol()),
        //     }
        // }

        return Ok(Arc::new(builder));
    }

    pub fn newBuilder() -> Self {
        return Self {
            exportTypes: Default::default(),
            exportFunctions: Default::default(),
        };
    }

    pub fn getExportedType(&self, name: &String, pos: SymbolPos, importedTables: &Vec<Arc<CompleteExportTable>>) -> Result<Type, ResolutionError> {
        let mut ty = self.exportTypes.get(name).map(|ty| ty.to_owned());
        
        for table in importedTables {
            if let Some(importedType) = table.exportTypes.get(name) {
                if let Some(ty) = ty {
                    return Err(ResolutionError::ConflictingType(pos, ty, importedType.to_owned()));
                }
                ty = Some(importedType.to_owned());
            }
        }
        
        return ty.ok_or(ResolutionError::UnknownType(pos, name.to_owned()));
    }

    pub fn getExportedFunction(&self, name: &String, pos: SymbolPos, importedTables: &Vec<Arc<CompleteExportTable>>) -> Result<Function, ResolutionError> {
        let mut function = self.exportFunctions.get(name).map(|function| function.to_owned());

        for table in importedTables {
            if let Some(importedFunction) = table.exportFunctions.get(name) {
                if let Some(function) = function {
                    return Err(ResolutionError::ConflictingFunction(pos, function, importedFunction.to_owned()));
                }
                function = Some(importedFunction.to_owned());
            }
        }

        return function.ok_or(ResolutionError::UnknownFunction(pos, name.to_owned()));
    }

    pub fn addExportedType(&mut self, pos: SymbolPos, ty: Type) -> Result<(), ResolutionError> {
        match self.exportTypes.entry(ty.getTypeName().to_owned()) {
            Entry::Occupied(entry) => {
                Err(ResolutionError::ConflictingType(pos, entry.get().to_owned(), ty))
            }
            Entry::Vacant(entry) => {
                entry.insert(ty);
                Ok(())
            }
        }
    }

    pub fn addExportedFunction(&mut self, pos: SymbolPos, function: Function) -> Result<(), ResolutionError> {
        match self.exportFunctions.entry(function.getFunctionName().to_owned()) {
            Entry::Occupied(entry) => {
                Err(ResolutionError::ConflictingFunction(pos, entry.get().to_owned(), function))
            }
            Entry::Vacant(entry) => {
                entry.insert(function);
                Ok(())
            }
        }
    }
}
