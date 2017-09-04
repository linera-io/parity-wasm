// In this example we execute a contract funciton exported as "_call"

extern crate parity_wasm;

use std::env::args;

use parity_wasm::{interpreter, ModuleInstanceInterface};

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() != 3 {
        println!("Usage: {} <wasm file> <arg>", args[0]);
        println!("    wasm file should contain exported `_call` function with single I32 argument");
        return;
    }

    // Intrepreter initialization.
    // parity_wasm::DefaultProgramInstance parametrize ProgramInstance with a pre-defined "DummyUserError"
    // Initializes a default "env" module also.
    let program = parity_wasm::DefaultProgramInstance::with_env_params(
        interpreter::EnvParams {
            total_stack: 128*1024,
            total_memory: 2*1024*1024,
            allow_memory_growth: false,
        }
    ).expect("Failed to load program");

    // Here we load module using dedicated for this purpose
    // `deserialize_file` function (which works only with modules)
    let module = parity_wasm::deserialize_file(&args[1]).expect("Failed to load module");

    // Intialize deserialized module. It adds module into It expects 3 parameters:
    // - a name for the module
    // - a module declaration
    // - "main" module doesn't import native module(s) this is why we don't need to provide external native modules here
    // This test shows how to implement native module https://github.com/NikVolf/parity-wasm/blob/master/src/interpreter/tests/basics.rs#L197
    let module = program.add_module("main", module, None).expect("Failed to initialize module");

    // The argument should be parsable as a valid integer
    let argument: i32 = args[2].parse().expect("Integer argument required");

    // "_call" export of function to be executed with an i32 argument and prints the result of execution
    println!("Result: {:?}", module.execute_export("_call", vec![parity_wasm::RuntimeValue::I32(argument)].into()));
}
