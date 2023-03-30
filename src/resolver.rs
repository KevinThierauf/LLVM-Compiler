use std::fmt::Debug;
use std::mem::swap;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use hashbrown::hash_map::Entry;
use hashbrown::HashMap;

use crate::ast::AbstractSyntaxTree;
use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::symbol::block::BlockSym;
use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::expr::functioncall::FunctionCallExpr;
use crate::ast::symbol::function::FunctionDefinitionSym;
use crate::module::modulepos::ModulePos;
use crate::module::Operator;
use crate::resolver::exporttable::completeexporttable::CompleteExportTable;
use crate::resolver::exporttable::GlobalExportTable;
use crate::resolver::exporttable::incompleteexporttable::IncompleteExportTable;
use crate::resolver::function::Function;
use crate::resolver::resolutionerror::ResolutionError;
use crate::resolver::resolvedast::functioncall::FunctionCall;
use crate::resolver::resolvedast::ifstatement::IfStatement;
use crate::resolver::resolvedast::printstatement::PrintStatement;
use crate::resolver::resolvedast::ResolvedAST;
use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::resolvedfunctiondefinition::ResolvedFunctionDefinition;
use crate::resolver::resolvedast::resolvedoperator::ResolvedOperator;
use crate::resolver::resolvedast::resolvedproperty::ResolvedProperty;
use crate::resolver::resolvedast::resolvedscope::ResolvedScope;
use crate::resolver::resolvedast::resolvedvariable::ResolvedVariable;
use crate::resolver::resolvedast::returnstatement::ReturnStatement;
use crate::resolver::resolvedast::statement::Statement;
use crate::resolver::resolvedast::variabledeclare::VariableDeclare;
use crate::resolver::resolvedast::whilestatement::WhileStatement;
use crate::resolver::typeinfo::primitive::boolean::BOOLEAN_TYPE;
use crate::resolver::typeinfo::primitive::character::CHARACTER_TYPE;
use crate::resolver::typeinfo::primitive::float::FLOAT_TYPE;
use crate::resolver::typeinfo::primitive::integer::INTEGER_TYPE;
use crate::resolver::typeinfo::string::STRING_TYPE;
use crate::resolver::typeinfo::Type;
use crate::resolver::typeinfo::void::VOID_TYPE;

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
        let mut resolutionHandler = ResolutionHandler {
            exportTable: self.exportTable.getCompleteExportTableBlocking().ok_or_else(|| vec![])?,
            resolver: vec![Rc::new(TopLevelResolver)],
            errorVec: Vec::new(),
            scope: Scope::root(),
        };

        return if let Some(statementVec) = resolutionHandler.resolveAll(true, self.ast.getSymbols().iter()) {
            debug_assert!(resolutionHandler.errorVec.is_empty());
            Ok(ResolvedAST::new(ResolvedScope {
                statementVec,
            }))
        } else {
            debug_assert!(!resolutionHandler.errorVec.is_empty(), "compilation failed but no errors provided");
            Err(resolutionHandler.errorVec)
        };
    }
}

enum Resolution {
    Ok(Statement),
    Err,
    Parent,
}

trait ResolverType: 'static + Debug {
    fn resolve(&self, resolutionHandler: &mut ResolutionHandler, symbol: &Symbol) -> Resolution;
}

#[derive(Debug)]
struct TopLevelResolver;

#[derive(Debug)]
struct FunctionResolver(Function);

struct Scope {
    parent: Option<Box<Scope>>,
    variableMap: HashMap<String, ResolvedVariable>,
}

struct ResolutionHandler {
    resolver: Vec<Rc<dyn ResolverType>>,
    errorVec: Vec<ResolutionError>,
    exportTable: Arc<CompleteExportTable>,
    scope: Scope,
}

impl TopLevelResolver {
    fn checkReturnStatement(errorVec: &mut Vec<ResolutionError>, returnType: Type, statementVec: &Vec<Statement>) {
        if returnType != VOID_TYPE.to_owned() {
            // last statement must be return statement
            if let Some(last) = statementVec.last() {
                if let Statement::Return(v) = last {
                    // should have been checked earlier
                    debug_assert_eq!(v.expr.as_ref().map(|v| v.getExpressionType()), Some(returnType));
                    // success
                    return;
                }
            }
            errorVec.push(ResolutionError::MissingReturn(format!("function missing return (non-void functions must always have return statement as last statement in function).", )))
        }
    }

