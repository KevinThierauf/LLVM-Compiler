use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

use crate::ast::ASTError;
use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::symbol::block::BlockSym;
use crate::ast::symbol::breaksym::BreakSym;
use crate::ast::symbol::classdefinition::{ClassDefinitionSym, ClassFieldDefinition, ClassMember, ClassStaticFieldDefinition};
use crate::ast::symbol::continuesym::ContinueSym;
use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::expr::functioncall::FunctionCallExpr;
use crate::ast::symbol::expr::literal::literalarray::LiteralArray;
use crate::ast::symbol::expr::literal::literalbool::LiteralBool;
use crate::ast::symbol::expr::literal::literalchar::LiteralChar;
use crate::ast::symbol::expr::literal::literalfloat::LiteralFloat;
use crate::ast::symbol::expr::literal::literalinteger::LiteralInteger;
use crate::ast::symbol::expr::literal::literalstring::LiteralString;
use crate::ast::symbol::expr::literal::literaltuple::LiteralTuple;
use crate::ast::symbol::expr::literal::literalvoid::LiteralVoid;
use crate::ast::symbol::expr::operatorexpr::{OperationComponent, OperatorExpr};
use crate::ast::symbol::expr::variabledeclaration::VariableDeclarationExpr;
use crate::ast::symbol::expr::variableexpr::VariableExpr;
use crate::ast::symbol::function::{FunctionAttribute, FunctionDefinitionSym, FunctionParameter};
use crate::ast::symbol::ifstatement::{ElseSym, IfSym};
use crate::ast::symbol::import::ImportSym;
use crate::ast::symbol::looptype::label::Label;
use crate::ast::symbol::looptype::whileloop::WhileLoop;
use crate::ast::symbol::returnsym::ReturnSym;
use crate::ast::tokensource::conflictresolution::{resolveClassDefinitionConflict, resolveSymbolConflict};
use crate::ast::tokensource::matchtype::{getLazyMatch, getMappedMatch, getMatchAnyOf, getMatchFrom, getMatchOneOf, getRepeatingMatch, Match, MatchOption, MatchType, OptionalMatch};
use crate::ast::visibility::Visibility;
use crate::module::{FileRange, Keyword, Module, Operator, ParenthesisType, QuoteType, TokenType, TokenTypeDiscriminants};
use crate::module::modulepos::{ModulePos, ModuleRange};
use crate::ast::symbol::expr::constructorcallexpr::ConstructorCallExpr;

pub fn getMatchKeyword(keyword: Keyword) -> impl MatchType<Value = ()> {
    return getMatchFrom(format!("{keyword:?}"), move |pos| {
        if let TokenType::Keyword(value) = pos.getToken().getTokenType() {
            if &keyword == value {
                return Ok(Match::new(pos.getRangeWithLength(1), ()));
            }
        }

        return Err(ASTError::ExpectedToken(pos, TokenType::Keyword(keyword)));
    });
}

pub fn getMatchOperator(operator: Operator) -> impl MatchType<Value = ()> {
    return getMatchFrom(format!("{operator:?}"), move |pos| {
        if let TokenType::Operator(value) = pos.getToken().getTokenType() {
            if &operator == value {
                return Ok(Match::new(pos.getRangeWithLength(1), ()));
            }
        }

        return Err(ASTError::ExpectedToken(pos, TokenType::Operator(operator)));
    });
}

pub fn getMatchFunctionAttribute() -> impl MatchType<Value = FunctionAttribute> {
    return getMatchFrom(format!("FunctionAttribute"), |pos| {
        if let TokenType::Keyword(keyword) = pos.getToken().getTokenType() {
            return Ok(Match::new(pos.getRangeWithLength(1), FunctionAttribute::fromKeyword(*keyword).ok_or(ASTError::ExpectedTokenDiscriminant(pos, TokenTypeDiscriminants::Keyword))?));
        }

        return Err(ASTError::ExpectedTokenDiscriminant(pos, TokenTypeDiscriminants::Keyword));
    });
}

