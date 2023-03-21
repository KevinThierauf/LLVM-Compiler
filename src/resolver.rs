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
use crate::resolver::typeinfo::primitive::integer::INTEGER_TYPE;
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

        enum Resolution {
            Ok(Statement),
            Err,
            Parent,
        }

        fn getResolvedType<R>(exportTable: &Arc<CompleteExportTable>, errorVec: &mut Vec<ResolutionError>, modulePos: &ModulePos, callback: impl FnOnce(&Arc<CompleteExportTable>, &mut Vec<ResolutionError>, Type) -> R) -> R {
            todo!()
        }

        fn getResolvedExpression<'a, R>(resolver: &mut dyn ResolutionContextType, exportTable: &Arc<CompleteExportTable>, errorVec: &mut Vec<ResolutionError>, expr: &Expr, callback: Box<dyn 'a + FnOnce(&mut dyn ResolutionContextType, &Arc<CompleteExportTable>, &mut Vec<ResolutionError>, ResolvedExpr) -> R>) -> Option<R> {
            let resolved = match expr {
                Expr::FunctionCall(expr) => {
                    match exportTable.getExportedFunction(expr.functionName.getToken().getSourceRange().getSourceInRange()) {
                        Ok(function) => {
                            if expr.argVec.len() == function.parameters.len() {
                                let mut argVec = Vec::new();
                                for index in 0..expr.argVec.len() {
                                    getResolvedExpression(resolver, exportTable, errorVec, &expr.argVec[index], Box::new(|_, _, errorVec, expression| {
                                        if expression.getExpressionType() == function.parameters[index].ty {
                                            argVec.push(expression);
                                        } else {
                                            errorVec.push(ResolutionError::ExpectedType(function.parameters[index].ty.to_owned(), expression.getExpressionType(), format!("parameter type incorrect in function call")));
                                        }
                                    }));
                                }

                                if argVec.len() == expr.argVec.len() {
                                    ResolvedExpr::FunctionCall(Box::new(FunctionCall {
                                        function,
                                        argVec,
                                    }))
                                } else {
                                    return None;
                                }
                            } else {
                                errorVec.push(ResolutionError::ParameterMismatch(function.to_owned(), format!("parameter mismatch: expected {} args, found {}", function.parameters.len(), expr.argVec.len())));
                                return None;
                            }
                        }
                        Err(error) => {
                            errorVec.push(error);
                            return None;
                        }
                    }
                }
                Expr::Operator(expr) => {
                    let mut exprVec = Vec::new();

                    for expr in expr.operands.iter().map(|expr| getResolvedExpression(resolver, exportTable, errorVec, expr, Box::new(|resolver, exportTable, errorVec, resolved| resolved))) {
                        exprVec.push(expr?);
                    }

                    let expressionType = match expr.operator {
                        Operator::Greater | Operator::Less | Operator::GreaterEq | Operator::LessEq | Operator::CompareEq | Operator::CompareNotEq | Operator::ModAssign | Operator::DivAssign | Operator::Div | Operator::MultAssign | Operator::MinusAssign | Operator::PlusAssign | Operator::Plus | Operator::Minus | Operator::Mult => {
                            // arithmetic type
                            if exprVec[0].getExpressionType() != exprVec[1].getExpressionType() {
                                errorVec.push(ResolutionError::ExpectedType(exprVec[0].getExpressionType(), exprVec[1].getExpressionType(), format!("mismatched types for operation expression")));
                                return None;
                            }

                            if !exprVec[0].getExpressionType().isArithmeticType() {
                                errorVec.push(ResolutionError::InvalidOperationType(exprVec[0].getExpressionType(), format!("cannot apply {:?} operator to non-arithmetic type", expr.operator)));
                                return None;
                            }
                            exprVec[0].getExpressionType()
                        }
                        Operator::Increment | Operator::Decrement | Operator::Mod => {
                            // integer
                            let exprType = exprVec[0].getExpressionType();
                            if exprType != INTEGER_TYPE.to_owned() {
                                errorVec.push(ResolutionError::ExpectedType(INTEGER_TYPE.to_owned(), exprType, format!("expected integer for operator {:?}", expr.operator)));
                                return None;
                            }
                            exprType
                        }
                        Operator::And | Operator::Or | Operator::Not => {
                            // bool
                            for operand in &exprVec {
                                if operand.getExpressionType() != BOOLEAN_TYPE.to_owned() {
                                    errorVec.push(ResolutionError::ExpectedType(BOOLEAN_TYPE.to_owned(), operand.getExpressionType(), format!("operator {:?} must be applied to a boolean expression", expr.operator)));
                                    return None;
                                }
                            }
                            BOOLEAN_TYPE.to_owned()
                        }
                        Operator::Dot => {
                            match exprVec[1] {
                                ResolvedExpr::FunctionCall(_) => {}
                                ResolvedExpr::Variable(_) => {}
                                _ => {
                                    errorVec.push(ResolutionError::InvalidOperation(format!("dot operator can only be used to access a variable or function, found {:?}", exprVec[1])));
                                    return None;
                                }
                            }
                            todo!()
                        }
                        Operator::AssignEq => {
                            // any type
                            if exprVec[0].getExpressionType() != exprVec[1].getExpressionType() {
                                errorVec.push(ResolutionError::ExpectedType(exprVec[0].getExpressionType(), exprVec[1].getExpressionType(), format!("mismatched types for assignment")));
                                return None;
                            }

                            if !exprVec[0].getResolvedExprType().isAssignable() {
                                errorVec.push(ResolutionError::InvalidOperation(format!("value is not assignable")));
                                return None;
                            }
                            exprVec[0].getExpressionType()
                        }
                        Operator::Cast | Operator::Range | Operator::Ellipsis | Operator::Colon | Operator::ErrorPropagation => {
                            errorVec.push(ResolutionError::Unsupported(expr.range.getStartPos(), format!("unsupported operator {:?}", expr.operator)));
                            return None;
                        }
                    };

                    ResolvedExpr::Operator(Box::new(ResolvedOperator {
                        operator: expr.operator,
                        operands: exprVec.into_boxed_slice(),
                        expressionType,
                    }))
                }
                Expr::VariableDeclaration(expr) => {
                    if let Some(explicitType) = &expr.explicitType {
                        getResolvedType(&exportTable, errorVec, explicitType, |_, _, ty| Some(ResolvedExpr::VariableDeclaration(resolver.declareVariable(expr.variableName.getToken().getSourceRange().getSourceInRange(), ty)?)))?
                    } else {
                        errorVec.push(ResolutionError::UnresolvedType(expr.range.getStartPos(), format!("unable to determine type for variable {}", expr.variableName.getToken().getSourceRange().getSourceInRange())));
                        return None;
                    }
                }
                Expr::Variable(expr) => {
                    todo!()
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
                Expr::LiteralVoid(_) => {
                    ResolvedExpr::LiteralVoid
                }
                Expr::LiteralArray(expr) => {
                    errorVec.push(ResolutionError::Unsupported(expr.range.getStartPos(), "unsupported array literal".to_owned()));
                    return None;
                }
                Expr::LiteralTuple(expr) => {
                    errorVec.push(ResolutionError::Unsupported(expr.range.getStartPos(), "unsupported tuple literal".to_owned()));
                    return None;
                }
            };

            return Some(callback(resolver, exportTable, errorVec, resolved));
        }

        trait ResolutionContextType {
            fn declareVariable(&mut self, name: &str, ty: Type) -> Option<VariableDeclare>;
            fn implHandleSymbol(&mut self, exportTable: &Arc<CompleteExportTable>, errorVec: &mut Vec<ResolutionError>, symbol: &Symbol) -> Resolution;
            fn getDynMut(&mut self) -> &mut dyn ResolutionContextType;

            fn handleSymbol(&mut self, exportTable: &Arc<CompleteExportTable>, errorVec: &mut Vec<ResolutionError>, symbol: &Symbol) -> Option<Statement> {
                match self.implHandleSymbol(exportTable, errorVec, symbol) {
                    Resolution::Ok(symbol) => return Some(symbol),
                    Resolution::Err => return None,
                    Resolution::Parent => {
                        // continue
                    }
                };

                return match symbol {
                    Symbol::Block(symbol) => {
                        // todo!()
                        return None;
                    }
                    Symbol::While(symbol) => {
                        return getResolvedExpression(self.getDynMut(), &exportTable, errorVec, &symbol.condition, Box::new(|resolver, exportTable, errorVec, expr| {
                            return if expr.getExpressionType() == BOOLEAN_TYPE.to_owned() {
                                let statement = resolver.handleSymbol(exportTable, errorVec, symbol.symbol.deref())?;
                                Some(Statement::While(Box::new(WhileStatement {
                                    condition: expr,
                                    statement,
                                })))
                            } else {
                                errorVec.push(ResolutionError::ExpectedType(BOOLEAN_TYPE.to_owned(), expr.getExpressionType(), format!("expected boolean conditional for while loop")));
                                None
                            };
                        })).flatten();
                    }
                    Symbol::IfSym(symbol) => {
                        return getResolvedExpression(self.getDynMut(), &exportTable, errorVec, &symbol.condition, Box::new(|resolver, exportTable, errorVec, expr| {
                            return if expr.getExpressionType() == BOOLEAN_TYPE.to_owned() {
                                let statement = resolver.handleSymbol(exportTable, errorVec, symbol.symbol.deref())?;
                                Some(Statement::If(Box::new(IfStatement {
                                    condition: expr,
                                    statement,
                                })))
                            } else {
                                errorVec.push(ResolutionError::ExpectedType(BOOLEAN_TYPE.to_owned(), expr.getExpressionType(), format!("expected boolean conditional for if statement")));
                                None
                            };
                        })).flatten();
                    }
                    Symbol::Expr(expr) => {
                        Some(Statement::Expr(getResolvedExpression(self.getDynMut(), exportTable, errorVec, expr, Box::new(|_, _, _, resolved| resolved))?))
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

        struct TopLevelResolutionContext;

        impl ResolutionContextType for TopLevelResolutionContext {
            fn declareVariable(&mut self, name: &str, ty: Type) -> Option<VariableDeclare> {
                todo!()
            }

            fn implHandleSymbol(&mut self, exportTable: &Arc<CompleteExportTable>, errorVec: &mut Vec<ResolutionError>, symbol: &Symbol) -> Resolution {
                return match symbol {
                    Symbol::ClassDefinition(symbol) => {
                        todo!()
                    }
                    Symbol::FunctionDefinition(symbol) => {
                        todo!()
                    }
                    _ => Resolution::Parent,
                };
            }

            fn getDynMut(&mut self) -> &mut dyn ResolutionContextType {
                return self;
            }
        }

        struct FunctionDefinitionResolutionContext;

        impl ResolutionContextType for FunctionDefinitionResolutionContext {
            fn declareVariable(&mut self, name: &str, ty: Type) -> Option<VariableDeclare> {
                todo!()
            }

            fn implHandleSymbol(&mut self, exportTable: &Arc<CompleteExportTable>, errorVec: &mut Vec<ResolutionError>, symbol: &Symbol) -> Resolution {
                return match symbol {
                    Symbol::Return(symbol) => {
                        todo!()
                    }
                    _ => Resolution::Parent,
                };
            }

            fn getDynMut(&mut self) -> &mut dyn ResolutionContextType {
                return self;
            }
        }

        let mut resolutionContext = ResolutionContext {
            resolver: Box::new(TopLevelResolutionContext),
        };

        resolutionContext.handleSymbols(&exportTable, &mut errorVec, self.ast.getSymbols().iter());

        return if errorVec.is_empty() {
            Ok(resolved)
        } else {
            Err(errorVec)
        };
    }
}
