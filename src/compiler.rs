use std::cmp::min;
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use std::thread::{Builder, JoinHandle};

use anyhow::Error;
use log::error;
use parking_lot::Mutex;

use crate::ast::{AbstractSyntaxTree, ASTError};
use crate::backend::{CompiledModule, Context};
use crate::module::{Module, ParseError, SourceFile};
use crate::resolver::exporttable::GlobalExportTable;
use crate::resolver::resolutionerror::ResolutionError;
use crate::resolver::Resolver;

pub enum CompilerError {
    ReadSourceError(Error),
    TokenParseError(ParseError),
    ASTParseError(ASTError),
    ResolutionError(Vec<ResolutionError>),
}

impl CompilerError {
    pub fn getErrorMessage(&self) -> String {
        return match self {
            CompilerError::ReadSourceError(error) => format!("Failed to read source: {error}"),
            CompilerError::TokenParseError(error) => format!("Failed to parse tokens: {}", error.getDisplayMessage()),
            CompilerError::ASTParseError(error) => format!("Failed to match syntax: {}", error.getDisplayMessage()),
            CompilerError::ResolutionError(error) => format!("Failed to resolve symbols: {error:?}"),
        };
    }
}

pub struct Compiler {
    context: Context,
    exportTable: GlobalExportTable,
    threads: Vec<JoinHandle<Option<CompiledModule>>>,
}

impl Compiler {
    pub fn new(threadCount: Option<NonZeroUsize>, sourceVec: Vec<String>) -> Self {
        let exportTable = GlobalExportTable::new();
        let threadCount = threadCount.unwrap_or(std::thread::available_parallelism().unwrap_or(NonZeroUsize::new(4).unwrap()));
        let threadCount = min(threadCount.into(), sourceVec.len());
        let jobManager = Arc::new(Mutex::new(JobManager::Source(exportTable.to_owned(), sourceVec)));

        let mut handleVec = Vec::new();
        let context = Context::new();

        for _ in 0..threadCount {
            handleVec.push(CompileJob::new(context.to_owned(), jobManager.to_owned()));
        }

        return Self {
            context,
            exportTable,
            threads: handleVec,
        };
    }

    fn compileFirstStage(exportTable: GlobalExportTable, source: String) -> Result<Resolver, CompilerError> {
        // open source file
        let sourceFile = SourceFile::new(PathBuf::from(source)).map_err(|error| CompilerError::ReadSourceError(error))?;
        // break source file down into tokens
        let module = Module::new(sourceFile).map_err(|error| CompilerError::TokenParseError(error))?;
        // info!("Module: {module:?}");
        // convert tokens into syntax expressions
        let ast = AbstractSyntaxTree::new(module).map_err(|error| CompilerError::ASTParseError(error))?;
        // info!("AST: {ast:?}");
        // first step of resolution (identifying export symbols)
        // local exports will be resolved after all global symbols have been resolved
        // global exports will be resolved after all global symbols have been identified
        return match Resolver::new(ast, exportTable) {
            Ok(resolver) => Ok(resolver),
            Err(err) => Err(CompilerError::ResolutionError(err))
        };
    }

    fn compileSecondStage(context: Context, resolver: Resolver) -> Result<CompiledModule, CompilerError> {
        // second step of resolution (resolving all symbols using export tables (global and local))
        let resolved = resolver.getResolvedAST().map_err(|error| CompilerError::ResolutionError(error))?;
        // info!("Resolved: {resolved:?}");
        // convert resolved ast into binary
        // source should be completely valid at this point; all errors should have been resolved
        return Ok(CompiledModule::new(context, resolved));
    }

    pub fn getCompiledResult(self) -> Option<CompiledModule> {
        match self.exportTable.getExportErrorsBlocking() {
            Ok(_) => {
                // do nothing
            }
            Err(err) => {
                error!("Global resolution error");
                error!("{}", CompilerError::ResolutionError(err).getErrorMessage());
                return None;
            }
        }

        let mut compiledModule = CompiledModule::empty(self.context);
        for handle in self.threads {
            if let Ok(other) = handle.join() {
                compiledModule.merge(other?);
            } else {
                exit(1);
            }
        }
        return Some(compiledModule);
    }
}

enum JobManager {
    Source(GlobalExportTable, Vec<String>),
    Complete,
}

struct CompileJob {
    error: bool,
    resolverVec: Vec<Resolver>,
}

impl CompileJob {
    fn new(context: Context, jobManager: Arc<Mutex<JobManager>>) -> JoinHandle<Option<CompiledModule>> {
        return Builder::new().spawn(|| {
            return Self {
                error: false,
                resolverVec: Vec::new(),
            }.start(context, jobManager);
        }).expect("unable to create thread for job");
    }

    fn start(mut self, context: Context, jobManager: Arc<Mutex<JobManager>>) -> Option<CompiledModule> {
        loop {
            let mut lock = jobManager.lock();
            if let JobManager::Source(exportTable, source) = lock.deref_mut() {
                if let Some(source) = source.pop() {
                    let exportTable = exportTable.to_owned();
                    drop(lock);
                    self.addSourceFile(source, exportTable);
                    continue;
                } else {
                    *lock = JobManager::Complete;
                }
            }
            debug_assert!(matches!(lock.deref(), JobManager::Complete));
            return self.getCompiledResult(context);
        }
    }

    fn getValue<T>(&mut self, result: Result<T, CompilerError>, callback: impl FnOnce(&mut Self, T)) {
        return match result {
            Ok(value) => callback(self, value),
            Err(error) => {
                self.error = true;
                error!("{}", error.getErrorMessage());
            }
        };
    }

    fn addSourceFile(&mut self, source: String, exportTable: GlobalExportTable) {
        self.getValue(Compiler::compileFirstStage(exportTable, source), |s, value| s.resolverVec.push(value));
    }

    fn getCompiledResult(mut self, context: Context) -> Option<CompiledModule> {
        let mut resolverVec = Vec::new();
        resolverVec.append(&mut self.resolverVec);

        let mut compiledModule = CompiledModule::empty(context.to_owned());
        for resolver in resolverVec {
            self.getValue(Compiler::compileSecondStage(context.to_owned(), resolver), |_, value| {
                compiledModule.merge(value);
            });
        }

        return if self.error {
            None
        } else {
            Some(compiledModule)
        };
    }
}
