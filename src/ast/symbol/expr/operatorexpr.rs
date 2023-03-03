use std::any::Any;
use std::cmp::Ordering;

use crate::ast::ASTError;
use crate::ast::symbol::{Symbol, SymbolType};
use crate::ast::symbol::expr::{Expr, ExprType};
use crate::module::modulepos::{ModulePos, ModuleRange};
use crate::module::Operator;

#[derive(Debug)]
pub struct OperatorExpr {
    pub range: ModuleRange,
    pub operands: Box<[Expr]>,
    pub operator: Operator,
}

impl OperatorExpr {
    fn getFromPostfix(mut operands: Vec<Expr>, operators: Vec<(ModuleRange, Operator)>) -> Result<Self, ASTError> {
        debug_assert_ne!(0, operands.len());
        debug_assert_ne!(0, operators.len());
        operands.reverse();

        for (moduleRange, operator) in operators {
            let operandAmount = operator.getOperands();
            match operandAmount {
                1 => {
                    let operand = operands.pop().expect("expected operand");
                    operands.push(Box::new(Self::unaryOperator(operand, operator, moduleRange)));
                }
                2 => {
                    let first = operands.pop().expect("expected operand");
                    let second = operands.pop().expect("expected operand");
                    operands.push(Box::new(Self::binaryExpr(first, operator, second)));
                }
                _ => panic!("{operandAmount} operands unsupported"),
            }
        }

        debug_assert_eq!(1, operands.len());
        return Ok(*operands.remove(0).downcast().map_err(|expr| ASTError::MatchFailed(expr.getRange().getEndPos().to_owned()))?);
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
                },
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

        let mut operandStack: Vec<Expr> = Vec::new();
        let mut tmpOperatorStack: Vec<(ModuleRange, Operator)> = Vec::new();
        let mut resultOperatorStack: Vec<(ModuleRange, Operator)> = Vec::new();

        for component in components {
            match component {
                OperationComponent::Operator(range, operator) => {
                    loop {
                        if let Some((lastRange, lastOperator)) = tmpOperatorStack.pop() {
                            match operator.getPrecedence().cmp(&lastOperator.getPrecedence()) {
                                Ordering::Greater => {
                                    tmpOperatorStack.push((lastRange, lastOperator));
                                    tmpOperatorStack.push((range, operator));
                                    break;
                                }
                                Ordering::Equal => {
                                    tmpOperatorStack.push((range, operator));
                                    resultOperatorStack.push((lastRange, lastOperator));
                                    break;
                                }
                                Ordering::Less => {
                                    resultOperatorStack.push((lastRange, lastOperator));
                                }
                            }
                        } else {
                            tmpOperatorStack.push((range, operator));
                            break;
                        }
                    }
                }
                OperationComponent::Expression(expression) => {
                    operandStack.push(expression);
                }
            }
        }

        tmpOperatorStack.reverse();
        resultOperatorStack.append(&mut tmpOperatorStack);
        return Self::getFromPostfix(operandStack, resultOperatorStack);
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

#[derive(Debug)]
pub enum OperationComponent {
    Operator(ModuleRange, Operator),
    Expression(Expr),
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
        static POS: ModulePos = MODULE.with(|module| module.getModulePos(0));
        static POS2: ModulePos = MODULE.with(|module| module.getModulePos(1));
        static POS3: ModulePos = MODULE.with(|module| module.getModulePos(2));
        static POS4: ModulePos = MODULE.with(|module| module.getModulePos(3));
        static POS5: ModulePos = MODULE.with(|module| module.getModulePos(4));
        static POS6: ModulePos = MODULE.with(|module| module.getModulePos(5));
        static POS7: ModulePos = MODULE.with(|module| module.getModulePos(6));
        static POS8: ModulePos = MODULE.with(|module| module.getModulePos(7));
        static POS9: ModulePos = MODULE.with(|module| module.getModulePos(8));
        static POS10: ModulePos = MODULE.with(|module| module.getModulePos(9));
        static POS11: ModulePos = MODULE.with(|module| module.getModulePos(10));
        static POS12: ModulePos = MODULE.with(|module| module.getModulePos(11));
    }

