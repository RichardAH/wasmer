// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/comments.wast
#![allow(
    warnings,
    dead_code
)]
use std::panic;
use wabt::wat2wasm;

use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, Instance, Export};
use super::_common::{
    spectest_importobject,
    NaNCheck,
};


// Line 10
fn create_module_1() -> ResultObject {
    let module_str = "(module)
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_1(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 52

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    // We group the calls together
    start_module_1(&result_object);
}
fn create_module_2() -> ResultObject {
    let module_str = "(module)
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_2(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 62

#[test]
fn test_module_2() {
    let result_object = create_module_2();
    // We group the calls together
    start_module_2(&result_object);
}
fn create_module_3() -> ResultObject {
    let module_str = "(module)
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_3(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 71

#[test]
fn test_module_3() {
    let result_object = create_module_3();
    // We group the calls together
    start_module_3(&result_object);
}
fn create_module_4() -> ResultObject {
    let module_str = "(module)
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_4(result_object: &ResultObject) {
    result_object.instance.start();
}

#[test]
fn test_module_4() {
    let result_object = create_module_4();
    // We group the calls together
    start_module_4(&result_object);
}