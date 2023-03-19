use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use crate::ast::AbstractSyntaxTree;
use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::symbol::expr::Expr;
use crate::module::modulepos::ModulePos;
use crate::module::Operator;
use crate::resolver::exporttable::completeexporttable::CompleteExportTable;
use crate::resolver::exporttable::GlobalExportTable;
use crate::resolver::exporttable::incompleteexporttable::IncompleteExportTable;
use crate::resolver::resolutionerror::ResolutionError;
use crate::resolver::resolvedast::functioncall::FunctionCall;
use crate::resolver::resolvedast::ifstatement::IfStatement;
use crate::resolver::resolvedast::ResolvedAST;
use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::resolvedoperator::ResolvedOperator;
use crate::resolver::resolvedast::statement::Statement;
use crate::resolver::resolvedast::variabledeclare::VariableDeclare;
use crate::resolver::resolvedast::whilestatement::WhileStatement;
use crate::resolver::typeinfo::primitive::boolean::BOOLEAN_TYPE;
use crate::resolver::typeinfo::Type;

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
        let mut resolved = ResolvedAST::new();
        let mut errorVec = Vec::new();

        fn getResolvedType<R>(exportTable: &Arc<CompleteExportTable>, errorVec: &mut Vec<ResolutionError>, modulePos: &ModulePos, callback: impl FnOnce(&Arc<CompleteExportTable>, &mut Vec<ResolutionError>, Type) -> R) -> R {
            todo!()
        }

        fn getResolvedExpression<R>(resolver: &mut dyn ResolutionContextType, exportTable: &Arc<CompleteExportTable>, errorVec: &mut Vec<ResolutionError>, expr: &Expr, callback: impl FnOnce(&mut dyn ResolutionContextType, &Arc<CompleteExportTable>, &mut Vec<ResolutionError>, ResolvedExpr) -> R) -> R {
            todo!()
        }

        trait ResolutionContextType {
            fn declareVariable(&mut self, name: &str, ty: Type) -> Option<VariableDeclare>;
            fn implHandleSymbol(&mut self, exportTable: &Arc<CompleteExportTable>, errorVec: &mut Vec<ResolutionError>, symbol: &Symbol) -> Option<Statement>;
            fn getDynMut(&mut self) -> &mut dyn ResolutionContextType;

            fn handleSymbol(&mut self, exportTable: &Arc<CompleteExportTable>, errorVec: &mut Vec<ResolutionError>, symbol: &Symbol) -> Option<Statement> {
                if let Some(symbol) = self.implHandleSymbol(exportTable, errorVec, symbol) {
                    return Some(symbol);
                }

                return match symbol {
                    Symbol::Block(symbol) => {
                        // todo!()
                        return None;
                    }
                    Symbol::While(symbol) => {
                        return getResolvedExpression(self.getDynMut(), &exportTable, errorVec, &symbol.condition, |resolver, exportTable, errorVec, expr| {
                            return if expr.getExpressionType() == BOOLEAN_TYPE.to_owned() {
                                let statement = resolver.handleSymbol(exportTable, errorVec, symbol.symbol.deref())?;
                                Some(Statement::While(Box::new(WhileStatement {
                                    condition: expr,
                                    statement,
                                })))
                            } else {
                                errorVec.push(ResolutionError::ExpectedType(BOOLEAN_TYPE.to_owned(), expr.getExpressionType()));
                                None
                            };
                        });
                    }
                    Symbol::IfSym(symbol) => {
                        return getResolvedExpression(self.getDynMut(), &exportTable, errorVec, &symbol.condition, |resolver, exportTable, errorVec, expr| {
                            return if expr.getExpressionType() == BOOLEAN_TYPE.to_owned() {
                                let statement = resolver.handleSymbol(exportTable, errorVec, symbol.symbol.deref())?;
                                Some(Statement::If(Box::new(IfStatement {
                                    condition: expr,
                                    statement,
                                })))
                            } else {
                                errorVec.push(ResolutionError::ExpectedType(BOOLEAN_TYPE.to_owned(), expr.getExpressionType()));
                                None
                            };
                        });
                    }
                    Symbol::FunctionCall(symbol) => {
                        match exportTable.getExportedFunction(symbol.functionName.getToken().getSourceRange().getSourceInRange()) {
                            Ok(function) => {
                                if symbol.argVec.len() == function.parameters.len() {
                                    let mut argVec = Vec::new();
                                    for index in 0..symbol.argVec.len() {
                                        getResolvedExpression(self.getDynMut(), exportTable, errorVec, &symbol.argVec[index], |resolver, exportTable, errorVec, expression| {
                                            if expression.getExpressionType() == function.parameters[index].ty {
                                                argVec.push(expression);
                                            } else {
                                                errorVec.push(ResolutionError::ExpectedType(function.parameters[index].ty.to_owned(), expression.getExpressionType()));
                                            }
                                        });
                                    }

                                    return if argVec.len() == symbol.argVec.len() {
                                        Some(Statement::Expr(ResolvedExpr::FunctionCall(Box::new(FunctionCall {
                                            function,
                                            argVec,
                                        }))))
                                    } else {
                                        None
                                    };
                                } else {
                                    errorVec.push(ResolutionError::ParameterMismatch(function.to_owned(), format!("parameter mismatch: expected {} args, found {}", function.parameters.len(), symbol.argVec.len())));
                                    None
                                }
                            }
                            Err(error) => {
                                errorVec.push(error);
                                None
                            }
                        }
                    }
                    Symbol::Operator(symbol) => {
                        let mut exprVec: Vec<ResolvedExpr> = Vec::new();
                        for expr in symbol.operands.deref() {
                            let s = getResolvedExpression(self.getDynMut(), exportTable, errorVec, expr, |resolver, exportTable, errorVec, resolved| {
                                if let Some(last) = exprVec.last() {
                                    if last.getExpressionType() != resolved.getExpressionType() {
                                        errorVec.push(ResolutionError::ExpectedType(last.getExpressionType(), resolved.getExpressionType()));
                                        return false;
                                    }
                                }
                                exprVec.push(resolved);
                                return true;
                            });
                            if !s {
                                return None;
                            }
                        }

                        let exprType = exprVec.last().unwrap().getExpressionType();

                        match symbol.operator {
                            Operator::Increment | Operator::Decrement => {}
                            Operator::Not => {}
                            Operator::Dot => {}
                            Operator::Range | Operator::Ellipsis | Operator::Colon | Operator::ErrorPropagation => {}
                            Operator::Cast => {}
                            Operator::Plus => {}
                            Operator::Minus => {}
                            Operator::Mult => {}
                            Operator::Div => {}
                            Operator::Mod => {}
                            Operator::PlusAssign => {}
                            Operator::MinusAssign => {}
                            Operator::MultAssign => {}
                            Operator::DivAssign => {}
                            Operator::ModAssign => {}
                            Operator::And => {}
                            Operator::Or => {}
                            Operator::Greater => {}
                            Operator::Less => {}
                            Operator::GreaterEq => {}
                            Operator::LessEq => {}
                            Operator::CompareEq => {}
                            Operator::CompareNotEq => {}
                            Operator::AssignEq => {}
                        }

                        return Some(Statement::Expr(ResolvedExpr::Operator(Box::new(ResolvedOperator {
                            operator: symbol.operator,
                            operands: exprVec.into_boxed_slice(),
                        }))));
                    }
                    Symbol::VariableDeclaration(symbol) => {
                        return if let Some(explicitType) = &symbol.explicitType {
                            getResolvedType(&exportTable, errorVec, explicitType, |_, _, ty| Some(Statement::Expr(ResolvedExpr::VariableDeclaration(self.declareVariable(symbol.variableName.getToken().getSourceRange().getSourceInRange(), ty)?))))
                        } else {
                            errorVec.push(ResolutionError::UnresolvedType(symbol.range.getStartPos(), format!("unable to determine type for variable {}", symbol.variableName.getToken().getSourceRange().getSourceInRange())));
                            None
                        };
                    }
                    Symbol::ClassDefinition(symbol) => {
                        errorVec.push(ResolutionError::Unexpected(symbol.getRange().getStartPos(), "unexpected class definition".to_owned()));
                        return None;
                    }
                    Symbol::Return(symbol) => {
                        errorVec.push(ResolutionError::Unexpected(symbol.getRange().getStartPos(), "unexpected return statement".to_owned()));
                        return None;
                    }
                    Symbol::FunctionDefinition(symbol) => {
                        errorVec.push(ResolutionError::Unexpected(symbol.getRange().getStartPos(), "unexpected function definition".to_owned()));
                        return None;
                    }
                    Symbol::Variable(symbol) => {
                        errorVec.push(ResolutionError::Unexpected(symbol.getRange().getStartPos(), "unexpected variable".to_owned()));
                        return None;
                    }
                    Symbol::LiteralBool(symbol) => {
                        errorVec.push(ResolutionError::Unexpected(symbol.getRange().getStartPos(), "unexpected bool literal".to_owned()));
                        return None;
                    }
                    Symbol::LiteralChar(symbol) => {
                        errorVec.push(ResolutionError::Unexpected(symbol.getRange().getStartPos(), "unexpected char literal".to_owned()));
                        return None;
                    }
                    Symbol::LiteralFloat(symbol) => {
                        errorVec.push(ResolutionError::Unexpected(symbol.getRange().getStartPos(), "unexpected float literal".to_owned()));
                        return None;
                    }
                    Symbol::LiteralInteger(symbol) => {
                        errorVec.push(ResolutionError::Unexpected(symbol.getRange().getStartPos(), "unexpected integer literal".to_owned()));
                        return None;
                    }
                    Symbol::LiteralString(symbol) => {
                        errorVec.push(ResolutionError::Unexpected(symbol.getRange().getStartPos(), "unexpected string literal".to_owned()));
                        return None;
                    }
                    Symbol::LiteralVoid(symbol) => {
                        errorVec.push(ResolutionError::Unexpected(symbol.getRange().getStartPos(), "unexpected void literal".to_owned()));
                        return None;
                    }
                    Symbol::Break(symbol) => {
                        errorVec.push(ResolutionError::Unsupported(symbol.range.getStartPos(), "break".to_owned()));
                        return None;
                    }
                    Symbol::Continue(symbol) => {
                        errorVec.push(ResolutionError::Unsupported(symbol.range.getStartPos(), "continue".to_owned()));
                        return None;
                    }
                    Symbol::ImportSym(symbol) => {
                        errorVec.push(ResolutionError::Unsupported(symbol.range.getStartPos(), "import".to_owned()));
                        return None;
                    }
                    Symbol::LiteralArray(symbol) => {
                        errorVec.push(ResolutionError::Unsupported(symbol.range.getStartPos(), "array literal".to_owned()));
                        return None;
                    }
                    Symbol::LiteralTuple(symbol) => {
                        errorVec.push(ResolutionError::Unsupported(symbol.range.getStartPos(), "tuple literal".to_owned()));
                        return None;
                    }
                };
            }
        }

        struct ResolutionContext {
            resolver: Box<dyn ResolutionContextType>,
        }

        impl ResolutionContext {
            fn handleSymbols<'a>(&mut self, exportTable: &Arc<CompleteExportTable>, errorVec: &mut Vec<ResolutionError>, iterator: impl Iterator<Item = &'a Symbol>) {
                for symbol in iterator {
                    if let Some(statement) = self.resolver.handleSymbol(exportTable, errorVec, symbol) {
                        // todo
                    }
                }
            }
        }

        for symbol in self.ast.getSymbols() {}

        return if errorVec.is_empty() {
            Ok(resolved)
        } else {
            Err(errorVec)
        };
    }
}
