use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use parking_lot::{Condvar, RawMutex};
use parking_lot::lock_api::Mutex;

use crate::module::visibility::Visibility::Public;
use crate::resolver::exporttable::completeexporttable::CompleteExportTable;
use crate::resolver::exporttable::completeexporttable::coreexporttable::CORE_EXPORT_TABLE;
use crate::resolver::exporttable::incompleteexporttable::{IncompleteExportTable, VisibilityExportHandler};

pub mod completeexporttable;
pub mod incompleteexporttable;

enum ExportTableState {
    Incomplete(IncompleteExportTable),
    Complete(Arc<CompleteExportTable>),
}

struct ExportImpl {
    writers: AtomicUsize,
    conditional: Condvar,
    table: Mutex<RawMutex, ExportTableState>,
}

impl ExportImpl {
    fn getReadTable(&self) -> Arc<CompleteExportTable> {
        let mut exportImpl = self.table.lock();
        loop {
            if let ExportTableState::Complete(table) = exportImpl.deref() {
                return table.to_owned();
            } else {
                self.conditional.wait(&mut exportImpl);
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

    fn setComplete(&self) {
        let mut exportImpl = self.table.lock();
        *exportImpl = match exportImpl.deref() {
            ExportTableState::Incomplete(incompleteTable) => ExportTableState::Complete(CompleteExportTable::new(incompleteTable, vec![CORE_EXPORT_TABLE.to_owned()])),
            ExportTableState::Complete(_) => panic!("export table has already been set to complete"),
        };
    }
}

pub struct ModuleExportTable {
    exportState: Arc<ExportImpl>,
}

impl Clone for ModuleExportTable {
    fn clone(&self) -> Self {
        self.addWriter();
        return Self {
            exportState: self.exportState.to_owned(),
        };
    }
}

impl Drop for ModuleExportTable {
    fn drop(&mut self) {
        self.removeWriter();
    }
}

impl ModuleExportTable {
    pub fn new() -> Self {
        return Self {
            exportState: Arc::new(ExportImpl {
                writers: AtomicUsize::new(1),
                conditional: Default::default(),
                table: Mutex::new(ExportTableState::Incomplete(IncompleteExportTable::new(VisibilityExportHandler(Public)))),
            }),
        };
    }

    fn addWriter(&self) {
        let _v = self.exportState.writers.fetch_add(1, Ordering::SeqCst);
        debug_assert_ne!(_v, 1, "writing has been closed");
    }

    fn removeWriter(&self) {
        let writers = self.exportState.writers.fetch_sub(1, Ordering::Relaxed);
        if writers == 0 {
            self.exportState.setComplete();
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

    pub fn getCompleteExportTableBlocking(&self) -> Arc<CompleteExportTable> {
        self.removeWriter();
        return ExportImpl::getReadTable(self.exportState.deref());
    }
}
