use std::rc::Rc;

use crate::ast::symbol::block::BlockSym;
use crate::ast::symbol::breaksym::BreakSym;
use crate::ast::symbol::classdefinition::{ClassDefinitionSym, ClassFieldDefinition, ClassMember, ClassStaticFieldDefinition};
use crate::ast::symbol::expr::Expr;
use crate::ast::symbol::expr::functioncall::FunctionCallExpr;
use crate::ast::symbol::expr::literal::literalarray::LiteralArray;
use crate::ast::symbol::expr::literal::literalbool::LiteralBool;
use crate::ast::symbol::expr::literal::literalchar::LiteralChar;
use crate::ast::symbol::expr::literal::literalFloat::LiteralFloat;
use crate::ast::symbol::expr::literal::literalinteger::LiteralInteger;
use crate::ast::symbol::expr::literal::literalstring::LiteralString;
use crate::ast::symbol::expr::literal::literaltuple::LiteralTuple;
use crate::ast::symbol::expr::literal::literalvoid::LiteralVoid;
use crate::ast::symbol::expr::operatorexpr::{OperationComponent, OperatorExpr};
use crate::ast::symbol::expr::parenthesisexpr::ParenthesisExpr;
use crate::ast::symbol::expr::variabledeclaration::VariableDeclarationExpr;
use crate::ast::symbol::expr::variableexpr::VariableExpr;
use crate::ast::symbol::function::{FunctionAttribute, FunctionDefinitionSym, FunctionParameter};
use crate::ast::symbol::ifstatement::{ElseSym, IfSym};
use crate::ast::symbol::import::ImportSym;
use crate::ast::symbol::looptype::label::Label;
use crate::ast::symbol::Symbol;
use crate::ast::tokensource::conflictresolution::resolveConflict;
use crate::ast::tokensource::symbolparser::{getMappedMatch, getMatchAnyOf, getMatchFrom, getMatchOneOf, getRepeatingMatch, Match, MatchOption, MatchType, OptionalMatch};
use crate::module::{FileRange, Keyword, Module, Operator, ParenthesisType, QuoteType, TokenType};
use crate::module::modulepos::{ModulePos, ModuleRange};
use crate::module::visibility::Visibility;

pub fn getMatchKeyword(keyword: Keyword) -> impl MatchType<Value = ()> {
    return getMatchFrom(move |pos| {
        if let TokenType::Keyword(value) = pos.getToken().getTokenType() {
            if &keyword == value {
                return Some(Match::new(pos.getRange(1), ()));
            }
        }

        return None;
    });
}

pub fn getMatchOperator(operator: Operator) -> impl MatchType<Value = ()> {
    return getMatchFrom(move |pos| {
        if let TokenType::Operator(value) = pos.getToken().getTokenType() {
            if &operator == value {
                return Some(Match::new(pos.getRange(1), ()));
            }
        }

        return None;
    });
}

pub fn getMatchFunctionAttribute() -> impl MatchType<Value = FunctionAttribute> {
    return getMatchFrom(|pos| {
        if let TokenType::Keyword(keyword) = pos.getToken().getTokenType() {
            return Some(Match::new(pos.getRange(1), FunctionAttribute::fromKeyword(*keyword)?));
        }

        return None;
    });
}

pub fn getMatchVisibility() -> impl MatchType<Value = Visibility> {
    return getMatchFrom(|pos| {
        if let TokenType::Keyword(keyword) = pos.getToken().getTokenType() {
            return Some(Match::new(pos.getRange(1), Visibility::fromKeyword(*keyword)?));
        }

        return None;
    });
}

pub fn getMatchIdentifier() -> impl MatchType<Value = ModulePos> {
    return getMatchFrom(|pos| {
        if let TokenType::Identifier = pos.getToken().getTokenType() {
            return Some(Match::new(pos.getRange(1), pos));
        }

        return None;
    });
}

