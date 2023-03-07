use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

use crate::ast::ASTError;
use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::symbol::expr::{Expr, ExprType};
use crate::module::modulepos::ModuleRange;
use crate::module::Operator;

#[derive(Debug)]
pub struct OperatorExpr {
    pub range: ModuleRange,
    pub operands: Box<[Expr]>,
    pub operator: Operator,
}

impl OperatorExpr {
    fn getFromPostfix(components: Vec<OperationComponent>) -> Result<Self, ASTError> {
        println!("{components:?}");

        let mut operandStack = Vec::new();

        for component in components {
            match component {
                OperationComponent::Expression(expression) => operandStack.push(expression),
                OperationComponent::Operator(mut range, operator) => {
                    let mut operands = Vec::new();
                    for _ in 0..operator.getOperands() {
                        let expression = operandStack.pop().expect("expected operand");
                        range = range.getCombined(expression.getRange());
                        operands.push(expression);
                    }
                    operands.reverse();
                    let expression = Self {
                        range,
                        operands: operands.into_boxed_slice(),
                        operator,
                    };
                    operandStack.push(Box::new(expression));
                }
            }
        }

        debug_assert_eq!(1, operandStack.len());
        return Ok(*operandStack.remove(0).downcast().map_err(|expr| ASTError::MatchFailed(expr.getRange().getEndPos().to_owned()))?);
    }

    fn getValidComponents(mut components: Vec<OperationComponent>) -> Result<Vec<OperationComponent>, ASTError> {
        #[derive(Copy, Clone)]
        enum NextComponent {
            Operator,
            Operand,
            Either,
        }

        let mut index = 0;
        let mut lastValidIndex = 0;
        let mut nextComponent = NextComponent::Either;
        let mut difference = 0;

        for component in &components {
            match component {
                OperationComponent::Operator(_, operator) => {
                    if let NextComponent::Operand = nextComponent {
                        break;
                    }
                    difference += operator.getOperands() as i32;
                }
                OperationComponent::Expression(_) => {
                    if let NextComponent::Operator = nextComponent {
                        break;
                    }
                    difference -= 1;
                }
            }

            index += 1;

            nextComponent = match difference.cmp(&0) {
                Ordering::Less => {
                    lastValidIndex = index;
                    NextComponent::Operator
                }
                Ordering::Equal => {
                    lastValidIndex = index;
                    // if difference is 0 the operator expression is valid
                    // operator expression is, itself, an expression to be used
                    difference -= 1;
                    NextComponent::Operator
                }
                Ordering::Greater => NextComponent::Operand
            }
        }

        components.resize_with(lastValidIndex, || unreachable!());
        return Ok(components);
    }

    pub fn getFromInfix(components: Vec<OperationComponent>) -> Result<Self, ASTError> {
        debug_assert_ne!(0, components.len());

        let startPos = match &components[0] {
            OperationComponent::Operator(range, _) => range.getStartPos(),
            OperationComponent::Expression(expr) => expr.getRange().getStartPos()
        };

        let components = Self::getValidComponents(components)?;
        if components.len() <= 1 {
            return Err(ASTError::MatchFailed(startPos));
        }

        let mut resultQueue = Vec::new();
        let mut operatorStack: Vec<(ModuleRange, Operator)> = Vec::new();

        for component in components {
            match component {
                OperationComponent::Expression(expression) => {
                    resultQueue.push(OperationComponent::Expression(expression));
                }
                OperationComponent::Operator(range, operator) => {
                    loop {
                        if let Some((lastRange, lastOperator)) = operatorStack.pop() {
                            match operator.getPrecedence().cmp(&lastOperator.getPrecedence()) {
                                Ordering::Greater => {
                                    operatorStack.push((lastRange, lastOperator));
                                    operatorStack.push((range, operator));
                                    break;
                                }
                                Ordering::Equal => {
                                    operatorStack.push((range, operator));
                                    resultQueue.push(OperationComponent::Operator(lastRange, lastOperator));
                                    break;
                                }
                                Ordering::Less => {
                                    resultQueue.push(OperationComponent::Operator(lastRange, lastOperator));
                                }
                            }
                        } else {
                            operatorStack.push((range, operator));
                            break;
                        }
                    }
                }
            }
        }

        operatorStack.reverse();
        resultQueue.extend(operatorStack.into_iter().map(|(range, operator)| OperationComponent::Operator(range, operator)));
        return Self::getFromPostfix(resultQueue);
    }

    pub fn binaryExpr(first: Expr, operator: Operator, second: Expr) -> Self {
        return Self {
            range: first.getRange().getCombined(second.getRange()),
            operands: vec![first, second].into_boxed_slice(),
            operator,
        };
    }

    pub fn unaryOperator(expr: Expr, operator: Operator, operatorRange: ModuleRange) -> Self {
        return Self {
            range: expr.getRange().getCombined(&operatorRange),
            operands: vec![expr].into_boxed_slice(),
            operator,
        };
    }
}