pub fn getMatchVisibility() -> impl MatchType<Value = Visibility> {
    return getMatchFrom(format!("Visibility"), |pos| {
        if let TokenType::Keyword(keyword) = pos.getToken().getTokenType() {
            return Ok(Match::new(pos.getRangeWithLength(1), Visibility::fromKeyword(*keyword).ok_or(ASTError::MatchFailed(pos))?));
        }

        return Err(ASTError::MatchFailed(pos));
    });
}

pub fn getMatchType() -> impl MatchType<Value = ModulePos> {
    return getMatchFrom(format!("type"), |pos| {
        return match pos.getToken().getTokenType() {
            TokenType::Keyword(Keyword::Void) | TokenType::Identifier => Ok(Match::new(pos.getRangeWithLength(1), pos)),
            _ => Err(ASTError::ExpectedType(pos)),
        };
    });
}

pub fn getMatchIdentifier() -> impl MatchType<Value = ModulePos> {
    return getMatchFrom(format!("Identifier"), |pos| {
        if let TokenType::Identifier = pos.getToken().getTokenType() {
            return Ok(Match::new(pos.getRangeWithLength(1), pos));
        }

        return Err(ASTError::ExpectedToken(pos, TokenType::Identifier));
    });
}

pub fn getMatchSemicolan() -> impl MatchType<Value = ModulePos> {
    return getMatchFrom(format!("Semicolan"), |pos| {
        if let TokenType::SemiColan = pos.getToken().getTokenType() {
            return Ok(Match::new(pos.getRangeWithLength(1), pos));
        }

        return Err(ASTError::ExpectedToken(pos, TokenType::SemiColan));
    });
}

pub fn getMatchParenthesis<T: Debug>(parenthesis: ParenthesisType, function: impl 'static + Clone + Fn(&Rc<Module>) -> Result<T, ASTError>) -> impl MatchType<Value = T> {
    return getMatchFrom(format!("Parenthesis"), move |pos| {
        if let TokenType::Parenthesis(parenthesisType, module) = pos.getToken().getTokenType() {
            if &parenthesis == parenthesisType {
                return Ok(Match::new(pos.getRangeWithLength(1), function(module)?));
            }
        }

        return Err(ASTError::ExpectedTokenDiscriminant(pos, TokenTypeDiscriminants::Parenthesis));
    });
}

pub fn getMatchQuote<T: Debug>(quote: QuoteType, function: impl 'static + Clone + Fn(ModuleRange, &FileRange) -> Result<T, ASTError>) -> impl MatchType<Value = T> {
    return getMatchFrom(format!("{quote:?}Quote"), move |pos| {
        if let TokenType::String(quoteType, fileRange) = pos.getToken().getTokenType() {
            if &quote == quoteType {
                let range = pos.getRangeWithLength(1);
                return Ok(Match::new(range.to_owned(), function(range, fileRange)?));
            }
        }

        return Err(ASTError::ExpectedTokenDiscriminant(pos, TokenTypeDiscriminants::String));
    });
}

pub fn getMatchAll<S: Debug>(matchType: impl 'static + Clone + MatchType<Value = S>) -> impl MatchType<Value = Vec<S>> {
    return getMatchFrom(format!("MatchAnyFullRange({matchType:?})"), move |mut pos| {
        let startIndex = pos.getTokenIndex();
        let endPos = pos.getModule().getModulePos(pos.getModule().getTokenVector().len());
        let mut matchVec = Vec::new();
        while pos != endPos {
            let _debugPos = pos.to_owned();
            // let matchValue = match matchType.getMatch(pos) {
            //     Ok(matchValue) => matchValue,
            //     Err(err) => {
            //         println!("failed to match {:?}", matchType.getDescription());
            //         return Err(err);
            //     }
            // };
            let matchValue = matchType.getMatch(pos)?;
            debug_assert_ne!(_debugPos, matchValue.getRange().getEndPos(), "zero length symbol matched");
            pos = matchValue.getRange().getEndPos().to_owned();
            // println!("matched {:?}", matchValue.getValue());
            matchVec.push(matchValue.take().1);
        }
        return Ok(Match::new(pos.getModule().getModuleRange(startIndex..endPos.getTokenIndex()), matchVec));
    });
}

