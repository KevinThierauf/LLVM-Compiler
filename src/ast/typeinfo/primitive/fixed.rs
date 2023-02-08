use crate::ast::typeinfo::primitive::integer::Integer;

pub struct Fixed {
    base: Integer,
    bitShift: i16,
}
