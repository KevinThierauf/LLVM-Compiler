use std::ffi::CString;

use llvm_sys::core::{LLVMBuildAdd, LLVMBuildAnd, LLVMBuildExactSDiv, LLVMBuildFAdd, LLVMBuildFDiv, LLVMBuildFMul, LLVMBuildFSub, LLVMBuildMul, LLVMBuildNot, LLVMBuildOr, LLVMBuildSRem, LLVMBuildSub, LLVMConstArray, LLVMConstInt, LLVMConstReal, LLVMFloatType, LLVMInt1Type, LLVMInt32Type, LLVMInt8Type};
use llvm_sys::prelude::{LLVMBool, LLVMBuilderRef, LLVMValueRef};

use crate::module::Operator;
use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::resolvedoperator::ResolvedOperator;
use crate::resolver::resolvedast::resolvedvariable::ResolvedVariable;
use crate::resolver::resolvedast::statement::Statement;
use crate::resolver::typeinfo::primitive::float::FLOAT_TYPE;
use crate::resolver::typeinfo::primitive::integer::INTEGER_TYPE;

unsafe fn getOperands(builder: LLVMBuilderRef, operands: Vec<ResolvedExpr>) -> Vec<LLVMValueRef> {
    return operands.into_iter().map(|expr| emitExpr(builder, expr)).collect::<Vec<_>>();
}

fn expectVariable(resolvedExpr: ResolvedExpr) -> ResolvedVariable {
    return if let ResolvedExpr::Variable(variable) = resolvedExpr {
        variable
    } else {
        unreachable!("expected variable");
    };
}

unsafe fn emitOperatorAssign(builder: LLVMBuilderRef, mut operands: Vec<ResolvedExpr>, operator: Operator) -> LLVMValueRef {
    let value = operands.remove(1);
    let variable = expectVariable(operands.remove(0));
    return emitExpr(builder, ResolvedExpr::Operator(Box::new(ResolvedOperator {
        operator: Operator::AssignEq,
        operands: Box::new([ResolvedExpr::Variable(variable.to_owned()), ResolvedExpr::Operator(Box::new(ResolvedOperator {
            operator,
            operands: Box::new([ResolvedExpr::Variable(variable), value]),
            expressionType: INTEGER_TYPE.to_owned(),
        }))]),
        expressionType: INTEGER_TYPE.to_owned(),
    })));
}