pub fn getMatchSymbolsAll() -> impl MatchType<Value = Vec<Symbol>> {
    return getMatchAll(getMatchSymbol());
}

pub fn getMatchSymbol() -> impl MatchType<Value = Symbol> {
    return getMappedMatch(getLazyMatch(|| (getRepeatingMatch(0, getMatchSemicolan()), getMatchAnyOf(&[
        MatchOption::new(getMatchBlockSym(), |_, v| Ok(Symbol::Block(v))),
        MatchOption::new(getMatchBreakSym(), |_, v| Ok(Symbol::Break(v))),
        MatchOption::new(getMatchContinueSym(), |_, v| Ok(Symbol::Continue(v))),
        MatchOption::new(getMatchClassDefinitionSym(), |_, v| Ok(Symbol::ClassDefinition(v))),
        MatchOption::new(getMatchFunctionDefinitionSym(), |_, v| Ok(Symbol::FunctionDefinition(v))),
        MatchOption::new(getMatchIfSym(), |_, v| Ok(Symbol::IfSym(v))),
        MatchOption::new(getMatchWhileSym(), |_, v| Ok(Symbol::While(v))),
        MatchOption::new(getMatchReturnSym(), |_, v| Ok(Symbol::Return(v))),
        MatchOption::new(getMatchImportSym(), |_, v| Ok(Symbol::ImportSym(v))),
        MatchOption::new(getMatchConstructorCallExpr(), |_, v| Ok(Symbol::Expr(Expr::ConstructorCall(v)))),
        MatchOption::new(getMatchFunctionCallExpr(), |_, v| Ok(Symbol::Expr(Expr::FunctionCall(v)))),
        MatchOption::new(getMatchOperatorExpr(), |_, v| Ok(Symbol::Expr(Expr::Operator(v)))),
        MatchOption::new(getMatchVariableDeclarationExpr(), |_, v| Ok(Symbol::Expr(Expr::VariableDeclaration(v)))),
        MatchOption::new(getMatchVariableExpr(), |_, v| Ok(Symbol::Expr(Expr::Variable(v)))),
        MatchOption::new(getMatchLiteralArray(), |_, v| Ok(Symbol::Expr(Expr::LiteralArray(v)))),
        MatchOption::new(getMatchLiteralBool(), |_, v| Ok(Symbol::Expr(Expr::LiteralBool(v)))),
        MatchOption::new(getMatchLiteralChar(), |_, v| Ok(Symbol::Expr(Expr::LiteralChar(v)))),
        MatchOption::new(getMatchLiteralFloat(), |_, v| Ok(Symbol::Expr(Expr::LiteralFloat(v)))),
        MatchOption::new(getMatchLiteralInteger(), |_, v| Ok(Symbol::Expr(Expr::LiteralInteger(v)))),
        MatchOption::new(getMatchLiteralString(), |_, v| Ok(Symbol::Expr(Expr::LiteralString(v)))),
        MatchOption::new(getMatchLiteralVoid(), |_, v| Ok(Symbol::Expr(Expr::LiteralVoid(v)))),
        MatchOption::new(getMatchLiteralTuple(), |_, v| Ok(Symbol::Expr(Expr::LiteralTuple(v)))),
    ], |pos, mut matchVec, errVec| {
        return if matchVec.is_empty() {
            Err(ASTError::MatchOptionsFailed(pos, errVec))
        } else {
            let index = resolveSymbolConflict(pos, matchVec.iter_mut().map(|symbolMatch| {
                if let Symbol::Expr(Expr::Operator(expr)) = symbolMatch.getValue() {
                    symbolMatch.range = expr.getRange().to_owned();
                }
                &*symbolMatch
            }))?;
            Ok(matchVec.swap_remove(index))
        };
    }))), |_, (_, symbol)| Ok(symbol));
}

