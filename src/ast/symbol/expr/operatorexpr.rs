use crate::ast::ASTError;
use crate::ast::symbol::expr::{Expr, ExprType};
use crate::ast::symbol::{Symbol, SymbolType};
use crate::module::modulepos::{ModulePos, ModuleRange};
use crate::module::Operator;

#[derive(Debug)]
pub struct OperatorExpr {
    pub range: ModuleRange,
    pub operands: Box<[Expr]>,
    pub operator: Operator,
}

impl OperatorExpr {
    pub fn getFromComponents(pos: ModulePos, components: Vec<OperationComponent>) -> Result<Self, ASTError> {
        // todo
        Ok(Self {
            range: match components.get(0).ok_or(ASTError::MatchFailed(pos))? {
                OperationComponent::Operator(r, _) => r.to_owned(),
                OperationComponent::Expression(v) => v.getRange().to_owned()
            },
            operands: Box::new([]),
            operator: Operator::Increment,
        })
    }

    pub fn binaryExpr(first: Expr, operator: Operator, second: Expr) -> Self {
        return Self {
            range: first.getRange().getCombined(second.getRange()),
            operands: vec![first, second].into_boxed_slice(),
            operator,
        };
    }

    pub fn unaryOperator(expr: Expr, operator: Operator, operatorPos: ModulePos) -> Self {
        return Self {
            range: expr.getRange().getCombined(&operatorPos.getRangeWithLength(1)),
            operands: vec![expr].into_boxed_slice(),
            operator,
        };
    }

    pub fn operator(operator: Operator, operatorPos: ModulePos) -> Self {
        return Self {
            range: operatorPos.getRangeWithLength(1),
            operands: Box::new([]),
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
        assert!(provided.operator.getOperands().contains(&provided.operands.len()), "expected {} operand(s), found {}\nExpected:\n{expected:#?}\n\nProvided:\n{provided:#?}", {
            let range = provided.operator.getOperands();
            if range.start() == range.end() {
                range.start().to_string()
            } else {
                format!("[{}, {}]", range.start(), range.end())
            }
        }, provided.operands.len());

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
            Err(err) => panic!("expected {expected:?}, found ASTError {}", err.getErrorMessage()),
        };

        checkEqualRef(&expected, &provided);
    }

    #[test]
    fn testAddition() {
        // a + b
        let expr = OperatorExpr::getFromComponents(getPosIndex(0), vec![
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
        let expr = OperatorExpr::getFromComponents(getPosIndex(0), vec![
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
        let expr = OperatorExpr::getFromComponents(getPosIndex(0), vec![
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
        let expr = OperatorExpr::getFromComponents(getPosIndex(0), vec![
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
        let expr = OperatorExpr::getFromComponents(getPosIndex(0), vec![
            getExprComponent(1),
            getOperatorComponent(2, Operator::Increment),
        ]);

        let expected = OperatorExpr::unaryOperator(getExpr(1), Operator::Increment, getPosIndex(2));

        checkEq(expected, expr);
    }

    #[test]
    fn testIncrementDivision() {
        // a++ / b
        // (a++) / b
        let expr = OperatorExpr::getFromComponents(getPosIndex(0), vec![
            getExprComponent(1),
            getOperatorComponent(2, Operator::Increment),
            getOperatorComponent(3, Operator::Div),
            getExprComponent(4),
        ]);

        let expected = OperatorExpr::binaryExpr(
            Box::new(OperatorExpr::unaryOperator(getExpr(1), Operator::Increment, getPosIndex(2))),
            Operator::Div,
            getExpr(4),
        );

        checkEq(expected, expr);
    }

    #[test]
    fn testPlusIncrementDivision() {
        // a + b++ / c
        // a + ((b++) / c)
        let expr = OperatorExpr::getFromComponents(getPosIndex(0), vec![
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
                Box::new(OperatorExpr::unaryOperator(getExpr(1), Operator::Increment, getPosIndex(2))),
                Operator::Div,
                getExpr(4),
            )),
        );

        checkEq(expected, expr);
    }

    #[test]
    fn testInvalidNoOperators() {
        // a
        let expr = OperatorExpr::getFromComponents(getPosIndex(0), vec![
            getExprComponent(1),
        ]);

        assert!(expr.is_err());
    }

    #[test]
    fn testInvalidMultiNoOperators() {
        // a b
        let expr = OperatorExpr::getFromComponents(getPosIndex(0), vec![
            getExprComponent(1),
            getExprComponent(2),
        ]);

        assert!(expr.is_err());
    }

    #[test]
    fn testInvalidNoOperands() {
        // +
        let expr = OperatorExpr::getFromComponents(getPosIndex(0), vec![
            getExprComponent(1),
        ]);

        assert!(expr.is_err());
    }

    #[test]
    fn testInvalidMultiNoOperands() {
        // + +
        let expr = OperatorExpr::getFromComponents(getPosIndex(0), vec![
            getOperatorComponent(1, Operator::Plus),
            getOperatorComponent(2, Operator::Plus),
        ]);

        assert!(expr.is_err());
    }

    #[test]
    fn testInvalidBinary() {
        // a +
        let expr = OperatorExpr::getFromComponents(getPosIndex(0), vec![
            getExprComponent(1),
            getOperatorComponent(2, Operator::Plus),
        ]);

        assert!(expr.is_err());
    }

    #[test]
    fn testInvalidUnary() {
        // a ++ b
        let expr = OperatorExpr::getFromComponents(getPosIndex(0), vec![
            getExprComponent(1),
            getOperatorComponent(2, Operator::Increment),
            getExprComponent(3),
        ]);

        assert!(expr.is_err());
    }
}