    fn resolveFunction(&self, selfType: Option<Type>, function: Function, resolutionHandler: &mut ResolutionHandler, functionDefinition: &FunctionDefinitionSym) -> Option<ResolvedFunctionDefinition> {
        // parameter scope
        resolutionHandler.pushScope();

        fn resolveFunctionInner(selfType: Option<Type>, function: &Function, resolutionHandler: &mut ResolutionHandler, functionDefinition: &FunctionDefinitionSym) -> Option<(ResolvedScope, Vec<usize>)> {
            let mut parameterVec = Vec::new();
            if let Some(selfType) = selfType {
                parameterVec.push(resolutionHandler.scope.declareVariable("self", selfType, &mut resolutionHandler.errorVec)?.id);
            }

            for parameter in &function.parameters {
                parameterVec.push(resolutionHandler.scope.declareVariable(&parameter.name, parameter.ty.to_owned(), &mut resolutionHandler.errorVec)?.id);
            }
            resolutionHandler.pushResolver(FunctionResolver(function.to_owned()));
            let resolvedScope = resolutionHandler.resolveBlock(&functionDefinition.functionBlock);
            resolutionHandler.popResolver();
            let resolvedScope = resolvedScope?;
            TopLevelResolver::checkReturnStatement(&mut resolutionHandler.errorVec, function.returnType.to_owned(), &resolvedScope.statementVec);
            return Some((resolvedScope, parameterVec));
        }

        let inner = resolveFunctionInner(selfType, &function, resolutionHandler, functionDefinition);
        resolutionHandler.popScope();
        let (resolvedScope, parameterVec) = inner?;

        return Some(ResolvedFunctionDefinition {
            function,
            parameterVecId: parameterVec,
            scope: resolvedScope,
        });
    }
}

impl ResolverType for TopLevelResolver {
    fn resolve(&self, resolutionHandler: &mut ResolutionHandler, symbol: &Symbol) -> Resolution {
        return match symbol {
            Symbol::ClassDefinition(symbol) => {
                let mut resolvedVec = Vec::new();
                let classType = resolutionHandler.exportTable.getExportedType(&symbol.name.getToken().getSourceRange().getSourceInRange()).expect("unable to find type defined by class");
                let functionInfo = resolutionHandler.exportTable.getTypeFunctionInfo(classType.to_owned());

                for functionDefinition in &symbol.methods {
                    let functionName = functionDefinition.functionName.getToken().getSourceRange().getSourceInRange();
                    let function = functionInfo.getFunction(functionName).expect("unable to find function for class definition");
                    if let Some(resolved) = self.resolveFunction(Some(classType.to_owned()), function, resolutionHandler, functionDefinition) {
                        resolvedVec.push(Statement::FunctionDefinition(resolved));
                    } else {
                        return Resolution::Err;
                    }
                }
                Resolution::Ok(Statement::Multiple(resolvedVec))
            }
            Symbol::FunctionDefinition(functionDefinition) => {
                let function = resolutionHandler.exportTable.getExportedFunction(functionDefinition.functionName.getToken().getSourceRange().getSourceInRange()).expect("unable to find function for definition");
                return if let Some(resolved) = self.resolveFunction(None, function, resolutionHandler, functionDefinition) {
                    Resolution::Ok(Statement::FunctionDefinition(resolved))
                } else {
                    Resolution::Err
                };
            }
            _ => Resolution::Parent,
        };
    }
}