pub fn getMatchExcludingExpr(excludeOperator: bool, excludeDeclaration: bool) -> impl MatchType<Value = Expr> {
    return getMatchAnyOf(&[
        MatchOption::new(getMatchConstructorCallExpr(), |_, v| Ok(Expr::ConstructorCall(v))),
        MatchOption::new(getMatchFunctionCallExpr(), |_, v| Ok(Expr::FunctionCall(v))),
        if !excludeOperator {
            MatchOption::new(getMatchOperatorExpr(), |_, v| Ok(Expr::Operator(v)))
        } else {
            MatchOption::new(getMatchFrom(format!("NOP"), |pos| Err(ASTError::MatchFailed(pos))), |pos, _: u8| Err(ASTError::MatchFailed(pos.getStartPos())))
        },
        if !excludeDeclaration {
            MatchOption::new(getMatchVariableDeclarationExpr(), |_, v| Ok(Expr::VariableDeclaration(v)))
        } else {
            MatchOption::new(getMatchFrom(format!("NOP"), |pos| Err(ASTError::MatchFailed(pos))), |pos, _: u8| Err(ASTError::MatchFailed(pos.getStartPos())))
        },
        MatchOption::new(getMatchVariableExpr(), |_, v| Ok(Expr::Variable(v))),
        MatchOption::new(getMatchLiteralArray(), |_, v| Ok(Expr::LiteralArray(v))),
        MatchOption::new(getMatchLiteralBool(), |_, v| Ok(Expr::LiteralBool(v))),
        MatchOption::new(getMatchLiteralChar(), |_, v| Ok(Expr::LiteralChar(v))),
        MatchOption::new(getMatchLiteralFloat(), |_, v| Ok(Expr::LiteralFloat(v))),
        MatchOption::new(getMatchLiteralInteger(), |_, v| Ok(Expr::LiteralInteger(v))),
        MatchOption::new(getMatchLiteralString(), |_, v| Ok(Expr::LiteralString(v))),
        MatchOption::new(getMatchLiteralVoid(), |_, v| Ok(Expr::LiteralVoid(v))),
        MatchOption::new(getMatchLiteralTuple(), |_, v| Ok(Expr::LiteralTuple(v))),
    ], |pos, options, err| {
        return if options.is_empty() {
            Err(ASTError::MatchOptionsFailed(pos, err))
        } else {
            let mut symbolVec = options.into_iter().map(|symbolMatch| {
                let (range, symbol) = symbolMatch.take();
                Match::new(range, symbol.toSymbol())
            }).collect::<Vec<_>>();
            let index = resolveSymbolConflict(pos, symbolVec.iter())?;
            let (range, symbol) = symbolVec.swap_remove(index).take();
            Ok(Match::new(range, symbol.toExpression().unwrap()))
        };
    });
}

pub fn getMatchExpr() -> impl MatchType<Value = Expr> {
    return getMatchExcludingExpr(false, false);
}

pub fn getMatchExprCommaList() -> impl MatchType<Value = Vec<Expr>> {
    return getMatchFrom(format!("CommaList(Expr)"), move |pos| {
        let token = pos.getToken();
        let matchExpr = getMatchExpr();
        return if let TokenType::CommaList(moduleVec) = token.getTokenType() {
            let mut exprVec = Vec::new();
            for module in moduleVec {
                let matchValue = matchExpr.getMatch(module.getModulePos(0))?;
                if matchValue.getRange().getEndIndex() != module.getTokenVector().len() {
                    return Err(ASTError::ExpectedExclusive(matchValue.getRange().getEndPos(), None));
                }
                exprVec.push(matchValue.take().1);
            }
            Ok(Match::new(pos.getRangeWithLength(1), exprVec))
        } else {
            if pos.getModule().getTokenVector().is_empty() {
                Ok(Match::new(pos.getRangeWithLength(pos.getModule().getTokenVector().len() - pos.getTokenIndex()), Vec::new()))
            } else {
                matchExpr.getMatch(pos.getModule().getModulePos(0)).map(|expr| {
                    let (range, value) = expr.take();
                    Match::new(range, vec![value])
                })
            }
        };
    });
}

pub fn getMatchBlockSym() -> impl MatchType<Value = BlockSym> {
    // { expressions }
    return getMatchParenthesis(ParenthesisType::Curly, |module| {
        getMatchSymbolsAll().getMatch(module.getModulePos(0)).map(|matchValue| {
            let (range, symbolVec) = matchValue.take();
            return BlockSym {
                range,
                symbolVec,
            };
        })
    });
}

