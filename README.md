# CGG

## Commands

### fmt

Formats the code using `clang-format`

### <s>get</s>

To be implemented

### list

Lists the project dependencies

### run

compiles the program to a temporary file and executes it

### build

Compiles the program using GCC

### <s>remove</s>

To be implemented

### <s>update</s>

To be implemented

### <s>search</s>

To be implemented

### init

Starts a new project

### change-default

Changes the default flags used by `build` and `run`

## Compilation

To compile the project, run `cargo build --release`

## Build WiX installer

run `cargo wix` and find the installer at `target/wix/*.msi`

## Prerequisites

- Rust (for compiling the project itself, otherfwise not required)
- GCC (for compilation)
- Clang-format (for formatting)