pub fn getMatchParenthesis<T>(parenthesis: ParenthesisType, function: impl 'static + Clone + Fn(&Rc<Module>) -> Option<T>) -> impl MatchType<Value = T> {
    return getMatchFrom(move |pos| {
        if let TokenType::Parenthesis(parenthesisType, module) = pos.getToken().getTokenType() {
            if &parenthesis == parenthesisType {
                return Some(Match::new(pos.getRange(1), function(module)?));
            }
        }

        return None;
    });
}

pub fn getMatchQuote<T>(quote: QuoteType, function: impl 'static + Clone + Fn(ModuleRange, &FileRange) -> Option<T>) -> impl MatchType<Value = T> {
    return getMatchFrom(move |pos| {
        if let TokenType::String(quoteType, fileRange) = pos.getToken().getTokenType() {
            if &quote == quoteType {
                let range = pos.getRange(1);
                return Some(Match::new(range.to_owned(), function(range, fileRange)?));
            }
        }

        return None;
    });
}

pub fn getMatchAll<S>(matchType: impl 'static + Clone + MatchType<Value = S>) -> impl MatchType<Value = Vec<S>> {
    return getMatchFrom(move |mut pos| {
        let startIndex = pos.getTokenIndex();
        let endPos = pos.getModule().getModulePos(pos.getModule().getTokenVector().len());
        let mut matchVec = Vec::new();
        while pos != endPos {
            let _debugPos = pos.to_owned();
            let matchValue = matchType.getMatch(pos)?;
            debug_assert_ne!(_debugPos, matchValue.getRange().getEndPos(), "zero length symbol matched");
            pos = matchValue.getRange().getEndPos().to_owned();
            matchVec.push(matchValue.take().1);
        }
        return Some(Match::new(pos.getModule().getModuleRange(startIndex..endPos.getTokenIndex()), matchVec));
    });
}

pub fn getMatchSymbolsAll() -> impl MatchType<Value = Vec<Symbol>> {
    return getMatchAll(getMatchSymbol());
}

pub fn getMatchSymbol() -> impl MatchType<Value = Symbol> {
    return getMatchAnyOf(&[
        MatchOption::new(getMatchBlockSym(), |_, v| Some(Symbol::Block(v))),
        MatchOption::new(getMatchBreakSym(), |_, v| Some(Symbol::Break(v))),
        MatchOption::new(getMatchClassDefinitionSym(), |_, v| Some(Symbol::ClassDefinition(v))),
        MatchOption::new(getMatchFunctionDefinitionSym(), |_, v| Some(Symbol::FunctionDefinition(v))),
        MatchOption::new(getMatchIfSym(), |_, v| Some(Symbol::IfSym(v))),
        MatchOption::new(getMatchImportSym(), |_, v| Some(Symbol::ImportSym(v))),
        MatchOption::new(getMatchFunctionCallExpr(), |_, v| Some(Symbol::FunctionCall(v))),
        MatchOption::new(getMatchOperatorExpr(), |_, v| Some(Symbol::Operator(v))),
        MatchOption::new(getMatchParenthesisExpr(), |_, v| Some(Symbol::Parenthesis(v))),
        MatchOption::new(getMatchVariableDeclarationExpr(), |_, v| Some(Symbol::VariableDeclaration(v))),
        MatchOption::new(getMatchVariableExpr(), |_, v| Some(Symbol::Variable(v))),
        MatchOption::new(getMatchLiteralArray(), |_, v| Some(Symbol::LiteralArray(v))),
        MatchOption::new(getMatchLiteralBool(), |_, v| Some(Symbol::LiteralBool(v))),
        MatchOption::new(getMatchLiteralChar(), |_, v| Some(Symbol::LiteralChar(v))),
        MatchOption::new(getMatchLiteralFloat(), |_, v| Some(Symbol::LiteralFloat(v))),
        MatchOption::new(getMatchLiteralInteger(), |_, v| Some(Symbol::LiteralInteger(v))),
        MatchOption::new(getMatchLiteralString(), |_, v| Some(Symbol::LiteralString(v))),
        MatchOption::new(getMatchLiteralVoid(), |_, v| Some(Symbol::LiteralVoid(v))),
        MatchOption::new(getMatchLiteralTuple(), |_, v| Some(Symbol::LiteralTuple(v))),
    ], |mut matchVec| {
        let index = resolveConflict(matchVec.iter().map(|symbolMatch| {
            symbolMatch.getValue()
        })).ok()?;
        return Some(matchVec.swap_remove(index));
    });
}