impl ResolverType for FunctionResolver {
    fn resolve(&self, resolutionHandler: &mut ResolutionHandler, symbol: &Symbol) -> Resolution {
        return match symbol {
            Symbol::Return(symbol) => {
                let (statement, ty) = if let Some(expr) = &symbol.value {
                    let resolved = resolutionHandler.resolveExpr(expr, false);
                    if let Some(resolved) = resolved {
                        let ty = resolved.getExpressionType();
                        (Statement::Return(ReturnStatement {
                            expr: Some(resolved),
                        }), ty)
                    } else {
                        return Resolution::Err;
                    }
                } else {
                    (Statement::Return(ReturnStatement {
                        expr: None,
                    }), VOID_TYPE.to_owned())
                };

                return if ty == self.0.returnType {
                    Resolution::Ok(statement)
                } else {
                    resolutionHandler.errorVec.push(ResolutionError::ExpectedType(self.0.returnType.to_owned(), ty, format!("mismatched return type")));
                    Resolution::Err
                };
            }
            _ => Resolution::Parent,
        };
    }
}

impl ResolutionHandler {
    fn resolveAll<'a>(&mut self, global: bool, symbols: impl Iterator<Item = &'a Symbol>) -> Option<Vec<Statement>> {
        let mut statementVec = Vec::new();
        for symbol in symbols {
            if let Some(statement) = self.resolve(symbol, global) {
                statementVec.push(statement);
            } else {
                debug_assert!(!self.errorVec.is_empty(), "failed to resolve symbol {symbol:?} but no error provided");
                return None;
            }
        }
        return Some(statementVec);
    }

    fn pushResolver(&mut self, resolver: impl ResolverType) {
        self.resolver.push(Rc::new(resolver));
    }

    fn popResolver(&mut self) {
        debug_assert!(self.resolver.len() > 1);
        self.resolver.pop();
    }

    fn pushScope(&mut self) {
        let mut tmp = Scope::root();
        swap(&mut self.scope, &mut tmp);
        self.scope = Scope::new(tmp);
    }

    fn popScope(&mut self) {
        let mut tmp = None;
        swap(&mut self.scope.parent, &mut tmp);
        if let Some(parent) = tmp {
            self.scope = *parent;
        } else {
            panic!("cannot pop scope (scope is root)");
        }
    }

    fn resolveBlock(&mut self, block: &BlockSym) -> Option<ResolvedScope> {
        self.pushScope();
        let resolved = self.resolveAll(false, block.symbolVec.iter());
        self.popScope();
        return Some(ResolvedScope {
            statementVec: resolved?,
        });
    }

    fn resolveExpr(&mut self, expr: &Expr, global: bool) -> Option<ResolvedExpr> {
        Some(getResolvedExpression(self, expr, global, Box::new(|_, resolved| resolved))?)
    }

    fn resolve(&mut self, symbol: &Symbol, global: bool) -> Option<Statement> {
        match self.resolver.last().unwrap().to_owned().resolve(self, symbol) {
            Resolution::Ok(symbol) => return Some(symbol),
            Resolution::Err => return None,
            Resolution::Parent => {
                // continue
            }
        };

        return match symbol {
            Symbol::Block(symbol) => {
                Some(Statement::Scope(self.resolveBlock(symbol)?))
            }
            Symbol::While(symbol) => {
                return getResolvedExpression(self, &symbol.condition, global, Box::new(|resolutionHandler, expr| {
                    return if expr.getExpressionType() == BOOLEAN_TYPE.to_owned() {
                        let statement = resolutionHandler.resolve(symbol.symbol.deref(), global)?;
                        Some(Statement::While(Box::new(WhileStatement {
                            condition: expr,
                            statement,
                        })))
                    } else {
                        resolutionHandler.errorVec.push(ResolutionError::ExpectedType(BOOLEAN_TYPE.to_owned(), expr.getExpressionType(), format!("expected boolean conditional for while loop")));
                        None
                    };
                })).flatten();
            }
            Symbol::IfSym(symbol) => {
                return getResolvedExpression(self, &symbol.condition, global, Box::new(|resolutionHandler, expr| {
                    return if expr.getExpressionType() == BOOLEAN_TYPE.to_owned() {
                        let statement = resolutionHandler.resolve(symbol.symbol.deref(), false)?;
                        let elseStatement = if let Some(elseSym) = &symbol.elseExpr {
                            Some(resolutionHandler.resolve(&elseSym.symbol, false)?)
                        } else {
                            None
                        };
                        Some(Statement::If(Box::new(IfStatement {
                            condition: expr,
                            statement,
                            elseStatement,
                        })))
                    } else {
                        resolutionHandler.errorVec.push(ResolutionError::ExpectedType(BOOLEAN_TYPE.to_owned(), expr.getExpressionType(), format!("expected boolean conditional for if statement")));
                        None
                    };
                })).flatten();
            }
            Symbol::PrintSym(symbol) => {
                return getResolvedExpression(self, &symbol.expr, false, Box::new(|resolutionHandler, expr| {
                    let ty = expr.getExpressionType();
                    return if ty == INTEGER_TYPE || ty == FLOAT_TYPE || ty == STRING_TYPE {
                        Some(Statement::Print(PrintStatement {
                            value: expr,
                        }))
                    } else {
                        resolutionHandler.errorVec.push(ResolutionError::InvalidOperationType(ty, format!("Cannot call print on type")));
                        None
                    };
                })).flatten();
            }
            Symbol::Expr(expr) => {
                Some(Statement::Expr(self.resolveExpr(expr, global)?))
            }
            Symbol::ClassDefinition(symbol) => {
                self.errorVec.push(ResolutionError::Unexpected(symbol.getRange().getStartPos(), format!("unexpected class definition {:?}", self.resolver.last().unwrap())));
                return None;
            }
            Symbol::Return(symbol) => {
                self.errorVec.push(ResolutionError::Unexpected(symbol.getRange().getStartPos(), "unexpected return statement".to_owned()));
                return None;
            }
            Symbol::FunctionDefinition(symbol) => {
                self.errorVec.push(ResolutionError::Unexpected(symbol.getRange().getStartPos(), "unexpected function definition".to_owned()));
                return None;
            }
            Symbol::Break(symbol) => {
                self.errorVec.push(ResolutionError::Unsupported(symbol.range.getStartPos(), "break".to_owned()));
                return None;
            }
            Symbol::Continue(symbol) => {
                self.errorVec.push(ResolutionError::Unsupported(symbol.range.getStartPos(), "continue".to_owned()));
                return None;
            }
            Symbol::ImportSym(symbol) => {
                self.errorVec.push(ResolutionError::Unsupported(symbol.range.getStartPos(), "import".to_owned()));
                return None;
            }
        };
    }
}

