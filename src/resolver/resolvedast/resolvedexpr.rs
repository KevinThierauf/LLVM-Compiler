use std::fmt::Debug;
use std::ops::Deref;

use once_cell::sync::Lazy;

use crate::resolver::resolvedast::defaultclass::DefaultClass;
use crate::resolver::resolvedast::defaultpointer::DefaultPointer;
use crate::resolver::resolvedast::defaultvalue::DefaultValue;
use crate::resolver::resolvedast::functioncall::FunctionCall;
use crate::resolver::resolvedast::readexpr::ReadExpr;
use crate::resolver::resolvedast::resolvedoperator::ResolvedOperator;
use crate::resolver::resolvedast::resolvedproperty::ResolvedProperty;
use crate::resolver::resolvedast::resolvedvariable::ResolvedVariable;
use crate::resolver::resolvedast::statement::StatementType;
use crate::resolver::resolvedast::variabledeclare::VariableDeclare;
use crate::resolver::typeinfo::primitive::boolean::BOOLEAN_TYPE;
use crate::resolver::typeinfo::primitive::character::CHARACTER_TYPE;
use crate::resolver::typeinfo::primitive::float::FLOAT_TYPE;
use crate::resolver::typeinfo::primitive::integer::INTEGER_TYPE;
use crate::resolver::typeinfo::string::STRING_TYPE;
use crate::resolver::typeinfo::Type;

pub trait ResolvedExprType: StatementType + Debug {
    fn getExpressionType(&self) -> Type;

    fn isAssignable(&self) -> bool {
        return false;
    }
}

#[derive(Debug)]
struct ResolvedExprTypeValue {
    ty: Type,
    assignable: bool,
}

impl ResolvedExprTypeValue {
    fn new(ty: Type, assignable: bool) -> Self {
        return Self {
            ty,
            assignable,
        };
    }
}

impl StatementType for ResolvedExprTypeValue {}

impl ResolvedExprType for ResolvedExprTypeValue {
    fn getExpressionType(&self) -> Type {
        return self.ty.to_owned();
    }

    fn isAssignable(&self) -> bool {
        return self.assignable;
    }
}

#[derive(Debug)]
pub enum ResolvedExpr {
    Operator(Box<ResolvedOperator>),
    FunctionCall(Box<FunctionCall>),
    Read(ReadExpr),
    // ConstructorCall(Box<ConstructorCall>),
    VariableDeclaration(VariableDeclare),
    Variable(ResolvedVariable),
    Property(Box<ResolvedProperty>),
    DefaultValue(DefaultValue),
    DefaultPointer(DefaultPointer),
    DefaultClass(DefaultClass),
    LiteralBool(bool),
    LiteralChar(u32),
    LiteralFloat(f64),
    LiteralInteger(i64),
    LiteralString(String),
}

impl ResolvedExpr {
    pub fn getResolvedExprType(&self) -> &dyn ResolvedExprType {
        return match self {
            ResolvedExpr::Operator(v) => v.deref(),
            ResolvedExpr::FunctionCall(v) => v.deref(),
            ResolvedExpr::VariableDeclaration(v) => v,
            ResolvedExpr::DefaultValue(v) => v,
            ResolvedExpr::DefaultClass(v) => v,
            ResolvedExpr::DefaultPointer(v) => v,
            ResolvedExpr::Variable(v) => v,
            ResolvedExpr::Property(v) => v.deref(),
            ResolvedExpr::Read(_) => {
                static LITERAL_RESOLVED_EXPR_TYPE: Lazy<ResolvedExprTypeValue> = Lazy::new(|| ResolvedExprTypeValue::new(INTEGER_TYPE.to_owned(), false));
                return LITERAL_RESOLVED_EXPR_TYPE.deref();
            }
            ResolvedExpr::LiteralBool(_) => {
                static LITERAL_RESOLVED_EXPR_TYPE: Lazy<ResolvedExprTypeValue> = Lazy::new(|| ResolvedExprTypeValue::new(BOOLEAN_TYPE.to_owned(), false));
                return LITERAL_RESOLVED_EXPR_TYPE.deref();
            }
            ResolvedExpr::LiteralChar(_) => {
                static LITERAL_RESOLVED_EXPR_TYPE: Lazy<ResolvedExprTypeValue> = Lazy::new(|| ResolvedExprTypeValue::new(CHARACTER_TYPE.to_owned(), false));
                return LITERAL_RESOLVED_EXPR_TYPE.deref();
            }
            ResolvedExpr::LiteralFloat(_) => {
                static LITERAL_RESOLVED_EXPR_TYPE: Lazy<ResolvedExprTypeValue> = Lazy::new(|| ResolvedExprTypeValue::new(FLOAT_TYPE.to_owned(), false));
                return LITERAL_RESOLVED_EXPR_TYPE.deref();
            }
            ResolvedExpr::LiteralInteger(_) => {
                static LITERAL_RESOLVED_EXPR_TYPE: Lazy<ResolvedExprTypeValue> = Lazy::new(|| ResolvedExprTypeValue::new(INTEGER_TYPE.to_owned(), false));
                return LITERAL_RESOLVED_EXPR_TYPE.deref();
            }
            ResolvedExpr::LiteralString(_) => {
                static LITERAL_RESOLVED_EXPR_TYPE: Lazy<ResolvedExprTypeValue> = Lazy::new(|| ResolvedExprTypeValue::new(STRING_TYPE.to_owned(), false));
                return LITERAL_RESOLVED_EXPR_TYPE.deref();
            }
        };
    }

    pub fn getExpressionType(&self) -> Type {
        return self.getResolvedExprType().getExpressionType();
    }
}