unsafe fn emitExpr(builder: LLVMBuilderRef, expr: ResolvedExpr) -> LLVMValueRef {
    return match expr {
        ResolvedExpr::Operator(expr) => {
            let mut operands = Vec::from(expr.operands);
            debug_assert_eq!(operands.len(), expr.operator.getOperands());
            let name = CString::new("operator").unwrap();
            let name = name.as_ptr();

            match expr.operator {
                Operator::Increment => {
                    if expr.expressionType == INTEGER_TYPE {
                        emitExpr(builder, ResolvedExpr::Operator(Box::new(ResolvedOperator {
                            operator: Operator::PlusAssign,
                            operands: Box::new([operands.remove(0), ResolvedExpr::LiteralInteger(1)]),
                            expressionType: INTEGER_TYPE.to_owned(),
                        })))
                    } else {
                        panic!("unexpected increment type {:?}", expr.expressionType)
                    }
                }
                Operator::Decrement => {
                    if expr.expressionType == INTEGER_TYPE {
                        emitExpr(builder, ResolvedExpr::Operator(Box::new(ResolvedOperator {
                            operator: Operator::DivAssign,
                            operands: Box::new([operands.remove(0), ResolvedExpr::LiteralInteger(1)]),
                            expressionType: INTEGER_TYPE.to_owned(),
                        })))
                    } else {
                        panic!("unexpected increment type {:?}", expr.expressionType)
                    }
                }
                Operator::Not => {
                    let operands = getOperands(builder, operands);
                    LLVMBuildNot(builder, operands[0], name)
                }
                Operator::Plus => {
                    let operands = getOperands(builder, operands);
                    if expr.expressionType == INTEGER_TYPE {
                        LLVMBuildAdd(builder, operands[0], operands[1], name)
                    } else if expr.expressionType == FLOAT_TYPE {
                        LLVMBuildFAdd(builder, operands[0], operands[1], name)
                    } else {
                        panic!("unknown arithmetic type {:?}", expr.expressionType);
                    }
                }
                Operator::Minus => {
                    let operands = getOperands(builder, operands);
                    if expr.expressionType == INTEGER_TYPE {
                        LLVMBuildSub(builder, operands[0], operands[1], name)
                    } else if expr.expressionType == FLOAT_TYPE {
                        LLVMBuildFSub(builder, operands[0], operands[1], name)
                    } else {
                        panic!("unknown arithmetic type {:?}", expr.expressionType);
                    }
                }
                Operator::Mult => {
                    let operands = getOperands(builder, operands);
                    if expr.expressionType == INTEGER_TYPE {
                        LLVMBuildMul(builder, operands[0], operands[1], name)
                    } else if expr.expressionType == FLOAT_TYPE {
                        LLVMBuildFMul(builder, operands[0], operands[1], name)
                    } else {
                        panic!("unknown arithmetic type {:?}", expr.expressionType);
                    }
                }
                Operator::Div => {
                    let operands = getOperands(builder, operands);
                    if expr.expressionType == INTEGER_TYPE {
                        LLVMBuildExactSDiv(builder, operands[0], operands[1], name)
                    } else if expr.expressionType == FLOAT_TYPE {
                        LLVMBuildFDiv(builder, operands[0], operands[1], name)
                    } else {
                        panic!("unknown arithmetic type {:?}", expr.expressionType);
                    }
                }
                Operator::Mod => {
                    let operands = getOperands(builder, operands);
                    if expr.expressionType == INTEGER_TYPE {
                        LLVMBuildSRem(builder, operands[0], operands[1], name)
                    } else {
                        panic!("unknown type for mod {:?}", expr.expressionType);
                    }
                }
                Operator::PlusAssign => {
                    emitOperatorAssign(builder, operands, Operator::Plus)
                }
                Operator::MinusAssign => {
                    emitOperatorAssign(builder, operands, Operator::Minus)
                }
                Operator::MultAssign => {
                    emitOperatorAssign(builder, operands, Operator::Mult)
                }
                Operator::DivAssign => {
                    emitOperatorAssign(builder, operands, Operator::Div)
                }
                Operator::ModAssign => {
                    emitOperatorAssign(builder, operands, Operator::Mod)
                }
                Operator::And => {
                    let operands = getOperands(builder, operands);
                    LLVMBuildAnd(builder, operands[0], operands[1], name)
                }
                Operator::Or => {
                    let operands = getOperands(builder, operands);
                    LLVMBuildOr(builder, operands[0], operands[1], name)
                }
                Operator::Greater => {
                    todo!()
                }
                Operator::Less => {
                    todo!()
                }
                Operator::GreaterEq => {
                    todo!()
                }
                Operator::LessEq => {
                    todo!()
                }
                Operator::CompareEq => {
                    todo!()
                }
                Operator::CompareNotEq => {
                    todo!()
                }
                Operator::AssignEq => {
                    todo!()
                }
                Operator::Cast | Operator::Dot | Operator::Range | Operator::Ellipsis | Operator::Colon | Operator::ErrorPropagation => {
                    // should have been previously handled/removed
                    unreachable!()
                }
            }
        }
        ResolvedExpr::FunctionCall(expr) => {
            todo!()
        }
        ResolvedExpr::ConstructorCall(expr) => {
            todo!()
        }
        ResolvedExpr::VariableDeclaration(expr) => {
            todo!()
        }
        ResolvedExpr::Variable(expr) => {
            todo!()
        }
        ResolvedExpr::Property(expr) => {
            todo!()
        }
        ResolvedExpr::LiteralBool(expr) => {
            LLVMConstInt(LLVMInt1Type(), if expr { 1 } else { 0 }, LLVMBool::from(false))
        }
        ResolvedExpr::LiteralChar(expr) => {
            LLVMConstInt(LLVMInt8Type(), expr as _, LLVMBool::from(true))
        }
        ResolvedExpr::LiteralFloat(expr) => {
            LLVMConstReal(LLVMFloatType(), 0.0)
        }
        ResolvedExpr::LiteralInteger(expr) => {
            LLVMConstInt(LLVMInt32Type(), expr as _, LLVMBool::from(true))
        }
        ResolvedExpr::LiteralString(expr) => {
            LLVMConstArray(LLVMInt8Type(), expr.chars().into_iter().map(|c| emitExpr(builder, ResolvedExpr::LiteralChar(c as _))).collect::<Vec<_>>().as_mut_ptr(), expr.len() as _)
        }
        ResolvedExpr::LiteralVoid => {
            todo!()
        }
    };
}

pub(in super) unsafe fn emit(builder: LLVMBuilderRef, statement: Statement) -> LLVMValueRef {
    return match statement {
        Statement::If(statement) => {
            todo!()
        }
        Statement::While(statement) => {
            todo!()
        }
        Statement::Return(statement) => {
            todo!()
        }
        Statement::Expr(expr) => {
            emitExpr(builder, expr)
        }
        Statement::FunctionDefinition(statement) => {
            todo!()
        }
        Statement::Scope(statement) => {
            todo!()
        }
        Statement::Multiple(statementVec) => {
            todo!()
        }
    };
}