pub fn getMatchExcludingExpr(excludeOperator: bool) -> impl MatchType<Value = Expr> {
    return getMatchOneOf(&[
        MatchOption::new(getMatchFunctionCallExpr(), |_, v| Some(Box::new(v) as Expr)),
        if excludeOperator {
            MatchOption::new(getMatchFrom(|_| None), |_, _: u8| None)
        } else {
            MatchOption::new(getMatchOperatorExpr(), |_, v| Some(Box::new(v) as Expr))
        },
        MatchOption::new(getMatchParenthesisExpr(), |_, v| Some(Box::new(v) as Expr)),
        MatchOption::new(getMatchVariableDeclarationExpr(), |_, v| Some(Box::new(v) as Expr)),
        MatchOption::new(getMatchVariableExpr(), |_, v| Some(Box::new(v) as Expr)),
        MatchOption::new(getMatchLiteralArray(), |_, v| Some(Box::new(v) as Expr)),
        MatchOption::new(getMatchLiteralBool(), |_, v| Some(Box::new(v) as Expr)),
        MatchOption::new(getMatchLiteralChar(), |_, v| Some(Box::new(v) as Expr)),
        MatchOption::new(getMatchLiteralFloat(), |_, v| Some(Box::new(v) as Expr)),
        MatchOption::new(getMatchLiteralInteger(), |_, v| Some(Box::new(v) as Expr)),
        MatchOption::new(getMatchLiteralString(), |_, v| Some(Box::new(v) as Expr)),
        MatchOption::new(getMatchLiteralVoid(), |_, v| Some(Box::new(v) as Expr)),
        MatchOption::new(getMatchLiteralTuple(), |_, v| Some(Box::new(v) as Expr)),
    ]);
}

pub fn getMatchExpr() -> impl MatchType<Value = Expr> {
    return getMatchExcludingExpr(false);
}

