# LLVM-Compiler

This repository contains a basic compiler written using rust and the LLVM backend for a toy language. Note that this compiler (and the language it compiles) is very simplistic, and many important features (such as dynamic memory allocation) are not supported. 

This project is intended as a learning exercise; feel free to use it however you'd like.

## Getting started
This project is built using the rust programming language and the LLVM library; both are needed to compile this project. Your LLVM installation must include llc -- you may need to build LLVM from source to obtain it.
1. Set the environment variable "LLVM_SYS_160_PREFIX" to the path of your LLVM installation.
2. Add the bin directory of your LLVM installation to your PATH environment variable.
3. `git clone https://github.com/KevinThierauf/LLVM-Compiler.git`
4. Navigate to the sdk directory (`cd LLVM-Compiler/lib/sdk`)
5. Build the SDK project (`cargo build`)
6. Go back to the LLVM-Compiler directory (`cd ../..`)
7. Compile and run the project (`cargo run`)
8. Done! There should be an executable named "output.exe" in the `LLVM-Compiler/output` directory.

The `examples/source.txt` is the source file used for compilation -- change that file and run the compiler again to generate a new executable.

## Source
The compiler breaks processes the source in four main stages:
1. Token-ization (`src/module`)
   - Break input source file into basic tokens (keywords, operators, parenthesis, etc.)
2. Abstract Syntax Tree (`src/ast`)
   - Group tokens together to create syntax structures (if/else, variable definitions, etc.)
3. Resolver (`src/resolver`)
   - Collect defined structures (types, functions, etc.) from AST
   - Map AST expressions to their definitions (e.g. function calls to function definition)
   - Validate provided AST (make sure typing is correct, referenced structures exist, etc.)
4. Backend (`src/backend`)
   - Pass resolved syntax to LLVM for generation
   - Invoke linker to build executable from generated bitcode 

## Notes
 - Windows is currently the only fully working operating system. Linux should build but fails during linking. Other operating systems will not compile.
