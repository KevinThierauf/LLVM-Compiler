use std::sync::atomic::{AtomicUsize, Ordering};

use crate::resolver::resolvedast::resolvedscope::ResolvedScope;

pub mod functioncall;
pub mod ifstatement;
pub mod resolvedexpr;
pub mod statement;
pub mod whilestatement;
pub mod variabledeclare;
pub mod returnstatement;
pub mod resolvedoperator;
pub mod resolvedvariable;
pub mod resolvedproperty;
pub mod resolvedfunctiondefinition;
pub mod resolvedscope;
pub mod defaultvalue;
pub mod defaultclass;
pub mod printstatement;
pub mod defaultpointer;
pub mod readexpr;

static NEXT_VARIABLE_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct ResolvedAST {
    resolved: ResolvedScope,
    id: usize,
}

impl ResolvedAST {
    pub fn new(resolved: ResolvedScope) -> Self {
        return Self {
            resolved,
            id: NEXT_VARIABLE_ID.fetch_add(1, Ordering::Relaxed),
        };
    }

    pub fn getId(&self) -> usize {
        return self.id;
    }

    pub fn take(self) -> ResolvedScope {
        return self.resolved;
    }
}
