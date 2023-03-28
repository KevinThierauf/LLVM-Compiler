use std::ffi::CString;

use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction};
use llvm_sys::core::{LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMBasicBlockAsValue, LLVMBuildAdd, LLVMBuildAlloca, LLVMBuildAnd, LLVMBuildBr, LLVMBuildCall2, LLVMBuildCondBr, LLVMBuildExactSDiv, LLVMBuildFAdd, LLVMBuildFDiv, LLVMBuildFMul, LLVMBuildFSub, LLVMBuildICmp, LLVMBuildLoad2, LLVMBuildMul, LLVMBuildNot, LLVMBuildOr, LLVMBuildRet, LLVMBuildRetVoid, LLVMBuildSRem, LLVMBuildStore, LLVMBuildSub, LLVMConstArray, LLVMConstInt, LLVMConstNull, LLVMConstReal, LLVMCreateBasicBlockInContext, LLVMFloatType, LLVMFunctionType, LLVMInt1Type, LLVMInt32Type, LLVMInt8Type, LLVMPositionBuilderAtEnd};
use llvm_sys::LLVMIntPredicate;
use llvm_sys::prelude::{LLVMBasicBlockRef, LLVMBool, LLVMContextRef, LLVMTypeRef, LLVMValueRef};

use crate::backend::CompiledModule;
use crate::module::Operator;
use crate::resolver::function::Function;
use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::resolvedoperator::ResolvedOperator;
use crate::resolver::resolvedast::resolvedscope::ResolvedScope;
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
                    let operands = getOperands(module, operands);
                    LLVMBuildStore(module.builder, operands[1], operands[0])
                }
                Operator::Cast | Operator::Dot | Operator::Range | Operator::Ellipsis | Operator::Colon | Operator::ErrorPropagation => {
                    // should have been previously handled/removed
                    unreachable!()
                }
            }
        }
        ResolvedExpr::FunctionCall(expr) => {
            let functionName = expr.function.name.to_owned();
            let (function, functionType) = getFunctionValue(module, expr.function);
            let mut operands = getOperands(module, expr.argVec);
            let name = CString::new(format!("Call{}", functionName)).unwrap();
            LLVMBuildCall2(module.builder, functionType, function, operands.as_mut_ptr(), operands.len() as _, name.as_ptr())
        }
        ResolvedExpr::ConstructorCall(expr) => {
            todo!()
        }
        ResolvedExpr::VariableDeclaration(expr) => {
            let name = CString::new(format!("Allocate{}", expr.ty.getTypeName())).unwrap();
            let value = LLVMBuildAlloca(module.builder, expr.ty.getLLVMType(module.context.0.lock_arc().context), name.as_ptr());
            let _v = module.variableMap.insert(expr.id, value);
            debug_assert!(_v.is_none());
            value
        }
        ResolvedExpr::Variable(expr) => {
            let name = CString::new(format!("Load{}", expr.ty.getTypeName())).unwrap();
            LLVMBuildLoad2(module.builder, expr.ty.getLLVMType(module.context.0.lock_arc().context), *module.variableMap.get(&expr.id).unwrap(), name.as_ptr())
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

unsafe fn emitScope(module: &mut CompiledModule, scope: ResolvedScope, basicBlockCallback: impl FnOnce(LLVMContextRef, &CString) -> LLVMBasicBlockRef, endCallback: impl FnOnce(&mut CompiledModule, LLVMBasicBlockRef)) -> LLVMBasicBlockRef {
    let contextLock = module.context.0.lock_arc();
    let context = contextLock.context;
    let blockName = CString::new("block").unwrap();
    let basicBlock = basicBlockCallback(context, &blockName);
    drop(contextLock);

    module.blockStack.push(basicBlock);
    LLVMPositionBuilderAtEnd(module.builder, basicBlock);
    for statement in scope.statementVec {
        emit(module, statement);
    }
    let last = module.blockStack.pop().unwrap();
    endCallback(module, last);
    LLVMPositionBuilderAtEnd(module.builder, last);

    return basicBlock;
}

fn wrapInScope(statement: Statement) -> ResolvedScope {
    return match statement {
        Statement::Scope(resolved) => resolved,
        _ => ResolvedScope {
            statementVec: vec![statement],
        }
    };
}

unsafe fn getFunctionValue(module: &mut CompiledModule, function: Function) -> (LLVMValueRef, LLVMTypeRef) {
    let contextLock = module.context.0.lock_arc();
    let context = contextLock.context;
    let mut parameterTypes = function.parameters.iter().map(|v| v.ty.getLLVMType(context)).collect::<Vec<_>>();
    let functionType = LLVMFunctionType(
        function.returnType.getLLVMType(context),
        parameterTypes.as_mut_ptr(), parameterTypes.len() as _,
        0,
    );
    drop(contextLock);
    let functionName = CString::new(function.name.as_str()).unwrap();
    let function = LLVMAddFunction(module.module, functionName.as_ptr(), functionType);
    return (function, functionType);
}

pub(in super) unsafe fn emit(module: &mut CompiledModule, statement: Statement) -> LLVMValueRef {
    return match statement {
        Statement::If(statement) => {
            let contextLock = module.context.0.lock_arc();
            let context = contextLock.context;
            let name = CString::new("IfEnd").unwrap();
            let endBlock = LLVMCreateBasicBlockInContext(context, name.as_ptr());
            *module.blockStack.last_mut().unwrap() = endBlock;
            drop(contextLock);

            let ifBlock = emitScope(module, wrapInScope(statement.statement), |context, name| LLVMCreateBasicBlockInContext(context, name.as_ptr()), |module, _| {
                LLVMBuildBr(module.builder, endBlock);
            });
            let elseBlock = emitScope(module, wrapInScope(statement.elseStatement.unwrap_or(Statement::Scope(ResolvedScope {
                statementVec: Vec::new(),
            }))), |context, name| LLVMCreateBasicBlockInContext(context, name.as_ptr()), |module, _| {
                LLVMBuildBr(module.builder, endBlock);
            });

            let name = CString::new("if").unwrap();
            let condition = LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntNE, emitExpr(module, statement.condition), emitExpr(module, ResolvedExpr::LiteralInteger(0)), name.as_ptr());
            LLVMBuildCondBr(module.builder, condition, ifBlock, elseBlock)
        }
        Statement::While(statement) => {
            todo!()
        }
        Statement::Return(statement) => {
            if let Some(expr) = statement.expr {
                LLVMBuildRet(module.builder, emitExpr(module, expr))
            } else {
                LLVMBuildRetVoid(module.builder)
            }
        }
        Statement::Expr(expr) => {
            emitExpr(module, expr)
        }
        Statement::FunctionDefinition(statement) => {
            let function = getFunctionValue(module, statement.function).0;
            emitScope(module, statement.scope, |context, name| LLVMAppendBasicBlockInContext(context, function, name.as_ptr()), |_, _| {});
            LLVMVerifyFunction(function, LLVMVerifierFailureAction::LLVMAbortProcessAction);
            function
        }
        Statement::Scope(statement) => {
            return LLVMBasicBlockAsValue(emitScope(module, statement, |context, name| LLVMCreateBasicBlockInContext(context, name.as_ptr()), |module, next| {
                LLVMBuildBr(module.builder, next);
            }));
        }
        Statement::Multiple(statementVec) => {
            for statement in statementVec {
                emit(module, statement);
            }
            LLVMConstNull(LLVMInt8Type())
        }
    };
}
