use std::ffi::CString;

use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction};
use llvm_sys::core::{LLVMAddFunction, LLVMAddGlobal, LLVMAppendBasicBlockInContext, LLVMBasicBlockAsValue, LLVMBuildAdd, LLVMBuildAlloca, LLVMBuildAnd, LLVMBuildBr, LLVMBuildCall2, LLVMBuildCondBr, LLVMBuildExactSDiv, LLVMBuildFAdd, LLVMBuildFDiv, LLVMBuildFMul, LLVMBuildFSub, LLVMBuildICmp, LLVMBuildLoad2, LLVMBuildMul, LLVMBuildNot, LLVMBuildOr, LLVMBuildRet, LLVMBuildRetVoid, LLVMBuildSRem, LLVMBuildStore, LLVMBuildSub, LLVMConstArray, LLVMConstInt, LLVMConstNull, LLVMConstReal, LLVMFloatTypeInContext, LLVMFunctionType, LLVMGetInsertBlock, LLVMGetParam, LLVMInsertBasicBlockInContext, LLVMInt1TypeInContext, LLVMInt32TypeInContext, LLVMInt8TypeInContext, LLVMIsNull, LLVMPositionBuilderAtEnd};
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
use crate::resolver::resolvedast::variabledeclare::VariableDeclare;
use crate::resolver::typeinfo::primitive::float::FLOAT_TYPE;
use crate::resolver::typeinfo::primitive::integer::INTEGER_TYPE;
use crate::resolver::typeinfo::void::VOID_TYPE;

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
    let expressionType = value.getExpressionType();
    return emitExpr(module, ResolvedExpr::Operator(Box::new(ResolvedOperator {
        operator: Operator::AssignEq,
        operands: Box::new([ResolvedExpr::Variable(variable.to_owned()), ResolvedExpr::Operator(Box::new(ResolvedOperator {
            operator,
            operands: Box::new([ResolvedExpr::Variable(variable), value]),
            expressionType: expressionType.to_owned(),
        }))]),
        expressionType,
    })));
}

unsafe fn getAssignValue(module: &mut CompiledModule, expr: ResolvedExpr) -> LLVMValueRef {
    return match expr {
        ResolvedExpr::VariableDeclaration(v) => {
            emitExpr(module, ResolvedExpr::VariableDeclaration(v))
        }
        ResolvedExpr::Variable(v) => {
            *module.variableMap.get(&v.id).unwrap()
        }
        ResolvedExpr::Property(_) => {
            todo!()
        }
        expr if expr.getResolvedExprType().isAssignable() => {
            panic!("missing assignable branch");
        }
        _ => panic!("unexpected non-assignable value")
    };
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
                        debug_assert_eq!(operands.len(), 1);
                        debug_assert!(matches!(&operands[0], ResolvedExpr::Variable(_)));
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
                    let value = emitExpr(module, operands.remove(1));
                    let assignValue = getAssignValue(module, operands.remove(0));
                    LLVMBuildStore(module.builder, value, assignValue)
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
            let name = CString::new(format!("Call_{}", functionName)).unwrap();
            LLVMBuildCall2(module.builder, functionType, function, operands.as_mut_ptr(), operands.len() as _, name.as_ptr())
        }
        ResolvedExpr::ConstructorCall(expr) => {
            todo!()
        }
        ResolvedExpr::VariableDeclaration(expr) => {
            let value = if expr.global {
                let name = CString::new(format!("Global_{}", expr.ty.getTypeName())).unwrap();
                LLVMAddGlobal(module.module, expr.ty.getLLVMType(module.context.0.try_lock_arc().unwrap().context), name.as_ptr())
            } else {
                let name = CString::new(format!("Allocate_{}", expr.ty.getTypeName())).unwrap();
                LLVMBuildAlloca(module.builder, expr.ty.getLLVMType(module.context.0.try_lock_arc().unwrap().context), name.as_ptr())
            };
            let _v = module.variableMap.insert(expr.id, value);
            debug_assert!(_v.is_none());
            value
        }
        ResolvedExpr::Variable(expr) => {
            let name = CString::new(format!("Load_{}", expr.ty.getTypeName())).unwrap();
            LLVMBuildLoad2(module.builder, expr.ty.getLLVMType(module.context.0.try_lock_arc().unwrap().context), *module.variableMap.get(&expr.id).unwrap(), name.as_ptr())
        }
        ResolvedExpr::Property(expr) => {
            todo!()
        }
        ResolvedExpr::LiteralBool(expr) => {
            LLVMConstInt(LLVMInt1TypeInContext(module.context.0.try_lock_arc().unwrap().context), if expr { 1 } else { 0 }, LLVMBool::from(false))
        }
        ResolvedExpr::LiteralChar(expr) => {
            LLVMConstInt(LLVMInt8TypeInContext(module.context.0.try_lock_arc().unwrap().context), expr as _, LLVMBool::from(true))
        }
        ResolvedExpr::LiteralFloat(expr) => {
            LLVMConstReal(LLVMFloatTypeInContext(module.context.0.try_lock_arc().unwrap().context), expr)
        }
        ResolvedExpr::LiteralInteger(expr) => {
            LLVMConstInt(LLVMInt32TypeInContext(module.context.0.try_lock_arc().unwrap().context), expr as _, LLVMBool::from(true))
        }
        ResolvedExpr::LiteralString(expr) => {
            LLVMConstArray(LLVMInt8TypeInContext(module.context.0.try_lock_arc().unwrap().context), expr.chars().into_iter().map(|c| emitExpr(module, ResolvedExpr::LiteralChar(c as _))).collect::<Vec<_>>().as_mut_ptr(), expr.len() as _)
        }
    };
}

enum Next {
    // None,
    End,
    // Before(LLVMBasicBlockRef),
    Block(LLVMBasicBlockRef),
}