impl Scope {
    fn root() -> Self {
        return Self {
            parent: None,
            variableMap: Default::default(),
        };
    }

    fn new(parent: Scope) -> Self {
        return Self {
            parent: Some(Box::new(parent)),
            variableMap: Default::default(),
        };
    }

    fn getVariable(&self, name: &str) -> Option<ResolvedVariable> {
        return self.variableMap.get(name).map(|v| v.to_owned()).or_else(|| if let Some(parent) = &self.parent {
            parent.getVariable(name)
        } else {
            None
        });
    }

    fn getVariableOrError(&self, variableName: &str, errorVec: &mut Vec<ResolutionError>) -> Option<ResolvedVariable> {
        return if let Some(variable) = self.getVariable(variableName) {
            Some(variable)
        } else {
            errorVec.push(ResolutionError::UnknownVariable(format!("unknown variable '{}'", variableName)));
            None
        };
    }

    fn declareVariable(&mut self, name: &str, ty: Type, errorVec: &mut Vec<ResolutionError>) -> Option<ResolvedVariable> {
        static NEXT_VARIABLE_ID: AtomicUsize = AtomicUsize::new(0);

        return match self.variableMap.entry(name.to_owned()) {
            Entry::Occupied(_) => {
                errorVec.push(ResolutionError::ConflictingVariable(name.to_owned(), format!("found multiple variables in scope with same variable name")));
                None
            }
            Entry::Vacant(v) => {
                Some(v.insert(ResolvedVariable {
                    ty,
                    id: NEXT_VARIABLE_ID.fetch_add(1, Ordering::Relaxed),
                }).to_owned())
            }
        };
    }
}

fn getResolvedType<R>(resolutionHandler: &mut ResolutionHandler, modulePos: &ModulePos, callback: impl FnOnce(&mut ResolutionHandler, Type) -> R) -> Option<R> {
    let typeName = modulePos.getToken().getSourceRange().getSourceInRange();
    return match resolutionHandler.exportTable.getExportedType(typeName) {
        Ok(ty) => Some(callback(resolutionHandler, ty)),
        Err(err) => {
            resolutionHandler.errorVec.push(err);
            return None;
        }
    };
}