pub fn getMatchBreakSym() -> impl MatchType<Value = BreakSym> {
    // break
    // break label
    return getMappedMatch(
        (
            getMatchKeyword(Keyword::Break), // break
            OptionalMatch::new(getMatchIdentifier()), // label
        ), |range, (_, label)|
            Ok(BreakSym {
                range,
                label: label.map(|identifier| Label {
                    identifier,
                }),
            }),
    );
}

pub fn getMatchContinueSym() -> impl MatchType<Value = ContinueSym> {
    // continue
    // continue label
    return getMappedMatch(
        (
            getMatchKeyword(Keyword::Continue), // continue
            OptionalMatch::new(getMatchIdentifier()), // label
        ), |range, (_, label)|
            Ok(ContinueSym {
                range,
                label: label.map(|identifier| Label {
                    identifier,
                }),
            }),
    );
}

pub fn getMatchClassMember() -> impl MatchType<Value = ClassMember> {
    return getMatchAnyOf(&[
        // type name = value
        MatchOption::new(
            (
                OptionalMatch::new(getMatchVisibility()),
                OptionalMatch::new(getMatchKeyword(Keyword::Static)),
                getMatchType(), // type
                getMatchIdentifier(), // name
                OptionalMatch::new((
                    getMatchOperator(Operator::AssignEq),
                    getMatchExpr()
                ))
            ), |_, (visibility, staticOption, typeName, name, value)|
                Ok(if staticOption.is_some() {
                    ClassMember::StaticFieldDefinition(ClassStaticFieldDefinition {
                        name,
                        visibility: visibility.unwrap_or(Visibility::Private),
                        typeName: Some(typeName),
                        defaultValue: value.map(|(_, v)| v),
                    })
                } else {
                    ClassMember::FieldDefinition(ClassFieldDefinition {
                        name,
                        visibility: visibility.unwrap_or(Visibility::Private),
                        typeName: Some(typeName),
                        defaultValue: value.map(|(_, v)| v),
                    })
                }),
        ),
        // let name = value
        MatchOption::new(
            (
                OptionalMatch::new(getMatchVisibility()),
                OptionalMatch::new(getMatchKeyword(Keyword::Static)),
                getMatchKeyword(Keyword::Let),
                getMatchIdentifier(), // name
                getMatchOperator(Operator::AssignEq),
                getMatchExpr()
            ), |_, (visibility, staticOption, _, name, _, value)|
                Ok(if staticOption.is_some() {
                    ClassMember::StaticFieldDefinition(ClassStaticFieldDefinition {
                        name,
                        visibility: visibility.unwrap_or(Visibility::Private),
                        typeName: None,
                        defaultValue: Some(value),
                    })
                } else {
                    ClassMember::FieldDefinition(ClassFieldDefinition {
                        name,
                        visibility: visibility.unwrap_or(Visibility::Private),
                        typeName: None,
                        defaultValue: Some(value),
                    })
                }),
        ),
        // function definition
        // returntype name() { expressions }
        MatchOption::new(
            getMatchFunctionDefinitionSym(),
            |_, function| Ok(ClassMember::FunctionDefinition(function)),
        ),
    ], |pos, mut options, err| {
        return if options.is_empty() {
            Err(ASTError::MatchOptionsFailed(pos, err))
        } else {
            let index = resolveClassDefinitionConflict(pos, options.iter())?;
            return Ok(options.swap_remove(index));
        };
    });
}

pub fn getMatchClassDefinitionSym() -> impl MatchType<Value = ClassDefinitionSym> {
    return getMappedMatch(
        // visibility class name { ... }
        (
            OptionalMatch::new(getMatchVisibility()),
            getMatchKeyword(Keyword::Class),
            getMatchIdentifier(), // name
            getMatchParenthesis(ParenthesisType::Curly, |module| {
                return getMatchAll(getMatchClassMember()).getMatch(module.getModulePos(0)).map(|v| v.take().1);
            })
        ), |range, (visibility, _, name, classMembers)| {
            let mut fields = Vec::new();
            let mut methods = Vec::new();
            let mut staticFields = Vec::new();

            for member in classMembers {
                match member {
                    ClassMember::FieldDefinition(field) => fields.push(field),
                    ClassMember::FunctionDefinition(function) => methods.push(function),
                    ClassMember::StaticFieldDefinition(field) => staticFields.push(field),
                }
            }

            Ok(ClassDefinitionSym {
                visibility: visibility.unwrap_or(Visibility::Private),
                range,
                name,
                fields,
                methods,
                staticFields,
                // inherited: vec![],
            })
        });
}