pub fn getMatchExprCommaList(exclusive: bool) -> impl MatchType<Value = Vec<Expr>> {
    return getMatchFrom(move |pos| {
        if exclusive && pos.getModule().getTokenVector().len() > 1 {
            return None;
        }

        let token = pos.getToken();
        return if let TokenType::CommaList(moduleVec) = token.getTokenType() {
            let mut exprVec = Vec::new();
            for module in moduleVec {
                let matchValue = getMatchExpr().getMatch(module.getModulePos(0))?;
                if matchValue.getRange().getEndIndex() != module.getTokenVector().len() {
                    return None;
                }
                exprVec.push(matchValue.take().1);
            }
            Some(Match::new(pos.getRange(1), exprVec))
        } else {
            None
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
            Some(BreakSym {
                range,
                label: label.map(|identifier| Label {
                    identifier,
                }),
            }),
    );
}

pub fn getMatchClassMember() -> impl MatchType<Value = ClassMember> {
    return getMatchOneOf(&[
        // type name = value
        MatchOption::new(
            (
                OptionalMatch::new(getMatchVisibility()),
                OptionalMatch::new(getMatchKeyword(Keyword::Static)),
                getMatchIdentifier(), // type
                getMatchIdentifier(), // name
                OptionalMatch::new((
                    getMatchOperator(Operator::AssignEq),
                    getMatchExpr()
                ))
            ), |_, (visibility, staticOption, typeName, name, value)|
                Some(if staticOption.is_some() {
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
                Some(if staticOption.is_some() {
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
        MatchOption::new(
            getMatchFunctionDefinitionSym(),
            |_, function| Some(ClassMember::FunctionDefinition(function)),
        ),
    ]);
}

pub fn getMatchClassDefinitionSym() -> impl MatchType<Value = ClassDefinitionSym> {
    return getMappedMatch(
        (
            OptionalMatch::new(getMatchVisibility()),
            getMatchKeyword(Keyword::Class),
            getMatchIdentifier(),
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

            Some(ClassDefinitionSym {
                visibility: visibility.unwrap_or(Visibility::Private),
                range,
                name,
                fields,
                methods,
                staticFields,
                inherited: vec![],
            })
        });
}

pub fn getMatchFunctionParameter() -> impl MatchType<Value = FunctionParameter> {
    // type name
    // type name = expr
    return getMappedMatch(
        (
            getMatchIdentifier(), // type
            getMatchIdentifier(), // name
            OptionalMatch::new(getMappedMatch(
                // default value
                (
                    getMatchOperator(Operator::AssignEq),
                    getMatchExpr()
                ), |_, (_, expr)| {
                    Some(expr)
                },
            ))
        ), |_, (typeName, parameterName, defaultExpr)| {
            return Some(FunctionParameter {
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
            getMatchIdentifier(), // return type
            getMatchIdentifier(), // function name
            getMatchParenthesis(ParenthesisType::Rounded, |module| {
                return match module.getTokenVector().as_slice() {
                    [token] => {
                        if let TokenType::CommaList(tokens) = token.getTokenType() {
                            let mut parameterVec = Vec::new();
                            for module in tokens {
                                let matchValue = getMatchFunctionParameter().getMatch(module.getModulePos(0))?;
                                if matchValue.getRange().getEndIndex() != module.getTokenVector().len() {
                                    return None;
                                }
                                parameterVec.push(matchValue.take().1);
                            }
                            Some(parameterVec)
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
            }), // args
            getMatchBlockSym(), // expressions
        ), |range, (visibility, attributeVec, returnType, functionName, parameters, functionBlock)| {
            return Some(FunctionDefinitionSym {
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
                ), |_, (_, expr)| Some(expr))
            ),
        ), |range, (_, condition, symbol, elseExpr)| {
            Some(IfSym {
                symbol: Box::new(symbol),
                condition,
                range,
                elseExpr: elseExpr.map(|symbol| Box::new(ElseSym {
                    symbol,
                })),
            })
        });
}

pub fn getMatchImportSym() -> impl MatchType<Value = ImportSym> {
    // import name
    // import name as name
    return getMappedMatch(
        (
            getMatchKeyword(Keyword::Import),
            getMatchIdentifier(),
            OptionalMatch::new(getMappedMatch(
                (
                    getMatchKeyword(Keyword::As), getMatchIdentifier()
                ), |_, ((), name)| Some(name))
            )
        ), |range, (_, packageName, localName)| Some(ImportSym {
            range,
            packageName,
            localName,
        }));
}

pub fn getMatchFunctionCallExpr() -> impl MatchType<Value = FunctionCallExpr> {
    // functionName(args)
    return getMappedMatch(
        (
            getMatchIdentifier(),
            getMatchExprCommaList(true),
        ), |range, (functionName, argVec)| Some(FunctionCallExpr {
            range,
            functionName,
            argVec,
        }),
    );
}

pub fn getMatchOperatorExpr() -> impl MatchType<Value = OperatorExpr> {
    return getMappedMatch(getRepeatingMatch(1, getMatchOneOf(&[
        MatchOption::new(getMatchExcludingExpr(true), |_, expression|
            Some(OperationComponent::Expression(expression))),
        MatchOption::new(
            getMatchFrom(|pos| {
                return if let TokenType::Operator(operator) = pos.getToken().getTokenType() {
                    Some(Match::new(pos.getRange(1), OperationComponent::Operator(*operator)))
                } else {
                    None
                };
            }),
            |_, component| Some(component),
        )
    ])), |_, components: Vec<OperationComponent>| OperatorExpr::getFromComponents(components));
}

pub fn getMatchParenthesisExpr() -> impl MatchType<Value = ParenthesisExpr> {
    // ()
    return getMatchParenthesis(ParenthesisType::Rounded, |module| {
        let (range, expression) = getMatchExpr().getMatch(module.getModulePos(0))?.take();
        if range.getEndIndex() != module.getTokenVector().len() {
            return None;
        }
        return Some(ParenthesisExpr {
            range,
            expression,
        });
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
            ), |range, (_, variableName)| Some(VariableDeclarationExpr {
                range,
                variableName,
                explicitType: None,
            })), |_, v| Some(v)),
        MatchOption::new(getMappedMatch(
            (
                getMatchIdentifier(), // type
                getMatchIdentifier() // name
            ), |range, (typeName, variableName)| Some(VariableDeclarationExpr {
                range,
                variableName,
                explicitType: Some(typeName),
            })), |_, v| Some(v))
    ]);
}

pub fn getMatchVariableExpr() -> impl MatchType<Value = VariableExpr> {
    // name
    return getMappedMatch((getMatchIdentifier(), ), |range, _| Some(VariableExpr {
        range,
    }));
}

pub fn getMatchLiteralArray() -> impl MatchType<Value = LiteralArray> {
    // [a, b, c]
    return getMatchParenthesis(ParenthesisType::Square, |parenthesisModule| {
        getMatchExprCommaList(true).getMatch(parenthesisModule.getModulePos(0)).map(|matchValue| {
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
        MatchOption::new(getMatchKeyword(Keyword::True), |range, _| Some(LiteralBool {
            range: range.to_owned(),
            value: true,
        })),
        MatchOption::new(getMatchKeyword(Keyword::False), |range, _| Some(LiteralBool {
            range: range.to_owned(),
            value: false,
        })),
    ]);
}

pub fn getMatchLiteralChar() -> impl MatchType<Value = LiteralChar> {
    // 'a'
    return getMatchQuote(QuoteType::Single, |range, fileRange| Some(LiteralChar {
        range,
        character: {
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
    return getMatchFrom(|pos| {
        if let TokenType::Number = pos.getToken().getTokenType() {
            if isFloat(pos.getToken().getSourceRange()) {
                let range = pos.getRange(1);
                return Some(Match::new(range.to_owned(), LiteralFloat {
                    range,
                }));
            }
        }
        return None;
    });
}

pub fn getMatchLiteralInteger() -> impl MatchType<Value = LiteralInteger> {
    // 0
    return getMatchFrom(|pos| {
        if let TokenType::Number = pos.getToken().getTokenType() {
            if !isFloat(pos.getToken().getSourceRange()) {
                let range = pos.getRange(1);
                return Some(Match::new(range.to_owned(), LiteralInteger {
                    range,
                }));
            }
        }
        return None;
    });
}

pub fn getMatchLiteralString() -> impl MatchType<Value = LiteralString> {
    // "abc"
    return getMatchQuote(QuoteType::Double, |range, fileRange| Some(LiteralString {
        range,
        fileRange: fileRange.to_owned(),
    }));
}

pub fn getMatchLiteralVoid() -> impl MatchType<Value = LiteralVoid> {
    // void
    return getMappedMatch(getMatchKeyword(Keyword::Void), |range, _| Some(LiteralVoid {
        range,
    }));
}

pub fn getMatchLiteralTuple() -> impl MatchType<Value = LiteralTuple> {
    // (a, b, c)
    return getMatchParenthesis(ParenthesisType::Rounded, |module| {
        getMatchExprCommaList(true).getMatch(module.getModulePos(0)).map(|matchValue| {
            let (range, exprVec) = matchValue.take();
            return LiteralTuple {
                range,
                exprVec,
            };
        })
    });
}
