use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr::null_mut;
use std::sync::Arc;

use hashbrown::HashMap;
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};
use llvm_sys::bit_writer::LLVMWriteBitcodeToFile;
use llvm_sys::core::{LLVMConstNull, LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext, LLVMDisposeBuilder, LLVMDisposeModule, LLVMInt8TypeInContext, LLVMModuleCreateWithNameInContext, LLVMSetTarget};
use llvm_sys::linker::LLVMLinkModules2;
use llvm_sys::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMValueRef};
use llvm_sys::target::{LLVM_InitializeAllAsmParsers, LLVM_InitializeAllAsmPrinters, LLVM_InitializeAllTargetInfos, LLVM_InitializeAllTargetMCs, LLVM_InitializeAllTargets, LLVMSetModuleDataLayout};
use llvm_sys::target_machine::{LLVMCodeGenOptLevel, LLVMCodeModel, LLVMCreateTargetDataLayout, LLVMCreateTargetMachine, LLVMGetDefaultTargetTriple, LLVMGetTargetFromTriple, LLVMRelocMode};
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
    entryName: String,
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
            if self.module != null_mut() {
                LLVMDisposeModule(self.module);
            }
        }
    }
}

impl CompiledModule {
    pub fn new(context: Context, resolved: ResolvedAST) -> Self {
        let mut module = Self::empty(context);
        module.entryName = format!("main_{}", resolved.getId());
        let statementVec = resolved.take().statementVec;
        let mainFunction = ResolvedFunctionDefinition {
            function: Function::new(module.entryName.to_owned(), Visibility::Public, VOID_TYPE.to_owned(), Vec::new()),
            parameterVecId: Vec::new(),
            scope: ResolvedScope {
                statementVec,
            },
        };
        unsafe {
            let null = LLVMConstNull(LLVMInt8TypeInContext(module.context.0.lock_arc().context));
            emit(&mut module, null, Statement::FunctionDefinition(mainFunction));
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

            return CompiledModule {
                entryName: String::new(),
                context,
                module,
                builder,
                blockStack: Vec::new(),
                variableMap: HashMap::new(),
            };
        }
    }

    pub fn merge(&mut self, mut other: CompiledModule) {
        if self.entryName.is_empty() {
            self.entryName = other.entryName.to_owned();
        }

        if unsafe { LLVMLinkModules2(self.module, other.module) != 0 } {
            panic!("failed to link modules");
        }
        other.module = null_mut();
    }

    pub fn writeBitcode(&self, path: impl AsRef<Path>) {
        let cstring = CString::new(path.as_ref().to_str().expect("invalid string")).expect("invalid string");
        unsafe {
            // safe to call multiple times
            LLVM_InitializeAllTargetInfos();
            LLVM_InitializeAllTargets();
            LLVM_InitializeAllTargetMCs();
            LLVM_InitializeAllAsmParsers();
            LLVM_InitializeAllAsmPrinters();

            let triple = LLVMGetDefaultTargetTriple();
            let mut target = null_mut();
            let mut str = null_mut();
            if LLVMGetTargetFromTriple(triple, &mut target as *mut _, &mut str as *mut _) != 0 {
                panic!("failed to get target from triple: {}", CStr::from_ptr(str).to_str().unwrap());
            }
            let cpu = CString::new("x86-64").unwrap();
            let features = CString::new("").unwrap();
            let targetMachine = LLVMCreateTargetMachine(target, triple, cpu.as_ptr(), features.as_ptr(), LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault, LLVMRelocMode::LLVMRelocDefault, LLVMCodeModel::LLVMCodeModelDefault);
            let dataLayout = LLVMCreateTargetDataLayout(targetMachine);

            LLVMSetModuleDataLayout(self.module, dataLayout);
            LLVMSetTarget(self.module, triple);
            LLVMWriteBitcodeToFile(self.module, cstring.as_ptr());
        }
    }

    pub fn writeExecutable(&self, path: impl AsRef<Path>) {
        let bitcodePath = path.as_ref().to_str().expect("invalid string").to_owned() + ".bc";
        self.writeBitcode(&bitcodePath);
        linkExecutable(&self.entryName, bitcodePath, path);
    }
}
