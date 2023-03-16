use std::fmt::Debug;

use hashbrown::{HashMap, HashSet};

use crate::ast::symbol::classdefinition::ClassDefinitionSym;
use crate::ast::symbol::function::FunctionDefinitionSym;
use crate::ast::symbol::Symbol;
use crate::ast::SymbolPos;
use crate::ast::visibility::Visibility;
use crate::resolver::exporttable::completeexporttable::CompleteExportTable;
use crate::resolver::function::{Function, Parameter};
use crate::resolver::resolutionerror::ResolutionError;
use crate::resolver::typefunctioninfo::TypeFunctionInfo;
use crate::resolver::typeinfo::class::ClassTypeInfo;
use crate::resolver::typeinfo::Type;

#[derive(Debug)]
struct IncompleteFunctionParameter {
    typeName: String,
    name: String,
}

#[derive(Debug)]
struct IncompleteFunction {
    name: String,
    returnType: String,
    visibility: Visibility,
    parameters: Vec<IncompleteFunctionParameter>,
}

impl IncompleteFunction {
    fn new(functionDefinition: &FunctionDefinitionSym) -> Result<Self, ResolutionError> {
        if functionDefinition.parameters.iter().any(|parameter| parameter.defaultExpr.is_some()) {
            return Err(ResolutionError::Unsupported(functionDefinition.range.getStartPos(), "unsupported default function parameters".to_owned()));
        }
        if !functionDefinition.attributeVec.is_empty() {
            return Err(ResolutionError::Unsupported(functionDefinition.range.getStartPos(), format!("attributes unsupported: {:?}", functionDefinition.attributeVec)));
        }
        return Ok(Self {
            name: functionDefinition.functionName.getToken().getSourceRange().getSourceInRange().to_owned(),
            returnType: functionDefinition.returnType.getToken().getSourceRange().getSourceInRange().to_string(),
            visibility: functionDefinition.visibility,
            parameters: functionDefinition.parameters.iter().map(|parameter| IncompleteFunctionParameter {
                typeName: parameter.typeName.getToken().getSourceRange().getSourceInRange().to_owned(),
                name: parameter.parameterName.getToken().getSourceRange().getSourceInRange().to_owned(),
            }).collect(),
        });
    }
}

#[derive(Debug)]
struct IncompleteField {
    visibility: Visibility,
    typeName: String,
    name: String,
}

#[derive(Debug)]
struct IncompleteClass {
    name: String,
    visibility: Visibility,
    fields: Vec<IncompleteField>,
    functions: Vec<IncompleteFunction>,
}

impl IncompleteClass {
    fn new(classDefinition: &ClassDefinitionSym) -> Result<Self, ResolutionError> {
        return Ok(Self {
            name: classDefinition.name.getToken().getSourceRange().getSourceInRange().to_owned(),
            visibility: classDefinition.visibility,
            fields: classDefinition.fields.iter().map(|v| IncompleteField {
                visibility: v.visibility,
                typeName: v.typeName.to_owned().unwrap().getToken().getSourceRange().getSourceInRange().to_owned(),
                name: v.name.getToken().getSourceRange().getSourceInRange().to_owned(),
            }).collect(),
            functions: {
                let mut methods = Vec::new();
                for method in &classDefinition.methods {
                    methods.push(IncompleteFunction::new(&method)?);
                }
                methods
            },
        });
    }
}

#[derive(Debug)]
pub struct IncompleteExportTable {
    classVec: Vec<IncompleteClass>,
    functionVec: Vec<IncompleteFunction>,
}

impl IncompleteExportTable {
    pub fn new() -> Self {
        return Self {
            classVec: Vec::new(),
            functionVec: Vec::new(),
        };
    }

    pub fn merge(&mut self, mut other: Self) {
        self.classVec.append(&mut other.classVec);
        self.functionVec.append(&mut other.functionVec);
    }

    fn addSymbol(&mut self, symbolPos: SymbolPos) -> Result<(), ResolutionError> {
        match symbolPos.getSymbol() {
            Symbol::ClassDefinition(definition) => {
                if !definition.staticFields.is_empty() {
                    return Err(ResolutionError::Unsupported(symbolPos.getModulePos(), "static fields".to_owned()));
                }
                if let Some(field) = definition.fields.iter().find(|field| field.defaultValue.is_some()) {
                    return Err(ResolutionError::Unsupported(field.name.to_owned(), "default values".to_owned()));
                }
                if let Some(field) = definition.fields.iter().find(|field| field.typeName.is_none() && field.defaultValue.is_none()) {
                    return Err(ResolutionError::ResolutionClassField(field.name.to_owned()));
                }
                self.classVec.push(IncompleteClass::new(definition)?);
            }
            Symbol::FunctionDefinition(definition) => {
                self.functionVec.push(IncompleteFunction::new(definition)?);
            }
            _ if self.isExported(&symbolPos) => unimplemented!("missing export handle for {:?}", symbolPos.getSymbol()),
            _ => panic!("cannot export symbol {:?}", symbolPos.getSymbol()),
        }
        return Ok(());
    }

