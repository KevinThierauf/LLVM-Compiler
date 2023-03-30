use std::ffi::CString;

use hashbrown::hash_map::Entry;
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction};
use llvm_sys::core::{LLVMAddFunction, LLVMAddGlobal, LLVMAppendBasicBlockInContext, LLVMBasicBlockAsValue, LLVMBuildAdd, LLVMBuildAlloca, LLVMBuildAnd, LLVMBuildBr, LLVMBuildCall2, LLVMBuildCondBr, LLVMBuildExactSDiv, LLVMBuildExtractValue, LLVMBuildFAdd, LLVMBuildFDiv, LLVMBuildFMul, LLVMBuildFSub, LLVMBuildGlobalString, LLVMBuildICmp, LLVMBuildLoad2, LLVMBuildMul, LLVMBuildNot, LLVMBuildOr, LLVMBuildRet, LLVMBuildRetVoid, LLVMBuildSRem, LLVMBuildStore, LLVMBuildStructGEP2, LLVMBuildSub, LLVMConstInt, LLVMConstNull, LLVMConstPointerNull, LLVMConstReal, LLVMConstStructInContext, LLVMFloatTypeInContext, LLVMFunctionType, LLVMGetInsertBlock, LLVMGetParam, LLVMInsertBasicBlockInContext, LLVMInt1TypeInContext, LLVMInt32TypeInContext, LLVMInt8TypeInContext, LLVMIsNull, LLVMPositionBuilderAtEnd, LLVMSetInitializer};
use llvm_sys::LLVMIntPredicate;
use llvm_sys::prelude::{LLVMBasicBlockRef, LLVMBool, LLVMContextRef, LLVMTypeRef, LLVMValueRef};
use once_cell::sync::Lazy;

use crate::ast::visibility::Visibility;
use crate::backend::CompiledModule;
use crate::module::Operator;
use crate::resolver::function::{Function, Parameter};
use crate::resolver::resolvedast::defaultvalue::DefaultValue;
use crate::resolver::resolvedast::resolvedexpr::ResolvedExpr;
use crate::resolver::resolvedast::resolvedoperator::ResolvedOperator;
use crate::resolver::resolvedast::resolvedscope::ResolvedScope;
use crate::resolver::resolvedast::resolvedvariable::ResolvedVariable;
use crate::resolver::resolvedast::statement::Statement;
use crate::resolver::resolvedast::variabledeclare::VariableDeclare;
use crate::resolver::typeinfo::pointer::PointerType;
use crate::resolver::typeinfo::primitive::character::CHARACTER_TYPE;
use crate::resolver::typeinfo::primitive::float::FLOAT_TYPE;
use crate::resolver::typeinfo::primitive::integer::INTEGER_TYPE;
use crate::resolver::typeinfo::string::STRING_TYPE;
use crate::resolver::typeinfo::Type;
use crate::resolver::typeinfo::void::VOID_TYPE;

unsafe fn getOperands(module: &mut CompiledModule, operands: Vec<ResolvedExpr>) -> Vec<LLVMValueRef> {
    return operands.into_iter().map(|expr| emitExpr(module, expr)).collect::<Vec<_>>();
}

unsafe fn emitOperatorAssign(module: &mut CompiledModule, mut operands: Vec<ResolvedExpr>, operator: Operator) -> LLVMValueRef {
    let variable = operands.remove(0);
    let expressionType = variable.getExpressionType();
    let llvmExpressionType = expressionType.getLLVMType(module.context.0.lock_arc().context);

    let name = CString::new("value").unwrap();

    let variable = getAssignValue(module, variable);
    let loadedVariable = LLVMBuildLoad2(module.builder, llvmExpressionType, variable, name.as_ptr());
    let value = emitExpr(module, operands.remove(0));

    let modifiedValue = basicOperator(module, operator, expressionType.to_owned(), vec![loadedVariable, value]);
    return LLVMBuildStore(module.builder, modifiedValue, variable);
}

