use std::rc::Rc;

use crate::ast::symbol::block::BlockSym;
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
use crate::ast::symbol::expr::operatorexpr::OperatorExpr;
use crate::ast::symbol::expr::variabledeclaration::VariableDeclarationExpr;
use crate::ast::symbol::expr::variableexpr::VariableExpr;
use crate::ast::symbol::function::{FunctionDefinitionSym, FunctionParameter};
use crate::ast::symbol::ifstatement::{ElseSym, IfSym};
use crate::ast::symbol::import::ImportSym;
use crate::ast::symbol::Symbol;
use crate::ast::tokensource::symbolparser::{getMatchFrom, getNestedMatch, Match, MatchType, OptionalMatch};
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

pub fn getMatchSymbol() -> impl MatchType<Value = Symbol> {
    return getMatchFrom(|pos| {
        todo!()
    });
}

pub fn getMatchExpr() -> impl MatchType<Value = Expr> {
    return getMatchFrom(|pos| {
        todo!()
    });
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
    return getMatchFrom(|pos| {
        todo!()
    });
}

pub fn getMatchBreakSym() -> impl MatchType<Value = Expr> {
    // break
    // break label
    return getMatchFrom(|pos| {
        todo!()
    });
}

pub fn getMatchClassDefinitionSym() -> impl MatchType<Value = Expr> {
    return getMatchFrom(|pos| {
        todo!()
    });
}

pub fn getMatchFunctionParameter() -> impl MatchType<Value = FunctionParameter> {
    // type name
    // type name = expr
    return getNestedMatch(
        (
            getMatchIdentifier(), // type
            getMatchIdentifier(), // name
            OptionalMatch::new(getNestedMatch(
                // default value
                (
                    getMatchOperator(Operator::AssignEq),
                    getMatchExpr()
                ), |_, (_, expr)| {
                    Some(expr)
                },
            ))
        ), |range, (typeName, parameterName, defaultExpr)| {
            return Some(FunctionParameter {
                typeName,
                parameterName,
                defaultExpr,
            });
        });
}

pub fn getMatchFunctionDefinitionSym() -> impl MatchType<Value = FunctionDefinitionSym> {
    // visibility returnType functionName(args) { expressions}
    return getNestedMatch(
        (
            OptionalMatch::new(getMatchVisibility()),
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
        ), |range, (visibility, returnType, functionName, parameters, functionBlock)| {
            return Some(FunctionDefinitionSym {
                range,
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
    return getNestedMatch(
        (
            getMatchKeyword(Keyword::If), // if
            getMatchExpr(), // condition
            getMatchSymbol(), // symbol
            OptionalMatch::new(getNestedMatch(
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
    return getNestedMatch(
        (
            getMatchKeyword(Keyword::Import),
            getMatchIdentifier(),
            OptionalMatch::new(getNestedMatch(
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
    return getNestedMatch(
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
    return getMatchFrom(|pos| {
        todo!()
    });
}

pub fn getMatchVariableDeclaration() -> impl MatchType<Value = VariableDeclarationExpr> {
    // let name
    // type name
    return getMatchFrom(|pos| {
        todo!()
    });
}

pub fn getMatchVariable() -> impl MatchType<Value = VariableExpr> {
    // name
    return getNestedMatch((getMatchIdentifier(), ), |range, _| Some(VariableExpr {
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
    return getMatchFrom(|pos| {
        todo!()
    });
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

pub fn getMatchLiteralFloat() -> impl MatchType<Value = LiteralFloat> {
    // 0.0
    return getMatchFrom(|pos| {
        todo!()
    });
}

pub fn getMatchLiteralInteger() -> impl MatchType<Value = LiteralInteger> {
    // 0
    return getMatchFrom(|pos| {
        todo!()
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
    return getNestedMatch((getMatchKeyword(Keyword::Void)), |range, _| Some(LiteralVoid {
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