impl Next {
    unsafe fn setBlock(&self, module: &mut CompiledModule, function: LLVMValueRef, name: &str) {
        let name = CString::new(name).unwrap();
        match self {
            Next::End => {
                let block = LLVMAppendBasicBlockInContext(module.context.0.try_lock_arc().unwrap().context, function, name.as_ptr());
                LLVMPositionBuilderAtEnd(module.builder, block);
            }
            // Next::Before(block) => {
            //     let block = LLVMInsertBasicBlockInContext(module.context.0.try_lock_arc().unwrap().context, *block, name.as_ptr());
            //     LLVMPositionBuilderAtEnd(module.builder, block);
            // }
            Next::Block(block) => {
                let block = *block;
                LLVMPositionBuilderAtEnd(module.builder, block);
            }
            // Next::None => {
                // do nothing
            // }
        }
    }
}

unsafe fn emitScope(module: &mut CompiledModule, branch: bool, name: &str, function: LLVMValueRef, scope: ResolvedScope, basicBlockCallback: impl FnOnce(&mut CompiledModule, LLVMContextRef, &CString) -> LLVMBasicBlockRef, startCallback: impl FnOnce(&mut CompiledModule), endCallback: impl FnOnce(&mut CompiledModule) -> Next) -> LLVMBasicBlockRef {
    let contextLock = module.context.0.try_lock_arc().unwrap();
    let context = contextLock.context;
    let blockName = CString::new(name).unwrap();
    let basicBlock = basicBlockCallback(module, context, &blockName);
    drop(contextLock);
    if branch {
        LLVMBuildBr(module.builder, basicBlock);
    }

    LLVMPositionBuilderAtEnd(module.builder, basicBlock);
    startCallback(module);
    for statement in scope.statementVec {
        emit(module, function, statement);
    }
    endCallback(module).setBlock(module, function, &format!("after_{name}"));

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
    let contextLock = module.context.0.try_lock_arc().unwrap();
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

pub(in super) unsafe fn emit(module: &mut CompiledModule, function: LLVMValueRef, statement: Statement) -> LLVMValueRef {
    // println!("> {statement:?}");
    debug_assert!(LLVMIsNull(function) == 0 || matches!(statement, Statement::FunctionDefinition(_)));
    return match statement {
        Statement::If(statement) => {
            let condition = emitExpr(module, statement.condition);
            let falseValue = emitExpr(module, ResolvedExpr::LiteralBool(false));
            let contextLock = module.context.0.try_lock_arc().unwrap();
            let context = contextLock.context;
            let name = CString::new("IfEnd").unwrap();
            let endBlock = LLVMAppendBasicBlockInContext(context, function, name.as_ptr());

            let name = CString::new("ifcmp").unwrap();
            let condition = LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntNE, condition, falseValue, name.as_ptr());
            let name = CString::new("if_block").unwrap();
            let ifBlock = LLVMInsertBasicBlockInContext(context, endBlock, name.as_ptr());
            let name = CString::new("else_block").unwrap();
            let elseBlock = LLVMInsertBasicBlockInContext(context, endBlock, name.as_ptr());
            let branch = LLVMBuildCondBr(module.builder, condition, ifBlock, elseBlock);

            drop(contextLock);

            emitScope(module, false, "if", function, wrapInScope(statement.statement), |_, _, _| ifBlock, |_| {}, |module| {
                LLVMBuildBr(module.builder, endBlock);
                Next::Block(endBlock)
            });
            emitScope(module, false, "else", function, wrapInScope(statement.elseStatement.unwrap_or(Statement::Scope(ResolvedScope {
                statementVec: Vec::new(),
            }))), |_, _, _| elseBlock, |_| {}, |module| {
                LLVMBuildBr(module.builder, endBlock);
                Next::Block(endBlock)
            });
            branch
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
            let functionReturnType = statement.function.returnType.to_owned();
            let function = statement.function.to_owned();
            let functionName = &function.name;
            let parameters = &function.parameters;
            let function = getFunctionValue(module, statement.function).0;

            emitScope(module, false, &format!("start_{}", functionName), function, statement.scope, |module, context, name| {
                let block = LLVMAppendBasicBlockInContext(context, function, name.as_ptr());
                module.blockStack.push(LLVMGetInsertBlock(module.builder));
                block
            }, |module| {
                for index in 0..parameters.len() {
                    let parameterVariable = emitExpr(module, ResolvedExpr::VariableDeclaration(VariableDeclare {
                        ty: parameters[index].ty.to_owned(),
                        id: statement.parameterVecId[index],
                        global: false,
                    }));
                    let parameterValue = LLVMGetParam(function, index as _);
                    LLVMBuildStore(module.builder, parameterValue, parameterVariable);
                }
            }, |module| {
                if functionReturnType == VOID_TYPE {
                    LLVMBuildRetVoid(module.builder);
                }
                let prev = module.blockStack.pop().unwrap();
                Next::Block(prev)
            });
            // println!("{:#?}", CStr::from_ptr(LLVMPrintValueToString(function)).to_str().unwrap());
            LLVMVerifyFunction(function, LLVMVerifierFailureAction::LLVMAbortProcessAction);
            function
        }
        Statement::Scope(statement) => {
            return LLVMBasicBlockAsValue(emitScope(module, true, "Scope", function, statement, |_, context, name| LLVMAppendBasicBlockInContext(context, function, name.as_ptr()), |_| {}, |module| {
                Next::End
            }));
        }
        Statement::Multiple(statementVec) => {
            for statement in statementVec {
                emit(module, function, statement);
            }
            LLVMConstNull(LLVMInt8TypeInContext(module.context.0.try_lock_arc().unwrap().context))
        }
    };
}
