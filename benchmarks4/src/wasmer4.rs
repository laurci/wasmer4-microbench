extern crate test;
use test::Bencher;

use anyhow::Result;

use wasmer_4::{imports, Function, Instance, Module, Store, Value};
use wasmer_compiler_cranelift_4::Cranelift;
use wasmer_compiler_singlepass_4::Singlepass;

const WASM_LIB: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/wasm.wasm");
const WASM_MEDIUM: &[u8] = include_bytes!("../../wasm/src/printf.wasm");
const WASM_LARGE: &[u8] = include_bytes!("../../wasm/src/lua.wasm");

fn compile_with_single_pass(code: &[u8]) -> Result<(Store, Module)> {
    let compiler = Singlepass::default();
    let store = Store::new(compiler);

    let module = Module::new(&store, code)?;

    Ok((store, module))
}

fn compile_with_cranelift(code: &[u8]) -> Result<(Store, Module)> {
    let compiler = Cranelift::default();
    let store = Store::new(compiler);

    let module = Module::new(&store, code)?;

    Ok((store, module))
}

fn instance_from_module(mut store: Store, module: Module) -> Result<(Store, Instance)> {
    let double = Function::new_typed(&mut store, |a: i32| -> i32 { a * 2 });

    let instance = Instance::new(
        &mut store,
        &module,
        &imports! {
            "env" => {
                "double" => double,
            }
        },
    )?;

    Ok((store, instance))
}

#[bench]
fn compile_small_single_pass(b: &mut Bencher) {
    b.iter(|| {
        let _ = compile_with_single_pass(&WASM_LIB).unwrap();
    });
}

#[bench]
fn compile_small_cranelift(b: &mut Bencher) {
    b.iter(|| {
        let _ = compile_with_cranelift(&WASM_LIB).unwrap();
    });
}

#[bench]
fn compile_medium_single_pass(b: &mut Bencher) {
    b.iter(|| {
        let _ = compile_with_single_pass(&WASM_MEDIUM).unwrap();
    });
}

#[bench]
fn compile_medium_cranelift(b: &mut Bencher) {
    b.iter(|| {
        let _ = compile_with_cranelift(&WASM_MEDIUM).unwrap();
    });
}

#[bench]
fn compile_large_single_pass(b: &mut Bencher) {
    b.iter(|| {
        let _ = compile_with_single_pass(&WASM_LARGE).unwrap();
    });
}

#[bench]
fn compile_large_cranelift(b: &mut Bencher) {
    b.iter(|| {
        let _ = compile_with_cranelift(&WASM_LARGE).unwrap();
    });
}

#[bench]
fn exec_fib_single_pass(b: &mut Bencher) {
    let (store, module) = compile_with_single_pass(&WASM_LIB).unwrap();
    let (mut store, instance) = instance_from_module(store, module).unwrap();
    let fib = instance.exports.get_function("fib").unwrap();

    b.iter(|| {
        let _ = fib.call(&mut store, &[Value::I64(25)]).unwrap();
    });
}

#[bench]
fn exec_fib_cranelift(b: &mut Bencher) {
    let (store, module) = compile_with_cranelift(&WASM_LIB).unwrap();
    let (mut store, instance) = instance_from_module(store, module).unwrap();
    let fib = instance.exports.get_function("fib").unwrap();

    b.iter(|| {
        let _ = fib.call(&mut store, &[Value::I64(25)]).unwrap();
    });
}

#[bench]
fn exec_sha_single_pass(b: &mut Bencher) {
    let (store, module) = compile_with_single_pass(&WASM_LIB).unwrap();
    let (mut store, instance) = instance_from_module(store, module).unwrap();
    let sha1 = instance.exports.get_function("sha1").unwrap();

    b.iter(|| {
        let _ = sha1.call(&mut store, &[Value::I32(1000)]).unwrap();
    });
}

#[bench]
fn exec_sha_cranelift(b: &mut Bencher) {
    let (store, module) = compile_with_cranelift(&WASM_LIB).unwrap();
    let (mut store, instance) = instance_from_module(store, module).unwrap();
    let sha1 = instance.exports.get_function("sha1").unwrap();

    b.iter(|| {
        let _ = sha1.call(&mut store, &[Value::I32(1000)]).unwrap();
    });
}

#[bench]
fn exec_double_single_pass(b: &mut Bencher) {
    let (store, module) = compile_with_single_pass(&WASM_LIB).unwrap();
    let (mut store, instance) = instance_from_module(store, module).unwrap();
    let double = instance.exports.get_function("bench_double").unwrap();

    b.iter(|| {
        let _ = double.call(&mut store, &[Value::I32(50_000)]).unwrap();
    });
}

#[bench]
fn exec_double_cranelift(b: &mut Bencher) {
    let (store, module) = compile_with_cranelift(&WASM_LIB).unwrap();
    let (mut store, instance) = instance_from_module(store, module).unwrap();
    let double = instance.exports.get_function("bench_double").unwrap();

    b.iter(|| {
        let _ = double.call(&mut store, &[Value::I32(50_000)]).unwrap();
    });
}