pub fn getMatchFunctionParameter() -> impl MatchType<Value = FunctionParameter> {
    // type name
    // type name = expr
    return getMappedMatch(
        (
            getMatchType(), // type
            getMatchIdentifier(), // name
            OptionalMatch::new(getMappedMatch(
                // default value
                (
                    getMatchOperator(Operator::AssignEq),
                    getMatchExpr()
                ), |_, (_, expr)| {
                    Ok(expr)
                },
            ))
        ), |_, (typeName, parameterName, defaultExpr)| {
            return Ok(FunctionParameter {
                typeName,
                parameterName,
                defaultExpr,
            });
        });
}

pub fn getMatchFunctionDefinitionSym() -> impl MatchType<Value = FunctionDefinitionSym> {
    // visibility returnType functionName(args) { expressions}
    return getMappedMatch(
        (
            OptionalMatch::new(getMatchVisibility()),
            getRepeatingMatch(0, getMatchFunctionAttribute()),
            getMatchType(), // return type
            getMatchIdentifier(), // function name
            getMatchParenthesis(ParenthesisType::Rounded, |module| {
                fn parseFunctionParameter(module: &Rc<Module>) -> Result<FunctionParameter, ASTError> {
                    let matchValue = getMatchFunctionParameter().getMatch(module.getModulePos(0))?;
                    if matchValue.getRange().getEndIndex() != module.getTokenVector().len() {
                        return Err(ASTError::ExpectedExclusive(matchValue.getRange().getEndPos(), None));
                    }
                    return Ok(matchValue.take().1);
                }

                return match module.getTokenVector().as_slice() {
                    [] => Ok(Vec::new()),
                    [token] => {
                        if let TokenType::CommaList(tokens) = token.getTokenType() {
                            let mut parameterVec = Vec::new();
                            for module in tokens {
                                parameterVec.push(parseFunctionParameter(module)?);
                            }
                            Ok(parameterVec)
                        } else {
                            Ok(vec![parseFunctionParameter(module)?])
                        }
                    }
                    _ => Ok(vec![parseFunctionParameter(module)?]),
                };
            }), // args
            getMatchBlockSym(), // expressions
        ), |range, (visibility, attributeVec, returnType, functionName, parameters, functionBlock)| {
            return Ok(FunctionDefinitionSym {
                range,
                attributeVec,
                returnType,
                functionName,
                parameters,
                functionBlock,
                visibility: visibility.unwrap_or(Visibility::Private),
            });
        });
}

pub fn getMatchIfSym() -> impl MatchType<Value = IfSym> {
    // if condition { symbols }
    // if condition { symbols } else { symbols }
    return getMappedMatch(
        (
            getMatchKeyword(Keyword::If), // if
            getMatchExpr(), // condition
            getMatchSymbol(), // symbol
            OptionalMatch::new(getMappedMatch(
                (
                    getMatchKeyword(Keyword::Else), // else
                    getMatchSymbol() // symbols
                ), |_, (_, expr)| Ok(expr))
            ),
        ), |range, (_, condition, symbol, elseExpr)| {
            Ok(IfSym {
                symbol: Box::new(symbol),
                condition,
                range,
                elseExpr: elseExpr.map(|symbol| Box::new(ElseSym {
                    symbol,
                })),
            })
        });
}

pub fn getMatchWhileSym() -> impl MatchType<Value = WhileLoop> {
    // label: while condition { symbols }
    // while condition { symbols }
    return getMappedMatch(
        (
            OptionalMatch::new((getMatchIdentifier(), getMatchOperator(Operator::Colon))),
            getMatchKeyword(Keyword::While), // while
            getMatchExpr(), // condition
            getMatchSymbol(), // symbol
        ), |range, (label, _, condition, symbol)| {
            Ok(WhileLoop {
                symbol: Box::new(symbol),
                condition,
                range,
                label: label.map(|(identifier, _)| Label {
                    identifier,
                }),
            })
        });
}

