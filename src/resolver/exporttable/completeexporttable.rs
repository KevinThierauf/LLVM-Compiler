use std::sync::Arc;

use hashbrown::hash_map::Entry;
use hashbrown::HashMap;

use crate::resolver::exporttable::incompleteexporttable::IncompleteExportTable;
use crate::resolver::function::Function;
use crate::resolver::resolutionerror::ResolutionError;
use crate::resolver::typeinfo::Type;

pub mod coreexporttable;

pub struct CompleteExportTable {
    exportTypes: HashMap<String, Type>,
    exportFunctions: HashMap<String, Function>,
}

impl CompleteExportTable {
    pub fn new(exportTable: &IncompleteExportTable, importedTables: Vec<Arc<CompleteExportTable>>) -> Arc<Self> {
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

        return Arc::new(builder);
    }

    pub fn newBuilder() -> Self {
        return Self {
            exportTypes: Default::default(),
            exportFunctions: Default::default(),
        };
    }

    pub fn addExportedType(&mut self, ty: Type) -> Result<(), ResolutionError> {
        match self.exportTypes.entry(ty.getTypeName().to_owned()) {
            Entry::Occupied(entry) => {
                Err(ResolutionError::ConflictingTypeDefinition(entry.get().to_owned(), ty))
            }
            Entry::Vacant(entry) => {
                entry.insert(ty);
                Ok(())
            },
        }
    }

    pub fn addExportedFunction(&mut self, function: Function) -> Result<(), ResolutionError> {
        match self.exportFunctions.entry(function.getFunctionName().to_owned()) {
            Entry::Occupied(entry) => {
                Err(ResolutionError::ConflictingFunctionDefinition(entry.get().to_owned(), function))
            }
            Entry::Vacant(entry) => {
                entry.insert(function);
                Ok(())
            },
        }
    }
}
