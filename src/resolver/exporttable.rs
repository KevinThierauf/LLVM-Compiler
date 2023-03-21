use std::mem::swap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use parking_lot::{Condvar, RawMutex};
use parking_lot::lock_api::Mutex;

use crate::resolver::exporttable::completeexporttable::CompleteExportTable;
use crate::resolver::exporttable::completeexporttable::coreexporttable::CORE_EXPORT_TABLE;
use crate::resolver::exporttable::incompleteexporttable::IncompleteExportTable;
use crate::resolver::resolutionerror::ResolutionError;

pub mod completeexporttable;
pub mod incompleteexporttable;

#[derive(Debug)]
enum ExportTableState {
    Incomplete(IncompleteExportTable),
    NotifyComplete(IncompleteExportTable),
    CompleteFailed,
    Complete(Arc<CompleteExportTable>),
}

struct ExportImpl {
    writers: AtomicUsize,
    conditional: Condvar,
    table: Mutex<RawMutex, ExportTableState>,
}

impl ExportImpl {
    fn getReadTable(&self) -> Option<Arc<CompleteExportTable>> {
        let mut exportImpl = self.table.lock();
        loop {
            match exportImpl.deref() {
                ExportTableState::Complete(table) => return Some(table.to_owned()),
                ExportTableState::CompleteFailed => return None,
                ExportTableState::Incomplete(_) | ExportTableState::NotifyComplete(_) => self.conditional.wait(&mut exportImpl),
            }
        }
    }

    fn getWriteTable<R>(&self, callback: impl FnOnce(&mut IncompleteExportTable) -> R) -> R {
        if let ExportTableState::Incomplete(table) = self.table.lock().deref_mut() {
            return callback(table);
        } else {
            panic!("cannot get export table for writing (table is no longer in writable state)");
        }
    }

    fn getExportErrors(&self) -> Result<(), Vec<ResolutionError>> {
        let mut exportImpl = self.table.lock();
        loop {
            let mut exportState = ExportTableState::CompleteFailed; // tmp
            swap(&mut exportState, exportImpl.deref_mut());
            match exportState {
                ExportTableState::NotifyComplete(incompleteTable) => {
                    let result = match CompleteExportTable::new(incompleteTable, vec![CORE_EXPORT_TABLE.to_owned()]) {
                        Ok(complete) => {
                            *exportImpl = ExportTableState::Complete(complete);
                            Ok(())
                        }
                        Err(errorVec) => {
                            *exportImpl = ExportTableState::CompleteFailed;
                            Err(errorVec)
                        }
                    };
                    self.conditional.notify_all();
                    return result;
                }
                ExportTableState::Incomplete(_) => {
                    swap(&mut exportState, exportImpl.deref_mut());
                    self.conditional.wait(&mut exportImpl);
                }
                ExportTableState::Complete(_) | ExportTableState::CompleteFailed => panic!("export table has already been set to complete"),
            }
        }
    }
}

pub struct GlobalExportTable {
    exportState: Arc<ExportImpl>,
}

impl Clone for GlobalExportTable {
    fn clone(&self) -> Self {
        self.addWriter();
        return Self {
            exportState: self.exportState.to_owned(),
        };
    }
}

impl Drop for GlobalExportTable {
    fn drop(&mut self) {
        self.removeWriter();
    }
}

impl GlobalExportTable {
    pub fn new() -> Self {
        return Self {
            exportState: Arc::new(ExportImpl {
                writers: AtomicUsize::new(1),
                conditional: Default::default(),
                table: Mutex::new(ExportTableState::Incomplete(IncompleteExportTable::new())),
            }),
        };
    }

    fn addWriter(&self) {
        let _v = self.exportState.writers.fetch_add(1, Ordering::SeqCst);
        debug_assert_ne!(_v, 0, "writing has been closed");
    }

    fn removeWriter(&self) {
        let writers = self.exportState.writers.fetch_sub(1, Ordering::Relaxed);
        if writers == 1 {
            let mut lock = self.exportState.table.lock();
            let mut exportTableState = ExportTableState::CompleteFailed; // tmp
            swap(&mut exportTableState, lock.deref_mut());
            if let ExportTableState::Incomplete(incomplete) = exportTableState {
                *lock = ExportTableState::NotifyComplete(incomplete);
            } else {
                panic!("expected state to be incomplete, found {exportTableState:?}");
            }
            self.exportState.conditional.notify_all();
        }
    }

    pub fn getIncompleteExportTable<R>(&self, function: impl FnOnce(&mut IncompleteExportTable) -> R) -> R {
        return self.exportState.getWriteTable(function);
    }

    pub fn merge(&self, other: IncompleteExportTable) {
        self.getIncompleteExportTable(|table| {
            table.merge(other);
        });
    }

    pub fn getCompleteExportTableBlocking(&self) -> Option<Arc<CompleteExportTable>> {
        self.removeWriter();
        return ExportImpl::getReadTable(self.exportState.deref());
    }

    pub fn getExportErrorsBlocking(&self) -> Result<(), Vec<ResolutionError>> {
        self.removeWriter();
        return self.exportState.getExportErrors();
    }
}