unsafe fn basicOperator(module: &mut CompiledModule, operator: Operator, exprType: Type, operands: Vec<LLVMValueRef>) -> LLVMValueRef {
    let name = CString::new(format!("operator_{:?}", operator)).unwrap();
    let name = name.as_ptr();

    match operator {
        Operator::Not => {
            LLVMBuildNot(module.builder, operands[0], name)
        }
        Operator::Plus => {
            if exprType == INTEGER_TYPE {
                LLVMBuildAdd(module.builder, operands[0], operands[1], name)
            } else if exprType == FLOAT_TYPE {
                LLVMBuildFAdd(module.builder, operands[0], operands[1], name)
            } else {
                panic!("unknown arithmetic type {:?}", exprType);
            }
        }
        Operator::Minus => {
            if exprType == INTEGER_TYPE {
                LLVMBuildSub(module.builder, operands[0], operands[1], name)
            } else if exprType == FLOAT_TYPE {
                LLVMBuildFSub(module.builder, operands[0], operands[1], name)
            } else {
                panic!("unknown arithmetic type {:?}", exprType);
            }
        }
        Operator::Mult => {
            if exprType == INTEGER_TYPE {
                LLVMBuildMul(module.builder, operands[0], operands[1], name)
            } else if exprType == FLOAT_TYPE {
                LLVMBuildFMul(module.builder, operands[0], operands[1], name)
            } else {
                panic!("unknown arithmetic type {:?}", exprType);
            }
        }
        Operator::Div => {
            if exprType == INTEGER_TYPE {
                LLVMBuildExactSDiv(module.builder, operands[0], operands[1], name)
            } else if exprType == FLOAT_TYPE {
                LLVMBuildFDiv(module.builder, operands[0], operands[1], name)
            } else {
                panic!("unknown arithmetic type {:?}", exprType);
            }
        }
        Operator::Mod => {
            if exprType == INTEGER_TYPE {
                LLVMBuildSRem(module.builder, operands[0], operands[1], name)
            } else {
                panic!("unknown type for mod {:?}", exprType);
            }
        }
        Operator::And => {
            LLVMBuildAnd(module.builder, operands[0], operands[1], name)
        }
        Operator::Or => {
            LLVMBuildOr(module.builder, operands[0], operands[1], name)
        }
        Operator::Greater => {
            LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntSGT, operands[0], operands[1], name)
        }
        Operator::Less => {
            LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntSLT, operands[0], operands[1], name)
        }
        Operator::GreaterEq => {
            LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntSGE, operands[0], operands[1], name)
        }
        Operator::LessEq => {
            LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntSLE, operands[0], operands[1], name)
        }
        Operator::CompareEq => {
            LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntEQ, operands[0], operands[1], name)
        }
        Operator::CompareNotEq => {
            LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntNE, operands[0], operands[1], name)
        }
        _ => unreachable!()
    }
}

unsafe fn basicOperatorResolved(module: &mut CompiledModule, operator: Operator, exprType: Type, operands: Vec<ResolvedExpr>) -> LLVMValueRef {
    let operands = operands.into_iter().map(|v| emitExpr(module, v)).collect();
    return basicOperator(module, operator, exprType, operands);
}

unsafe fn getAssignValue(module: &mut CompiledModule, expr: ResolvedExpr) -> LLVMValueRef {
    return match expr {
        ResolvedExpr::VariableDeclaration(v) => {
            emitExpr(module, ResolvedExpr::VariableDeclaration(v))
        }
        ResolvedExpr::Variable(v) => {
            *module.variableMap.get(&v.id).unwrap()
        }
        ResolvedExpr::Property(expr) => {
            let name = CString::new(format!("property_{}", expr.property.name)).unwrap();
            let exprType = expr.value.getExpressionType().getLLVMType(module.context.0.lock_arc().context);
            let exprValue = getAssignValue(module, expr.value);
            LLVMBuildStructGEP2(module.builder, exprType, exprValue, expr.property.index as _, name.as_ptr())
        }
        expr if expr.getResolvedExprType().isAssignable() => {
            panic!("missing assignable branch");
        }
        _ => panic!("unexpected non-assignable value")
    };
}

