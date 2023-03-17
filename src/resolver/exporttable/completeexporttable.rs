use std::ops::Deref;
use std::sync::Arc;

use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use once_cell::sync::Lazy;

use crate::resolver::exporttable::incompleteexporttable::IncompleteExportTable;
use crate::resolver::function::Function;
use crate::resolver::resolutionerror::ResolutionError;
use crate::resolver::typefunctioninfo::TypeFunctionInfo;
use crate::resolver::typeinfo::Type;

pub mod coreexporttable;

#[derive(Debug)]
pub struct CompleteExportTable {
    exportTypes: HashMap<String, Type>,
    exportFunctions: HashMap<String, Function>,
    dependencies: Vec<Arc<CompleteExportTable>>,
    typeFunctionInfo: HashMap<Type, Arc<TypeFunctionInfo>>,
}

static EMPTY_FUNCTION_INFO: Lazy<Arc<TypeFunctionInfo>> = Lazy::new(|| Arc::new(TypeFunctionInfo::new()));

impl CompleteExportTable {
    pub fn new(exportTable: IncompleteExportTable, mut importedTables: Vec<Arc<CompleteExportTable>>) -> Result<Arc<CompleteExportTable>, Vec<ResolutionError>> {
        let mut builder = Self::newBuilder();
        builder.dependencies.append(&mut importedTables);
        exportTable.complete(&mut builder)?;
        return Ok(Arc::new(builder));
    }

    pub fn newBuilder() -> Self {
        return Self {
            exportTypes: Default::default(),
            exportFunctions: Default::default(),
            dependencies: Vec::new(),
            typeFunctionInfo: Default::default(),
        };
    }

    pub fn addDependency(&mut self, exportTable: Arc<CompleteExportTable>) {
        self.dependencies.push(exportTable);
    }

    pub fn setTypeFunctionInfo(&mut self, ty: Type, functionInfo: TypeFunctionInfo) {
        let _prev = self.typeFunctionInfo.insert(ty, Arc::new(functionInfo));
        debug_assert!(_prev.is_none());
    }

    pub fn getTypeFunctionInfo(&mut self, ty: Type) -> Arc<TypeFunctionInfo> {
        return self.typeFunctionInfo.get(&ty).unwrap_or(EMPTY_FUNCTION_INFO.deref()).to_owned();
    }

    pub fn getExportedType(&self, name: &str) -> Result<Type, ResolutionError> {
        let mut ty = self.exportTypes.get(name).map(|ty| ty.to_owned());

        for table in &self.dependencies {
            if let Some(importedType) = table.exportTypes.get(name) {
                if let Some(ty) = ty {
                    return Err(ResolutionError::ConflictingType(ty, importedType.to_owned()));
                }
                ty = Some(importedType.to_owned());
            }
        }

        return ty.ok_or(ResolutionError::UnknownType(name.to_owned()));
    }

    pub fn getExportedFunction(&self, name: &str) -> Result<Function, ResolutionError> {
        let mut function = self.exportFunctions.get(name).map(|function| function.to_owned());

        for table in &self.dependencies {
            if let Some(importedFunction) = table.exportFunctions.get(name) {
                if let Some(function) = function {
                    return Err(ResolutionError::ConflictingFunction(function, importedFunction.to_owned()));
                }
                function = Some(importedFunction.to_owned());
            }
        }

        return function.ok_or(ResolutionError::UnknownFunction(name.to_owned()));
    }

    pub fn addExportedType(&mut self, ty: Type) -> Result<(), ResolutionError> {
        match self.exportTypes.entry(ty.getTypeName().to_owned()) {
            Entry::Occupied(entry) => {
                Err(ResolutionError::ConflictingType(entry.get().to_owned(), ty))
            }
            Entry::Vacant(entry) => {
                entry.insert(ty);
                Ok(())
            }
        }
    }

    pub fn addExportedFunction(&mut self, function: Function) -> Result<(), ResolutionError> {
        match self.exportFunctions.entry(function.getFunctionName().to_owned()) {
            Entry::Occupied(entry) => {
                Err(ResolutionError::ConflictingFunction(entry.get().to_owned(), function))
            }
            Entry::Vacant(entry) => {
                entry.insert(function);
                Ok(())
            }
        }
    }
}
