use std::ffi::CString;
use std::path::Path;
use std::ptr::null_mut;
use std::sync::Arc;

use hashbrown::HashMap;
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};
use llvm_sys::bit_writer::LLVMWriteBitcodeToFile;
use llvm_sys::core::{LLVMContextCreate, LLVMContextDispose, LLVMCreateBasicBlockInContext, LLVMCreateBuilderInContext, LLVMDisposeBuilder, LLVMDisposeModule, LLVMModuleCreateWithNameInContext, LLVMPositionBuilderAtEnd};
use llvm_sys::linker::LLVMLinkModules2;
use llvm_sys::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMValueRef};
use parking_lot::Mutex;
use crate::ast::visibility::Visibility;

use crate::backend::emit::emit;
use crate::backend::link::linkExecutable;
use crate::resolver::function::Function;
use crate::resolver::resolvedast::ResolvedAST;
use crate::resolver::resolvedast::resolvedfunctiondefinition::ResolvedFunctionDefinition;
use crate::resolver::resolvedast::resolvedscope::ResolvedScope;
use crate::resolver::resolvedast::statement::Statement;
use crate::resolver::typeinfo::void::VOID_TYPE;

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
    blockStack: Vec<LLVMBasicBlockRef>,
    variableMap: HashMap<usize, LLVMValueRef>,
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
        let mainFunction = ResolvedFunctionDefinition {
            function: Function::new("main".to_owned(), Visibility::Public, VOID_TYPE.to_owned(), Vec::new()),
            scope: ResolvedScope {
                statementVec: resolved.take().statementVec,
            },
        };
        unsafe {
            emit(&mut module, Statement::FunctionDefinition(mainFunction));
        }
        unsafe {
            LLVMVerifyModule(module.module, LLVMVerifierFailureAction::LLVMAbortProcessAction, null_mut());
        }
        return module;
    }

    pub fn empty(context: Context) -> Self {
        unsafe {
            let llvmContext = context.0.lock_arc();
            let builder = LLVMCreateBuilderInContext(llvmContext.context);
            let name = CString::new("CompiledModule").unwrap();
            let module = LLVMModuleCreateWithNameInContext(name.as_ptr(), llvmContext.context);

            let blockName = CString::new("main").unwrap();
            let basicBlock = LLVMCreateBasicBlockInContext(llvmContext.context, blockName.as_ptr());
            LLVMPositionBuilderAtEnd(builder, basicBlock);

            return CompiledModule {
                context,
                module,
                builder,
                blockStack: vec![basicBlock],
                variableMap: HashMap::new(),
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
