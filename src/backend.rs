use std::ffi::CString;
use std::path::Path;
use std::sync::Arc;

use llvm_sys::bit_writer::LLVMWriteBitcodeToFile;
use llvm_sys::core::{LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext, LLVMDisposeBuilder, LLVMDisposeModule, LLVMModuleCreateWithNameInContext};
use llvm_sys::linker::LLVMLinkModules2;
use llvm_sys::prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef};
use parking_lot::Mutex;

use crate::backend::emit::emit;
use crate::backend::link::linkExecutable;
use crate::resolver::resolvedast::ResolvedAST;

pub mod emit;
pub mod link;

struct SharedContext {
    context: LLVMContextRef,
}

impl SharedContext {
    fn new() -> Self {
        unsafe {
            return Self {
                context: LLVMContextCreate(),
            };
        }
    }
}

impl Drop for SharedContext {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.context);
        }
    }
}

#[derive(Clone)]
pub struct Context(Arc<Mutex<SharedContext>>);

unsafe impl Send for Context {}

unsafe impl Sync for Context {}

impl Context {
    pub fn new() -> Self {
        return Self {
            0: Arc::new(Mutex::new(SharedContext::new())),
        };
    }
}

pub struct CompiledModule {
    context: Context,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
}

unsafe impl Send for CompiledModule {}

impl Drop for CompiledModule {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
            LLVMDisposeModule(self.module);
        }
    }
}

impl CompiledModule {
    pub fn new(context: Context, resolved: ResolvedAST) -> Self {
        let mut module = Self::empty(context);
        for statement in resolved.take().statementVec {
            unsafe {
                emit(&mut module, statement);
            }
        }
        return module;
    }

    pub fn empty(context: Context) -> Self {
        unsafe {
            let llvmContext = context.0.lock_arc();
            let name = CString::new("CompiledModule").unwrap();
            return CompiledModule {
                context,
                module: LLVMModuleCreateWithNameInContext(name.as_ptr(), llvmContext.context),
                builder: LLVMCreateBuilderInContext(llvmContext.context),
            };
        }
    }

    pub fn merge(&mut self, other: CompiledModule) {
        if unsafe { LLVMLinkModules2(self.module, other.module) != 0 } {
            panic!("failed to link modules");
        }
    }

    pub fn writeBitcode(&self, path: impl AsRef<Path>) {
        let cstring = CString::new(path.as_ref().to_str().expect("invalid string")).expect("invalid string");
        unsafe {
            LLVMWriteBitcodeToFile(self.module, cstring.as_ptr());
        }
    }

    pub fn writeExecutable(&self, path: impl AsRef<Path>) {
        let bitcodePath = path.as_ref().to_str().expect("invalid string").to_owned() + ".bc";
        self.writeBitcode(&bitcodePath);
        linkExecutable(bitcodePath, path);
    }
}