fn getResolvedFunctionCall(resolutionHandler: &mut ResolutionHandler, function: Function, functionCall: &FunctionCallExpr) -> Option<FunctionCall> {
    return if functionCall.argVec.len() == function.parameters.len() {
        let mut argVec = Vec::new();
        for index in 0..functionCall.argVec.len() {
            getResolvedExpression(resolutionHandler, &functionCall.argVec[index], false, Box::new(|resolutionHandler, expression| {
                if expression.getExpressionType() == function.parameters[index].ty {
                    argVec.push(expression);
                } else {
                    resolutionHandler.errorVec.push(ResolutionError::ExpectedType(function.parameters[index].ty.to_owned(), expression.getExpressionType(), format!("parameter type incorrect in function call")));
                }
            }));
        }

        if argVec.len() == functionCall.argVec.len() {
            Some(FunctionCall {
                function,
                argVec,
            })
        } else {
            None
        }
    } else {
        resolutionHandler.errorVec.push(ResolutionError::ParameterMismatch(function.to_owned(), format!("parameter mismatch: expected {} args, found {}", function.parameters.len(), functionCall.argVec.len())));
        None
    };
}

fn getResolvedExpression<'a, R>(resolutionHandler: &mut ResolutionHandler, expr: &Expr, global: bool, callback: Box<dyn 'a + FnOnce(&mut ResolutionHandler, ResolvedExpr) -> R>) -> Option<R> {
    let resolved = match expr {
        // Expr::ConstructorCall(expr) => {
        //     if !expr.argVec.is_empty() {
        //         resolutionHandler.errorVec.push(ResolutionError::Unsupported(expr.range.getStartPos(), format!("constructors do not support arguments")));
        //         return None;
        //     }
        //     getResolvedType(resolutionHandler, &expr.typeName, |_, ty| {
        //         ResolvedExpr::ConstructorCall(Box::new(ConstructorCall {
        //             ty,
        //         }))
        //     })?
        // }
        Expr::FunctionCall(expr) => {
            match resolutionHandler.exportTable.getExportedFunction(expr.functionName.getToken().getSourceRange().getSourceInRange()) {
                Ok(function) => {
                    ResolvedExpr::FunctionCall(Box::new(getResolvedFunctionCall(resolutionHandler, function, expr)?))
                }
                Err(error) => {
                    resolutionHandler.errorVec.push(error);
                    return None;
                }
            }
        }
        Expr::Operator(expr) => {
            if let Operator::Dot = expr.operator {
                let structure = getResolvedExpression(resolutionHandler, &expr.operands[0], global, Box::new(|_, resolved| resolved));
                debug_assert!(structure.is_some() || !resolutionHandler.errorVec.is_empty(), "failed to resolve {:?} but no error provided", &expr.operands[0]);
                let structure = structure?;
                let structureType = structure.getExpressionType();
                match &expr.operands[1] {
                    Expr::FunctionCall(functionCall) => {
                        let functionInfo = resolutionHandler.exportTable.getTypeFunctionInfo(structureType.to_owned());
                        let functionName = functionCall.functionName.getToken().getSourceRange().getSourceInRange();
                        match functionInfo.getFunction(functionName) {
                            Some(function) => {
                                let mut functionCall = getResolvedFunctionCall(resolutionHandler, function, functionCall)?;
                                functionCall.argVec.insert(0, structure);
                                ResolvedExpr::FunctionCall(Box::new(functionCall))
                            }
                            None => {
                                resolutionHandler.errorVec.push(ResolutionError::UnknownFunction(format!("unable to find method '{functionName}' of class '{}'", structureType.getTypeName())));
                                return None;
                            }
                        }
                    }
                    Expr::Variable(variable) => {
                        let variableName = variable.getRange().getSource();
                        if let Some(property) = structureType.getPropertyMap().get(&variableName) {
                            ResolvedExpr::Property(Box::new(ResolvedProperty {
                                value: structure,
                                property: property.to_owned(),
                            }))
                        } else {
                            resolutionHandler.errorVec.push(ResolutionError::UnknownVariable(format!("unable to find field '{variableName}' of class {}", structureType.getTypeName())));
                            return None;
                        }
                    }
                    _ => {
                        resolutionHandler.errorVec.push(ResolutionError::InvalidOperation(format!("dot operator can only be used to access a variable or function, found {:?}", expr.operands[1])));
                        return None;
                    }
                }
            } else {
                let mut exprVec = Vec::new();

                for expr in expr.operands.iter().map(|expr| getResolvedExpression(resolutionHandler, expr, global, Box::new(|_, resolved| resolved))).collect::<Vec<_>>() {
                    debug_assert!(expr.is_some() || !resolutionHandler.errorVec.is_empty());
                    exprVec.push(expr?);
                }

                let expressionType = match expr.operator {
                    Operator::Greater | Operator::Less | Operator::GreaterEq | Operator::LessEq | Operator::CompareEq | Operator::CompareNotEq => {
                        fn isPrimitiveType(ty: Type) -> bool {
                            return ty == INTEGER_TYPE.to_owned() || ty == BOOLEAN_TYPE.to_owned() || ty == FLOAT_TYPE.to_owned() || ty == CHARACTER_TYPE.to_owned();
                        }

                        if exprVec[0].getExpressionType() != exprVec[1].getExpressionType() {
                            resolutionHandler.errorVec.push(ResolutionError::ExpectedType(exprVec[0].getExpressionType(), exprVec[1].getExpressionType(), format!("mismatched types for operation expression")));
                            return None;
                        }

                        if !isPrimitiveType(exprVec[0].getExpressionType()) {
                            resolutionHandler.errorVec.push(ResolutionError::InvalidOperationType(exprVec[0].getExpressionType(), format!("cannot apply {:?} operator to non-primitive type", expr.operator)));
                            return None;
                        }

                        BOOLEAN_TYPE.to_owned()
                    }
                    Operator::Plus | Operator::Minus | Operator::Mult | Operator::Div => {
                        // arithmetic type
                        if exprVec[0].getExpressionType() != exprVec[1].getExpressionType() {
                            resolutionHandler.errorVec.push(ResolutionError::ExpectedType(exprVec[0].getExpressionType(), exprVec[1].getExpressionType(), format!("mismatched types for operation expression")));
                            return None;
                        }

                        if !exprVec[0].getExpressionType().isArithmeticType() {
                            resolutionHandler.errorVec.push(ResolutionError::InvalidOperationType(exprVec[0].getExpressionType(), format!("cannot apply {:?} operator to non-arithmetic type", expr.operator)));
                            return None;
                        }

                        exprVec[0].getExpressionType()
                    }
                    Operator::ModAssign | Operator::DivAssign | Operator::MultAssign | Operator::MinusAssign | Operator::PlusAssign => {
                        // arithmetic type
                        if exprVec[0].getExpressionType() != exprVec[1].getExpressionType() {
                            resolutionHandler.errorVec.push(ResolutionError::ExpectedType(exprVec[0].getExpressionType(), exprVec[1].getExpressionType(), format!("mismatched types for operation expression")));
                            return None;
                        }

                        if !exprVec[0].getExpressionType().isArithmeticType() {
                            resolutionHandler.errorVec.push(ResolutionError::InvalidOperationType(exprVec[0].getExpressionType(), format!("cannot apply {:?} operator to non-arithmetic type", expr.operator)));
                            return None;
                        }

                        if !exprVec[0].getResolvedExprType().isAssignable() {
                            resolutionHandler.errorVec.push(ResolutionError::InvalidOperation(format!("value is not assignable")));
                            return None;
                        }

                        if matches!(exprVec[0], ResolvedExpr::VariableDeclaration(_)) {
                            resolutionHandler.errorVec.push(ResolutionError::InvalidOperation(format!("cannot apply operator {:?} to variable declaration", expr.operator)));
                            return None;
                        }

                        exprVec[0].getExpressionType()
                    }
                    Operator::Increment | Operator::Decrement | Operator::Mod => {
                        // integer
                        let exprType = exprVec[0].getExpressionType();
                        if exprType != INTEGER_TYPE.to_owned() {
                            resolutionHandler.errorVec.push(ResolutionError::ExpectedType(INTEGER_TYPE.to_owned(), exprType, format!("expected integer for operator {:?}", expr.operator)));
                            return None;
                        }
                        exprType
                    }
                    Operator::And | Operator::Or | Operator::Not => {
                        // bool
                        for operand in &exprVec {
                            if operand.getExpressionType() != BOOLEAN_TYPE.to_owned() {
                                resolutionHandler.errorVec.push(ResolutionError::ExpectedType(BOOLEAN_TYPE.to_owned(), operand.getExpressionType(), format!("operator {:?} must be applied to a boolean expression", expr.operator)));
                                return None;
                            }
                        }
                        BOOLEAN_TYPE.to_owned()
                    }
                    Operator::Dot => unreachable!(),
                    Operator::AssignEq => {
                        // any type
                        if exprVec[0].getExpressionType() != exprVec[1].getExpressionType() {
                            resolutionHandler.errorVec.push(ResolutionError::ExpectedType(exprVec[0].getExpressionType(), exprVec[1].getExpressionType(), format!("mismatched types for assignment")));
                            return None;
                        }

                        if !exprVec[0].getResolvedExprType().isAssignable() {
                            resolutionHandler.errorVec.push(ResolutionError::InvalidOperation(format!("value is not assignable")));
                            return None;
                        }
                        exprVec[0].getExpressionType()
                    }
                    Operator::Cast | Operator::Range | Operator::Ellipsis | Operator::Colon | Operator::ErrorPropagation => {
                        resolutionHandler.errorVec.push(ResolutionError::Unsupported(expr.range.getStartPos(), format!("unsupported operator {:?}", expr.operator)));
                        return None;
                    }
                };

                ResolvedExpr::Operator(Box::new(ResolvedOperator {
                    operator: expr.operator,
                    operands: exprVec.into_boxed_slice(),
                    expressionType,
                }))
            }
        }
        Expr::VariableDeclaration(expr) => {
            if let Some(explicitType) = &expr.explicitType {
                getResolvedType(resolutionHandler, explicitType, |resolutionHandler, ty| {
                    let variable = resolutionHandler.scope.declareVariable(expr.variableName.getToken().getSourceRange().getSourceInRange(), ty, &mut resolutionHandler.errorVec)?;
                    Some(ResolvedExpr::VariableDeclaration(VariableDeclare {
                        ty: variable.ty,
                        id: variable.id,
                        global,
                    }))
                }).flatten()?
            } else {
                resolutionHandler.errorVec.push(ResolutionError::Unsupported(expr.range.getStartPos(), format!("type inference not supported (for variable '{}')", expr.variableName.getToken().getSourceRange().getSourceInRange())));
                return None;
            }
        }
        Expr::Variable(expr) => {
            let variableName = &expr.range.getSource();
            ResolvedExpr::Variable(resolutionHandler.scope.getVariableOrError(variableName, &mut resolutionHandler.errorVec)?)
        }
        Expr::LiteralBool(expr) => {
            ResolvedExpr::LiteralBool(expr.value)
        }
        Expr::LiteralChar(expr) => {
            ResolvedExpr::LiteralChar(expr.value)
        }
        Expr::LiteralFloat(expr) => {
            ResolvedExpr::LiteralFloat(expr.value)
        }
        Expr::LiteralInteger(expr) => {
            ResolvedExpr::LiteralInteger(expr.value)
        }
        Expr::LiteralString(expr) => {
            ResolvedExpr::LiteralString(expr.fileRange.getSourceInRange().to_owned())
        }
        Expr::LiteralArray(expr) => {
            resolutionHandler.errorVec.push(ResolutionError::Unsupported(expr.range.getStartPos(), "unsupported array literal".to_owned()));
            return None;
        }
        Expr::LiteralTuple(expr) => {
            resolutionHandler.errorVec.push(ResolutionError::Unsupported(expr.range.getStartPos(), "unsupported tuple literal".to_owned()));
            return None;
        }
    };

    return Some(callback(resolutionHandler, resolved));
}