    fn getPosIndex(index: usize) -> ModulePos {
        return match index {
            0 => POS.with(|v| v.to_owned()),
            1 => POS2.with(|v| v.to_owned()),
            2 => POS3.with(|v| v.to_owned()),
            3 => POS4.with(|v| v.to_owned()),
            4 => POS5.with(|v| v.to_owned()),
            5 => POS5.with(|v| v.to_owned()),
            6 => POS6.with(|v| v.to_owned()),
            7 => POS7.with(|v| v.to_owned()),
            8 => POS8.with(|v| v.to_owned()),
            9 => POS9.with(|v| v.to_owned()),
            10 => POS10.with(|v| v.to_owned()),
            11 => POS11.with(|v| v.to_owned()),
            12 => POS12.with(|v| v.to_owned()),
            _ => unreachable!("invalid index"),
        };
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

    fn checkEqualRef(expected: &OperatorExpr, provided: &OperatorExpr) {
        assert_eq!(provided.operator.getOperands(), provided.operands.len(), "expected {} operand(s), found {}\nExpected:\n{expected:#?}\n\nProvided:\n{provided:#?}", provided.operator.getOperands(), provided.operands.len());

        assert_eq!(expected.operator, provided.operator, "operator mismatch, expected {:?}, found {:?}\nExpected:\n{expected:#?}\n\nProvided:\n{provided:#?}", expected.operator, provided.operator);
        assert_eq!(expected.operands.len(), provided.operands.len(), "operand mismatch, expected {:?} operands, found {:?}\nExpected:\n{expected:#?}\n\nProvided:\n{provided:#?}", expected.operands.len(), provided.operands.len());

        for index in 0..expected.operands.len() {
            assert_eq!(expected.operands[index].getRange(), provided.operands[index].getRange(), "operand mismatch, expected {:?}, found {:?}\nExpected:\n{expected:#?}\n\nProvided:\n{provided:#?}", expected.operands[index], provided.operands[index]);

            let expectedOperand: Option<&OperatorExpr> = (&expected.operands[index] as &dyn Any).downcast_ref();
            let providedOperand: Option<&OperatorExpr> = (&provided.operands[index] as &dyn Any).downcast_ref();

            if let Some(expectedOperand) = expectedOperand {
                let providedOperand = providedOperand.expect(&format!("operand mismatch, expected {expectedOperand:?}, found {provided:?}\nExpected:\n{expected:#?}\n\nProvided:\n{provided:#?}"));
                checkEqualRef(expectedOperand, providedOperand);
            }
        }
    }

    fn checkEq(expected: OperatorExpr, provided: Result<OperatorExpr, ASTError>) {
        let provided = match provided {
            Ok(expr) => expr,
            Err(err) => panic!("expected {expected:#?}, found ASTError {}", err.getErrorMessage()),
        };

        checkEqualRef(&expected, &provided);
    }

    #[test]
    fn testAddition() {
        // a + b
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(1),
            getOperatorComponent(2, Operator::Plus),
            getExprComponent(3),
        ]);

        let expected = OperatorExpr::binaryExpr(getExpr(0), Operator::Plus, getExpr(3));

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
            getExprComponent(1),
            getOperatorComponent(2, Operator::Div),
            getExprComponent(3),
            getOperatorComponent(4, Operator::Plus),
            getExprComponent(5),
        ]);

        let expected = OperatorExpr::binaryExpr(
            Box::new(OperatorExpr::binaryExpr(getExpr(1), Operator::Div, getExpr(3))),
            Operator::Plus,
            getExpr(5),
        );

        checkEq(expected, expr);
    }

    #[test]
    fn testAdditionDivisionPriority() {
        // a + b / c
        // a + (b / c)
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(1),
            getOperatorComponent(2, Operator::Plus),
            getExprComponent(3),
            getOperatorComponent(4, Operator::Div),
            getExprComponent(5),
        ]);

        let expected = OperatorExpr::binaryExpr(
            getExpr(1),
            Operator::Plus,
            Box::new(OperatorExpr::binaryExpr(getExpr(3), Operator::Div, getExpr(5))),
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
            getExprComponent(1),
            getOperatorComponent(2, Operator::Increment),
            getOperatorComponent(3, Operator::Div),
            getExprComponent(4),
        ]);

        let expected = OperatorExpr::binaryExpr(
            Box::new(OperatorExpr::unaryOperator(getExpr(1), Operator::Increment, getPosIndex(2).getRangeWithLength(1))),
            Operator::Div,
            getExpr(4),
        );

        checkEq(expected, expr);
    }

    #[test]
    fn testPlusIncrementDivision() {
        // a + b++ / c
        // a + ((b++) / c)
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(1),
            getOperatorComponent(2, Operator::Plus),
            getExprComponent(3),
            getOperatorComponent(4, Operator::Increment),
            getOperatorComponent(5, Operator::Div),
            getExprComponent(6),
        ]);

        let expected = OperatorExpr::binaryExpr(
            getExpr(1),
            Operator::Plus,
            Box::new(OperatorExpr::binaryExpr(
                Box::new(OperatorExpr::unaryOperator(getExpr(1), Operator::Increment, getPosIndex(2).getRangeWithLength(1))),
                Operator::Div,
                getExpr(4),
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
        // a ++ b
        let expr = OperatorExpr::getFromInfix(vec![
            getExprComponent(1),
            getOperatorComponent(2, Operator::Increment),
            getExprComponent(3),
        ]);

        assert!(expr.is_err());
    }
}