pub fn getMatchReturnSym() -> impl MatchType<Value = ReturnSym> {
    // return
    // return value
    return getMappedMatch(
        (
            getMatchKeyword(Keyword::Return), // return
            OptionalMatch::new(
                getMatchExpr() // value
            ),
        ), |range, (_, value)| {
            Ok(ReturnSym {
                range,
                value,
            })
        });
}

pub fn getMatchImportSym() -> impl MatchType<Value = ImportSym> {
    // import name
    // import name as name
    return getMappedMatch(
        (
            getMatchKeyword(Keyword::Import),
            getMatchIdentifier(), // import name
            OptionalMatch::new(getMappedMatch(
                (
                    getMatchKeyword(Keyword::As), getMatchIdentifier() // alias
                ), |_, ((), name)| Ok(name))
            )
        ), |range, (_, packageName, localName)| Ok(ImportSym {
            range,
            packageName,
            localName,
        }));
}

pub fn getMatchConstructorCallExpr() -> impl MatchType<Value = ConstructorCallExpr> {
    // new Type(args)
    return getMappedMatch(
        (
            getMatchKeyword(Keyword::New),
            getMatchType(), // name
            getMatchParenthesis(ParenthesisType::Rounded, |module| getMatchExprCommaList().getMatch(module.getModulePos(0))),
        ), |range, (_, typeName, argVec)| Ok(ConstructorCallExpr {
            range,
            typeName,
            argVec: argVec.take().1,
        }),
    );
}

pub fn getMatchFunctionCallExpr() -> impl MatchType<Value = FunctionCallExpr> {
    // functionName(args)
    return getMappedMatch(
        (
            getMatchIdentifier(), // name
            getMatchParenthesis(ParenthesisType::Rounded, |module| getMatchExprCommaList().getMatch(module.getModulePos(0))),
        ), |range, (functionName, argVec)| Ok(FunctionCallExpr {
            range,
            functionName,
            argVec: argVec.take().1,
        }),
    );
}

pub fn getMatchOperatorExpr() -> impl MatchType<Value = OperatorExpr> {
    // only match variable declaration if it is the first expr provided
    return getMappedMatch(
        (OptionalMatch::new(getMatchExcludingExpr(true, false)),
         getRepeatingMatch(1, getMatchOneOf(&[
             MatchOption::new(getMatchExcludingExpr(true, true), |_, expression|
                 Ok(OperationComponent::Expression(expression))),
             MatchOption::new(
                 getMatchFrom(format!("Operator"), |pos| {
                     return if let TokenType::Operator(operator) = pos.getToken().getTokenType() {
                         Ok(Match::new(pos.getRangeWithLength(1), OperationComponent::Operator(pos.getRangeWithLength(1), *operator)))
                     } else {
                         Err(ASTError::ExpectedTokenDiscriminant(pos, TokenTypeDiscriminants::Operator))
                     };
                 }),
                 |_, component| Ok(component),
             )
         ]))), |_, (first, mut components)| {
            if let Some(first) = first {
                components.insert(0, OperationComponent::Expression(first));
            }
            OperatorExpr::getFromInfix(components)
        });
}

pub fn getMatchVariableDeclarationExpr() -> impl MatchType<Value = VariableDeclarationExpr> {
    // let name
    // type name
    return getMatchOneOf(&[
        MatchOption::new(getMappedMatch(
            (
                getMatchKeyword(Keyword::Let), // let
                getMatchIdentifier() // name
            ), |range, (_, variableName)| Ok(VariableDeclarationExpr {
                range,
                variableName,
                explicitType: None,
            })), |_, v| Ok(v)),
        MatchOption::new(getMappedMatch(
            (
                getMatchType(), // type
                getMatchIdentifier() // name
            ), |range, (typeName, variableName)| Ok(VariableDeclarationExpr {
                range,
                variableName,
                explicitType: Some(typeName),
            })), |_, v| Ok(v))
    ]);
}