impl SymbolType for OperatorExpr {
    fn getRange(&self) -> &ModuleRange {
        return &self.range;
    }
}

impl ExprType for OperatorExpr {
    fn toSymbol(self: Box<Self>) -> Symbol {
        return Symbol::Operator(*self);
    }
}

pub enum OperationComponent {
    Operator(ModuleRange, Operator),
    Expression(Expr),
}

impl Debug for OperationComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", match self {
            OperationComponent::Operator(_, operator) => operator.getCharacters().to_owned(),
            OperationComponent::Expression(expr) => expr.getRange().getStartIndex().to_string(),
        });
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;
    use std::path::PathBuf;
    use std::rc::Rc;

    use crate::ast::ASTError;
    use crate::ast::symbol::expr::Expr;
    use crate::ast::symbol::expr::literal::literalinteger::LiteralInteger;
    use crate::ast::symbol::expr::operatorexpr::{OperationComponent, OperatorExpr};
    use crate::module::{FilePos, FileRange, Module, Operator, SourceFile, Token, TokenType};
    use crate::module::modulepos::ModulePos;

    fn getDummyToken() -> Token {
        return Token::new(TokenType::SemiColan, FILE_RANGE.with(|v| v.to_owned()));
    }

    thread_local! {
        static FILE_RANGE: FileRange = FileRange::new(FilePos::new(SourceFile::fromSource(PathBuf::new(), String::new()), 0), 0);
        static MODULE: Rc<Module> = Module::newFrom(vec![
            getDummyToken(), getDummyToken(), getDummyToken(), getDummyToken(),
            getDummyToken(), getDummyToken(), getDummyToken(), getDummyToken(),
            getDummyToken(), getDummyToken(), getDummyToken(), getDummyToken(),
        ]);
    }

    fn getPosIndex(index: usize) -> ModulePos {
        return MODULE.with(|module| module.getModulePos(index));
    }

    fn getExpr(index: usize) -> Expr {
        return Box::new(LiteralInteger {
            range: getPosIndex(index).getRangeWithLength(0),
        });
    }

    fn getExprComponent(index: usize) -> OperationComponent {
        return OperationComponent::Expression(getExpr(index));
    }

    fn getOperatorComponent(index: usize, operator: Operator) -> OperationComponent {
        return OperationComponent::Operator(getPosIndex(index).getRangeWithLength(0), operator);
    }

    fn checkEqualRef(expected: &mut OperatorExpr, provided: &mut OperatorExpr) {
        assert_eq!(provided.operator.getOperands(), provided.operands.len(), "expected {} operand(s), found {}\nExpected:\n{expected:#?}\n\nProvided:\n{provided:#?}", provided.operator.getOperands(), provided.operands.len());

        assert_eq!(expected.operator, provided.operator, "operator mismatch, expected {:?}, found {:?}\nExpected:\n{expected:#?}\n\nProvided:\n{provided:#?}", expected.operator, provided.operator);
        assert_eq!(expected.operands.len(), provided.operands.len(), "operand mismatch, expected {:?} operands, found {:?}\nExpected:\n{expected:#?}\n\nProvided:\n{provided:#?}", expected.operands.len(), provided.operands.len());

        for index in 0..expected.operands.len() {
            if expected.operands[index].downcast_ref::<LiteralInteger>().is_some() {
                assert_eq!(expected.operands[index].getRange(), provided.operands[index].getRange(), "operand mismatch, expected {:?}, found {:?}\nExpected:\n{expected:#?}\n\nProvided:\n{provided:#?}", expected.operands[index], provided.operands[index]);
            }

            let expectedDebug = format!("{expected:#?}");
            let providedDebug = format!("{provided:#?}");

            let expectedOperand: Option<&mut OperatorExpr> = (&mut expected.operands[index] as &mut dyn Any).downcast_mut();
            let providedOperand: Option<&mut OperatorExpr> = (&mut provided.operands[index] as &mut dyn Any).downcast_mut();

            if let Some(expectedOperand) = expectedOperand {
                let providedOperand = providedOperand.expect(&format!("operand mismatch, expected {expectedOperand:?}\nExpected:\n{expectedDebug}\n\nProvided:\n{providedDebug}"));
                checkEqualRef(expectedOperand, providedOperand);
            }
        }
    }

    fn checkEq(mut expected: OperatorExpr, provided: Result<OperatorExpr, ASTError>) {
        let mut provided = match provided {
            Ok(expr) => expr,
            Err(err) => panic!("expected {expected:#?}, found ASTError {}", err.getErrorMessage()),
        };

        checkEqualRef(&mut expected, &mut provided);
    }

    #[test]
    fn testAddition() {
        // a + b
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(1),
            getOperatorComponent(2, Operator::Plus),
            getExprComponent(3),
        ]);

        let expected = OperatorExpr::binaryExpr(getExpr(1), Operator::Plus, getExpr(3));

        checkEq(expected, expr);
    }

    #[test]
    fn testAdditionMultiple() {
        // a + b + c
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(1),
            getOperatorComponent(2, Operator::Plus),
            getExprComponent(3),
            getOperatorComponent(4, Operator::Plus),
            getExprComponent(5),
        ]);

        let expected = OperatorExpr::binaryExpr(
            Box::new(OperatorExpr::binaryExpr(getExpr(1), Operator::Plus, getExpr(3))),
            Operator::Plus,
            getExpr(5),
        );

        checkEq(expected, expr);
    }

    #[test]
    fn testAdditionDivision() {
        // a / b + c
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(0),
            getOperatorComponent(1, Operator::Div),
            getExprComponent(2),
            getOperatorComponent(3, Operator::Plus),
            getExprComponent(4),
        ]);

        let expected = OperatorExpr::binaryExpr(
            Box::new(OperatorExpr::binaryExpr(getExpr(0), Operator::Div, getExpr(2))),
            Operator::Plus,
            getExpr(4),
        );

        checkEq(expected, expr);
    }

    #[test]
    fn testAdditionDivisionPriority() {
        // a + b / c
        // a + (b / c)
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(0),
            getOperatorComponent(1, Operator::Plus),
            getExprComponent(2),
            getOperatorComponent(3, Operator::Div),
            getExprComponent(4),
        ]);

        let expected = OperatorExpr::binaryExpr(
            getExpr(0),
            Operator::Plus,
            Box::new(OperatorExpr::binaryExpr(getExpr(2), Operator::Div, getExpr(4))),
        );

        checkEq(expected, expr);
    }

    #[test]
    fn testIncrement() {
        // a++
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(1),
            getOperatorComponent(2, Operator::Increment),
        ]);

        let expected = OperatorExpr::unaryOperator(getExpr(1), Operator::Increment, getPosIndex(2).getRangeWithLength(1));

        checkEq(expected, expr);
    }

    #[test]
    fn testIncrementDivision() {
        // a++ / b
        // (a++) / b
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(0),
            getOperatorComponent(1, Operator::Increment),
            getOperatorComponent(2, Operator::Div),
            getExprComponent(3),
        ]);

        let expected = OperatorExpr::binaryExpr(
            Box::new(OperatorExpr::unaryOperator(getExpr(0), Operator::Increment, getPosIndex(1).getRangeWithLength(1))),
            Operator::Div,
            getExpr(3),
        );

        checkEq(expected, expr);
    }

    #[test]
    fn testPlusIncrementDivision() {
        // a + b++ / c
        // a + ((b++) / c)
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(0),
            getOperatorComponent(1, Operator::Plus),
            getExprComponent(2),
            getOperatorComponent(3, Operator::Increment),
            getOperatorComponent(4, Operator::Div),
            getExprComponent(5),
        ]);

        let expected = OperatorExpr::binaryExpr(
            getExpr(0),
            Operator::Plus,
            Box::new(OperatorExpr::binaryExpr(
                Box::new(OperatorExpr::unaryOperator(getExpr(2), Operator::Increment, getPosIndex(3).getRangeWithLength(1))),
                Operator::Div,
                getExpr(5),
            )),
        );

        checkEq(expected, expr);
    }

    #[test]
    fn testInvalidNoOperators() {
        // a
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(1),
        ]);

        assert!(expr.is_err());
    }

    #[test]
    fn testInvalidMultiNoOperators() {
        // a b
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(1),
            getExprComponent(2),
        ]);

        assert!(expr.is_err());
    }

    #[test]
    fn testInvalidNoOperands() {
        // +
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(1),
        ]);

        assert!(expr.is_err());
    }

    #[test]
    fn testInvalidMultiNoOperands() {
        // + +
        let expr = OperatorExpr::getFromInfix(vec![
            getOperatorComponent(1, Operator::Plus),
            getOperatorComponent(2, Operator::Plus),
        ]);

        assert!(expr.is_err());
    }

    #[test]
    fn testInvalidBinary() {
        // a +
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(1),
            getOperatorComponent(2, Operator::Plus),
        ]);

        assert!(expr.is_err());
    }

    #[test]
    fn testInvalidUnary() {
        let expr = OperatorExpr::getFromInfix(vec![
            getOperatorComponent(0, Operator::Increment),
        ]);

        assert!(expr.is_err(), "{:#?}", expr);
    }

    #[test]
    fn testPartialInvalidUnary() {
        let expected = OperatorExpr::unaryOperator(getExpr(1), Operator::Increment, getPosIndex(2).getRangeWithLength(1));

        // a ++ b
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(1),
            getOperatorComponent(2, Operator::Increment),
            getExprComponent(3),
        ]);

        checkEq(expected, expr);
    }
}