pub unsafe fn emitExpr(module: &mut CompiledModule, expr: ResolvedExpr) -> LLVMValueRef {
    return match expr {
        ResolvedExpr::Operator(expr) => {
            let mut operands = Vec::from(expr.operands);
            debug_assert_eq!(operands.len(), expr.operator.getOperands());

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
                Operator::AssignEq => {
                    let value = emitExpr(module, operands.remove(1));
                    let assignValue = getAssignValue(module, operands.remove(0));
                    LLVMBuildStore(module.builder, value, assignValue)
                }
                Operator::Cast | Operator::Dot | Operator::Range | Operator::Ellipsis | Operator::Colon | Operator::ErrorPropagation => {
                    // should have been previously handled/removed
                    unreachable!()
                }
                _ => basicOperatorResolved(module, expr.operator, expr.expressionType, operands),
            }
        }
        ResolvedExpr::FunctionCall(expr) => {
            let returnType = expr.function.returnType.to_owned();
            let functionName = expr.function.name.to_owned();
            let (function, functionType) = getFunctionValue(module, expr.function);
            let mut operands = getOperands(module, expr.argVec);
            let name = if returnType == VOID_TYPE {
                "".to_owned()
            } else {
                format!("call_{}", functionName)
            };
            let name = CString::new(name).unwrap();
            LLVMBuildCall2(module.builder, functionType, function, operands.as_mut_ptr(), operands.len() as _, name.as_ptr())
        }
        ResolvedExpr::VariableDeclaration(expr) => {
            let value = if expr.global {
                let name = CString::new(format!("Global_{}", expr.ty.getTypeName())).unwrap();
                let value = LLVMAddGlobal(module.module, expr.ty.getLLVMType(module.context.0.lock_arc().context), name.as_ptr());
                value
            } else {
                let name = CString::new(format!("Allocate_{}", expr.ty.getTypeName())).unwrap();
                let alloc = LLVMBuildAlloca(module.builder, expr.ty.getLLVMType(module.context.0.lock_arc().context), name.as_ptr());
                alloc
            };
            let _v = module.variableMap.insert(expr.id, value);
            debug_assert!(_v.is_none());

            let defaultValue = ResolvedExpr::DefaultValue(DefaultValue {
                ty: expr.ty.to_owned(),
            });
            if expr.global {
                LLVMSetInitializer(value, emitExpr(module, defaultValue));
            } else {
                emitExpr(module, ResolvedExpr::Operator(Box::new(ResolvedOperator {
                    operator: Operator::AssignEq,
                    operands: Box::new([ResolvedExpr::Variable(ResolvedVariable {
                        ty: expr.ty.to_owned(),
                        id: expr.id,
                    }), defaultValue]),
                    expressionType: expr.ty.to_owned(),
                })));
            }

            value
        }
        ResolvedExpr::Variable(expr) => {
            let name = CString::new(format!("Load_{}", expr.ty.getTypeName())).unwrap();
            LLVMBuildLoad2(module.builder, expr.ty.getLLVMType(module.context.0.lock_arc().context), *module.variableMap.get(&expr.id).unwrap(), name.as_ptr())
        }
        ResolvedExpr::DefaultValue(expr) => {
            emitExpr(module, expr.ty.getDefaultValue())
        }
        ResolvedExpr::DefaultClass(expr) => {
            let mut properties = expr.ty.getPropertyMap().values().collect::<Vec<_>>();
            properties.sort_by_key(|property| property.index);
            let mut properties = properties.iter().map(|property| emitExpr(module, property.ty.getDefaultValue())).collect::<Vec<_>>();
            LLVMConstStructInContext(module.context.0.lock_arc().context, properties.as_mut_ptr(), properties.len() as _, 0)
        }
        ResolvedExpr::DefaultPointer(expr) => {
            LLVMConstPointerNull(expr.ty.getLLVMType(module.context.0.lock_arc().context))
        }
        ResolvedExpr::Property(expr) => {
            let name = CString::new(format!("property_{}", expr.property.name)).unwrap();
            LLVMBuildExtractValue(module.builder, emitExpr(module, expr.value), expr.property.index as _, name.as_ptr())
        }
        ResolvedExpr::LiteralBool(expr) => {
            LLVMConstInt(LLVMInt1TypeInContext(module.context.0.lock_arc().context), if expr { 1 } else { 0 }, LLVMBool::from(false))
        }
        ResolvedExpr::LiteralChar(expr) => {
            LLVMConstInt(LLVMInt8TypeInContext(module.context.0.lock_arc().context), expr as _, LLVMBool::from(true))
        }
        ResolvedExpr::LiteralFloat(expr) => {
            LLVMConstReal(LLVMFloatTypeInContext(module.context.0.lock_arc().context), expr)
        }
        ResolvedExpr::LiteralInteger(expr) => {
            LLVMConstInt(LLVMInt32TypeInContext(module.context.0.lock_arc().context), expr as _, LLVMBool::from(true))
        }
        ResolvedExpr::LiteralString(expr) => {
            let stringLength = expr.len();
            let string = CString::new(expr).unwrap();
            let stringName = CString::new(format!("string_literal")).unwrap();

            let mut properties = vec![
                emitExpr(module, ResolvedExpr::LiteralInteger(stringLength as _)),
                LLVMBuildGlobalString(module.builder, string.as_ptr(), stringName.as_ptr())
            ];
            LLVMConstStructInContext(module.context.0.lock_arc().context, properties.as_mut_ptr(), properties.len() as _, 0)
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
                let block = LLVMAppendBasicBlockInContext(module.context.0.lock_arc().context, function, name.as_ptr());
                LLVMPositionBuilderAtEnd(module.builder, block);
            }
            // Next::Before(block) => {
            //     let block = LLVMInsertBasicBlockInContext(module.context.0.lock_arc().context, *block, name.as_ptr());
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
    let contextLock = module.context.0.lock_arc();
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
    match module.functionMap.entry(function.id) {
        Entry::Occupied(v) => {
            *v.get()
        }
        Entry::Vacant(v) => {
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
            *v.insert((function, functionType))
        }
    }
}