    pub fn addSymbolIfExported(&mut self, symbolPos: SymbolPos) -> Result<(), ResolutionError> {
        if self.isExported(&symbolPos) {
            return self.addSymbol(symbolPos);
        }
        return Ok(());
    }

    pub fn isExported(&self, pos: &SymbolPos) -> bool {
        return Self::isExportable(pos);
    }

    pub fn isExportable(symbolPos: &SymbolPos) -> bool {
        return match symbolPos.getSymbol() {
            Symbol::ClassDefinition(_) | Symbol::FunctionDefinition(_) => true,
            _ => false
        };
    }

    pub fn complete(mut self, table: &mut CompleteExportTable) -> Result<(), Vec<ResolutionError>> {
        fn resolveFunction(errorVec: &mut Vec<ResolutionError>, function: IncompleteFunction, table: &CompleteExportTable) -> Option<Function> {
            fn getExported(errorVec: &mut Vec<ResolutionError>, typeName: &String, table: &CompleteExportTable) -> Option<Type> {
                return match table.getExportedType(typeName) {
                    Ok(ty) => Some(ty),
                    Err(error) => {
                        errorVec.push(error);
                        None
                    }
                };
            }

            let returnType = getExported(errorVec, &function.returnType, table)?;
            let mut parameterVec = Vec::new();

            if function.parameters.len() != function.parameters.iter().map(|parameter| &parameter.name).collect::<HashSet<_>>().len() {
                errorVec.push(ResolutionError::ConflictingParameterName(function.name));
                return None;
            }

            for parameter in function.parameters {
                parameterVec.push(Parameter {
                    ty: getExported(errorVec, &parameter.typeName, table)?,
                    name: parameter.name,
                })
            }

            return Some(Function::new(function.name, function.visibility, returnType, parameterVec));
        }

        let mut errorVec = Vec::new();

        let mut exportClasses = HashMap::new();
        let mut index = 0;
        while index < self.classVec.len() {
            let class = &mut self.classVec[index];
            if exportClasses.insert(class.name.to_owned(), ClassTypeInfo::newBuilder(class.name.to_owned(), class.visibility)).is_some() {
                errorVec.push(ResolutionError::ConflictingTypeDefinition(class.name.to_owned()));
                self.classVec.swap_remove(index);
            } else {
                index += 1;
            }
        }

        let mut classFunctionInfo = HashMap::new();

        while !self.classVec.is_empty() {
            let mut index = 0;
            let mut errorValue = false;
            while index < self.classVec.len() {
                let class = &mut self.classVec[index];
                class.fields.retain(|field| {
                    let fieldType = table.getExportedType(&field.typeName);
                    return match fieldType {
                        Ok(ty) => {
                            if let Err(error) = exportClasses.get_mut(&class.name.to_owned()).unwrap().addFieldFrom(field.visibility, ty, field.name.to_owned()) {
                                errorVec.push(error);
                                errorValue = true;
                            }
                            false
                        }
                        Err(err) => {
                            if exportClasses.contains_key(&field.typeName) {
                                true
                            } else {
                                if let ResolutionError::UnknownType(_) = err {
                                    true
                                } else {
                                    if class.name == field.name {
                                        errorVec.push(ResolutionError::CircularDependencies(vec![field.name.to_owned()]));
                                    } else {
                                        errorVec.push(err);
                                    }
                                    errorValue = true;
                                    false
                                }
                            }
                        }
                    };
                });

                if class.fields.is_empty() || errorValue {
                    let class = self.classVec.swap_remove(index);
                    let ty = exportClasses.remove(&class.name).unwrap().build();
                    if let Err(error) = table.addExportedType(ty.to_owned()) {
                        errorVec.push(error);
                    } else {
                        classFunctionInfo.insert(ty, class.functions);
                    }
                } else {
                    index += 1;
                }
            }
        }

        if errorVec.is_empty() {
            for (class, functions) in classFunctionInfo {
                let mut classFunctions = TypeFunctionInfo::new();
                functions.into_iter().for_each(|function| if let Some(function) = resolveFunction(&mut errorVec, function, table) {
                    if let Err(err) = classFunctions.addFunction(function) {
                        errorVec.push(err);
                    }
                });
                table.setTypeFunctionInfo(class, classFunctions);
            }

            for function in self.functionVec {
                if let Some(function) = resolveFunction(&mut errorVec, function, table) {
                    if let Err(err) = table.addExportedFunction(function) {
                        errorVec.push(err);
                    }
                }
            }
        }

        return if errorVec.is_empty() {
            Ok(())
        } else {
            Err(errorVec)
        };
    }
}
