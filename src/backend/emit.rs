use std::ffi::CString;

use llvm_sys::core::{LLVMBuildAdd, LLVMBuildAnd, LLVMBuildExactSDiv, LLVMBuildFAdd, LLVMBuildFDiv, LLVMBuildFMul, LLVMBuildFSub, LLVMBuildICmp, LLVMBuildMul, LLVMBuildNot, LLVMBuildOr, LLVMBuildSRem, LLVMBuildSub, LLVMConstArray, LLVMConstInt, LLVMConstReal, LLVMFloatType, LLVMInt1Type, LLVMInt32Type, LLVMInt8Type};
use llvm_sys::LLVMIntPredicate;
use llvm_sys::prelude::{LLVMBool, LLVMValueRef};

use crate::backend::CompiledModule;
use crate::module::Operator;
use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::resolvedoperator::ResolvedOperator;
use crate::resolver::resolvedast::resolvedvariable::ResolvedVariable;
use crate::resolver::resolvedast::statement::Statement;
use crate::resolver::typeinfo::primitive::float::FLOAT_TYPE;
use crate::resolver::typeinfo::primitive::integer::INTEGER_TYPE;

unsafe fn getOperands(module: &mut CompiledModule, operands: Vec<ResolvedExpr>) -> Vec<LLVMValueRef> {
    return operands.into_iter().map(|expr| emitExpr(module, expr)).collect::<Vec<_>>();
}

fn expectVariable(resolvedExpr: ResolvedExpr) -> ResolvedVariable {
    return if let ResolvedExpr::Variable(variable) = resolvedExpr {
        variable
    } else {
        unreachable!("expected variable");
    };
}

unsafe fn emitOperatorAssign(module: &mut CompiledModule, mut operands: Vec<ResolvedExpr>, operator: Operator) -> LLVMValueRef {
    let value = operands.remove(1);
    let variable = expectVariable(operands.remove(0));
    return emitExpr(module, ResolvedExpr::Operator(Box::new(ResolvedOperator {
        operator: Operator::AssignEq,
        operands: Box::new([ResolvedExpr::Variable(variable.to_owned()), ResolvedExpr::Operator(Box::new(ResolvedOperator {
            operator,
            operands: Box::new([ResolvedExpr::Variable(variable), value]),
            expressionType: INTEGER_TYPE.to_owned(),
        }))]),
        expressionType: INTEGER_TYPE.to_owned(),
    })));
}

