use std::sync::Arc;

use once_cell::sync::Lazy;

use crate::ast::visibility::Visibility;
use crate::resolver::exporttable::completeexporttable::CompleteExportTable;
use crate::resolver::function::{Function, Parameter};
use crate::resolver::typeinfo::primitive::boolean::BOOLEAN_TYPE;
use crate::resolver::typeinfo::primitive::character::CHARACTER_TYPE;
use crate::resolver::typeinfo::primitive::float::FLOAT_TYPE;
use crate::resolver::typeinfo::primitive::integer::INTEGER_TYPE;
use crate::resolver::typeinfo::string::STRING_TYPE;
use crate::resolver::typeinfo::void::VOID_TYPE;

pub static CORE_EXPORT_TABLE: Lazy<Arc<CompleteExportTable>> = Lazy::new(|| {
    let mut builder = CompleteExportTable::newBuilder();

    // primitive types
    builder.addExportedType(INTEGER_TYPE.to_owned()).expect("failed to build core table");
    builder.addExportedType(FLOAT_TYPE.to_owned()).expect("failed to build core table");
    builder.addExportedType(BOOLEAN_TYPE.to_owned()).expect("failed to build core table");
    builder.addExportedType(CHARACTER_TYPE.to_owned()).expect("failed to build core table");
    builder.addExportedType(STRING_TYPE.to_owned()).expect("failed to build core table");
    builder.addExportedType(VOID_TYPE.to_owned()).expect("failed to build core table");

    builder.addExportedFunction(Function::new("print".to_owned(), Visibility::Public, VOID_TYPE.to_owned(), vec![
        Parameter {
            ty: STRING_TYPE.to_owned(),
            name: "value".to_owned(),
        }
    ])).expect("failed to build core table");

    return Arc::new(builder);
});