pub unsafe fn emit(module: &mut CompiledModule, function: LLVMValueRef, statement: Statement) -> LLVMValueRef {
    // println!("> {statement:?}");
    debug_assert!(LLVMIsNull(function) == 0 || matches!(statement, Statement::FunctionDefinition(_)));
    return match statement {
        Statement::If(statement) => {
            let condition = emitExpr(module, statement.condition);
            let falseValue = emitExpr(module, ResolvedExpr::LiteralBool(false));
            let contextLock = module.context.0.lock_arc();
            let context = contextLock.context;
            let name = CString::new("if_end").unwrap();
            let endBlock = LLVMAppendBasicBlockInContext(context, function, name.as_ptr());

            let name = CString::new("if_cmp").unwrap();
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
            let contextLock = module.context.0.lock_arc();
            let context = contextLock.context;
            let name = CString::new("while_cmp").unwrap();
            let cmpBlock = LLVMAppendBasicBlockInContext(context, function, name.as_ptr());
            let name = CString::new("while_end").unwrap();
            let endBlock = LLVMAppendBasicBlockInContext(context, function, name.as_ptr());

            let name = CString::new("while_block").unwrap();
            let whileBlock = LLVMInsertBasicBlockInContext(context, endBlock, name.as_ptr());

            drop(contextLock);

            LLVMBuildBr(module.builder, cmpBlock);
            LLVMPositionBuilderAtEnd(module.builder, cmpBlock);
            let condition = emitExpr(module, statement.condition);
            let falseValue = emitExpr(module, ResolvedExpr::LiteralBool(false));
            let name = CString::new("while_condition").unwrap();
            let condition = LLVMBuildICmp(module.builder, LLVMIntPredicate::LLVMIntNE, condition, falseValue, name.as_ptr());
            LLVMBuildCondBr(module.builder, condition, whileBlock, endBlock);

            emitScope(module, false, "", function, wrapInScope(statement.statement), |_, _, _| whileBlock, |_| {}, |module| {
                LLVMBuildBr(module.builder, cmpBlock);
                Next::Block(endBlock)
            });
            LLVMConstNull(LLVMInt8TypeInContext(module.context.0.lock_arc().context))
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
        Statement::Print(statement) => {
            let ty = statement.value.getExpressionType();
            let value = emitExpr(module, statement.value);
            let valueName = CString::new("").unwrap();

            if ty == INTEGER_TYPE {
                static FUNCTION: Lazy<Function> = Lazy::new(|| Function::new("sdk_print_int".to_string(), Visibility::Public, VOID_TYPE.to_owned(), vec![
                    Parameter {
                        ty: INTEGER_TYPE.to_owned(),
                        name: "value".to_string(),
                    },
                ]));

                let mut operands = vec![value];
                let (function, functionType) = getFunctionValue(module, FUNCTION.to_owned());
                LLVMBuildCall2(module.builder, functionType, function, operands.as_mut_ptr(), operands.len() as _, valueName.as_ptr())
            } else if ty == FLOAT_TYPE {
                static FUNCTION: Lazy<Function> = Lazy::new(|| Function::new("sdk_print_float".to_string(), Visibility::Public, VOID_TYPE.to_owned(), vec![
                    Parameter {
                        ty: FLOAT_TYPE.to_owned(),
                        name: "value".to_string(),
                    },
                ]));

                let mut operands = vec![value];
                let (function, functionType) = getFunctionValue(module, FUNCTION.to_owned());
                LLVMBuildCall2(module.builder, functionType, function, operands.as_mut_ptr(), operands.len() as _, valueName.as_ptr())
            } else if ty == STRING_TYPE {
                static FUNCTION: Lazy<Function> = Lazy::new(|| Function::new("sdk_print_string".to_string(), Visibility::Public, VOID_TYPE.to_owned(), vec![
                    Parameter {
                        ty: PointerType::new(CHARACTER_TYPE.to_owned()),
                        name: "pointer".to_string(),
                    },
                    Parameter {
                        ty: INTEGER_TYPE.to_owned(),
                        name: "length".to_string(),
                    },
                ]));

                let pointerName = CString::new("pointer_value").unwrap();
                let lengthName = CString::new("pointer_value").unwrap();
                let mut operands = vec![
                    LLVMBuildExtractValue(module.builder, value, STRING_TYPE.getPropertyMap().get("pointer").unwrap().to_owned().index as _, pointerName.as_ptr()),
                    LLVMBuildExtractValue(module.builder, value, STRING_TYPE.getPropertyMap().get("length").unwrap().to_owned().index as _, lengthName.as_ptr())
                ];

                let (function, functionType) = getFunctionValue(module, FUNCTION.to_owned());
                LLVMBuildCall2(module.builder, functionType, function, operands.as_mut_ptr(), operands.len() as _, valueName.as_ptr())
            } else {
                panic!("unsupported print type");
            }
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
            return LLVMBasicBlockAsValue(emitScope(module, true, "Scope", function, statement, |_, context, name| LLVMAppendBasicBlockInContext(context, function, name.as_ptr()), |_| {}, |_| {
                Next::End
            }));
        }
        Statement::Multiple(statementVec) => {
            for statement in statementVec {
                emit(module, function, statement);
            }
            LLVMConstNull(LLVMInt8TypeInContext(module.context.0.lock_arc().context))
        }
    };
}