unsafe fn emitExpr(module: &mut CompiledModule, expr: ResolvedExpr) -> LLVMValueRef {
    return match expr {
        ResolvedExpr::Operator(expr) => {
            let mut operands = Vec::from(expr.operands);
            debug_assert_eq!(operands.len(), expr.operator.getOperands());
            let name = CString::new("operator").unwrap();
            let name = name.as_ptr();

            match expr.operator {
                Operator::Increment => {
                    if expr.expressionType == INTEGER_TYPE {
                        emitExpr(module, ResolvedExpr::Operator(Box::new(ResolvedOperator {
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
                        emitExpr(module, ResolvedExpr::Operator(Box::new(ResolvedOperator {
                            operator: Operator::DivAssign,
                            operands: Box::new([operands.remove(0), ResolvedExpr::LiteralInteger(1)]),
                            expressionType: INTEGER_TYPE.to_owned(),
                        })))
                    } else {
                        panic!("unexpected increment type {:?}", expr.expressionType)
                    }
                }
                Operator::Not => {
                    let operands = getOperands(module, operands);
                    LLVMBuildNot(module.builder, operands[0], name)
                }
                Operator::Plus => {
                    let operands = getOperands(module, operands);
                    if expr.expressionType == INTEGER_TYPE {
                        LLVMBuildAdd(module.builder, operands[0], operands[1], name)
                    } else if expr.expressionType == FLOAT_TYPE {
                        LLVMBuildFAdd(module.builder, operands[0], operands[1], name)
                    } else {
                        panic!("unknown arithmetic type {:?}", expr.expressionType);
                    }
                }
                Operator::Minus => {
                    let operands = getOperands(module, operands);
                    if expr.expressionType == INTEGER_TYPE {
                        LLVMBuildSub(module.builder, operands[0], operands[1], name)
                    } else if expr.expressionType == FLOAT_TYPE {
                        LLVMBuildFSub(module.builder, operands[0], operands[1], name)
                    } else {
                        panic!("unknown arithmetic type {:?}", expr.expressionType);
                    }
                }
                Operator::Mult => {
                    let operands = getOperands(module, operands);
                    if expr.expressionType == INTEGER_TYPE {
                        LLVMBuildMul(module.builder, operands[0], operands[1], name)
                    } else if expr.expressionType == FLOAT_TYPE {
                        LLVMBuildFMul(module.builder, operands[0], operands[1], name)
                    } else {
                        panic!("unknown arithmetic type {:?}", expr.expressionType);
                    }
                }
                Operator::Div => {
                    let operands = getOperands(module, operands);
                    if expr.expressionType == INTEGER_TYPE {
                        LLVMBuildExactSDiv(module.builder, operands[0], operands[1], name)
                    } else if expr.expressionType == FLOAT_TYPE {
                        LLVMBuildFDiv(module.builder, operands[0], operands[1], name)
                    } else {
                        panic!("unknown arithmetic type {:?}", expr.expressionType);
                    }
                }
                Operator::Mod => {
                    let operands = getOperands(module, operands);
                    if expr.expressionType == INTEGER_TYPE {
                        LLVMBuildSRem(module.builder, operands[0], operands[1], name)
                    } else {
                        panic!("unknown type for mod {:?}", expr.expressionType);
                    }
                }
                Operator::PlusAssign => {
                    emitOperatorAssign(module, operands, Operator::Plus)
                }
                Operator::MinusAssign => {
                    emitOperatorAssign(module, operands, Operator::Minus)
                }
                Operator::MultAssign => {
                    emitOperatorAssign(module, operands, Operator::Mult)
                }
                Operator::DivAssign => {
                    emitOperatorAssign(module, operands, Operator::Div)
                }
                Operator::ModAssign => {
                    emitOperatorAssign(module, operands, Operator::Mod)
                }
                Operator::And => {
                    let operands = getOperands(module, operands);
                    LLVMBuildAnd(module.builder, operands[0], operands[1], name)
                }
                Operator::Or => {
                    let operands = getOperands(module, operands);
                    LLVMBuildOr(module.builder, operands[0], operands[1], name)
                }
                Operator::Greater => {
                    let operands = getOperands(module, operands);
                    LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntSGT, operands[0], operands[1], name)
                }
                Operator::Less => {
                    let operands = getOperands(module, operands);
                    LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntSLT, operands[0], operands[1], name)
                }
                Operator::GreaterEq => {
                    let operands = getOperands(module, operands);
                    LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntSGE, operands[0], operands[1], name)
                }
                Operator::LessEq => {
                    let operands = getOperands(module, operands);
                    LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntSLE, operands[0], operands[1], name)
                }
                Operator::CompareEq => {
                    let operands = getOperands(module, operands);
                    LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntEQ, operands[0], operands[1], name)
                }
                Operator::CompareNotEq => {
                    let operands = getOperands(module, operands);
                    LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntNE, operands[0], operands[1], name)
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
            LLVMConstArray(LLVMInt8Type(), expr.chars().into_iter().map(|c| emitExpr(module, ResolvedExpr::LiteralChar(c as _))).collect::<Vec<_>>().as_mut_ptr(), expr.len() as _)
        }
    };
}

pub(in super) unsafe fn emit(module: &mut CompiledModule, statement: Statement) -> LLVMValueRef {
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
            emitExpr(module, expr)
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