pub fn getMatchVariableExpr() -> impl MatchType<Value = VariableExpr> {
    // name
    return getMappedMatch(getMatchIdentifier(), |range, _| Ok(VariableExpr {
        range,
    }));
}

pub fn getMatchLiteralArray() -> impl MatchType<Value = LiteralArray> {
    // [a, b, c]
    return getMatchParenthesis(ParenthesisType::Square, |parenthesisModule| {
        getMatchExprCommaList().getMatch(parenthesisModule.getModulePos(0)).map(|matchValue| {
            let (range, exprVec) = matchValue.take();
            return LiteralArray {
                range,
                exprVec,
            };
        })
    });
}

pub fn getMatchLiteralBool() -> impl MatchType<Value = LiteralBool> {
    // true
    // false
    return getMatchOneOf(&[
        MatchOption::new(getMatchKeyword(Keyword::True), |range, _| Ok(LiteralBool {
            range: range.to_owned(),
            value: true,
        })),
        MatchOption::new(getMatchKeyword(Keyword::False), |range, _| Ok(LiteralBool {
            range: range.to_owned(),
            value: false,
        })),
    ]);
}

pub fn getMatchLiteralChar() -> impl MatchType<Value = LiteralChar> {
    // 'a'
    return getMatchQuote(QuoteType::Single, |range, fileRange| Ok(LiteralChar {
        range,
        value: {
            let source = fileRange.getSourceInRange();
            debug_assert_eq!(source.len(), 1);
            source.chars().next().unwrap() as u32
        },
    }));
}

fn isFloat(number: &FileRange) -> bool {
    return number.getSourceInRange().contains('.');
}

pub fn getMatchLiteralFloat() -> impl MatchType<Value = LiteralFloat> {
    // 0.0
    return getMatchFrom(format!("LiteralFloat"), |pos| {
        if let TokenType::Number = pos.getToken().getTokenType() {
            if isFloat(pos.getToken().getSourceRange()) {
                let range = pos.getRangeWithLength(1);
                let source = pos.getToken().getSourceRange().getSourceInRange();
                return Ok(Match::new(range.to_owned(), LiteralFloat {
                    range,
                    value: f64::from_str(source).expect(&format!("failed to parse float \"{source}\"")),
                }));
            }
        }
        return Err(ASTError::ExpectedTokenDiscriminant(pos, TokenTypeDiscriminants::Number));
    });
}

pub fn getMatchLiteralInteger() -> impl MatchType<Value = LiteralInteger> {
    // 0
    return getMatchFrom(format!("LiteralInteger"), |pos| {
        if let TokenType::Number = pos.getToken().getTokenType() {
            if !isFloat(pos.getToken().getSourceRange()) {
                let range = pos.getRangeWithLength(1);
                let source = pos.getToken().getSourceRange().getSourceInRange();
                return Ok(Match::new(range.to_owned(), LiteralInteger {
                    range,
                    value: i64::from_str(source).expect(&format!("failed to parse integer \"{source}\"")),
                }));
            }
        }
        return Err(ASTError::ExpectedTokenDiscriminant(pos, TokenTypeDiscriminants::Number));
    });
}

pub fn getMatchLiteralString() -> impl MatchType<Value = LiteralString> {
    // "abc"
    return getMatchQuote(QuoteType::Double, |range, fileRange| Ok(LiteralString {
        range,
        fileRange: fileRange.to_owned(),
    }));
}

pub fn getMatchLiteralVoid() -> impl MatchType<Value = LiteralVoid> {
    // void
    return getMappedMatch(getMatchKeyword(Keyword::Void), |range, _| Ok(LiteralVoid {
        range,
    }));
}

pub fn getMatchLiteralTuple() -> impl MatchType<Value = LiteralTuple> {
    // (a, b, c)
    return getMatchParenthesis(ParenthesisType::Rounded, |module| {
        getMatchExprCommaList().getMatch(module.getModulePos(0)).map(|matchValue| {
            let (range, exprVec) = matchValue.take();
            return LiteralTuple {
                range,
                exprVec,
            };
        })
    });
}
