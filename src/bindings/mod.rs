use gluon;
use gluon::vm::api::{IO};
use std;


#[macro_use]
macro_rules! falcon_type_wrapper {
    ($p: path, $n: ident) => {
        #[derive(Clone, Debug)] pub(crate) struct $n { pub(crate) x: $p }
        impl VmType for $n { type Type = $n; }
        impl Traverseable for $n {}
        impl Userdata for $n {}
    }
}


mod il;
mod loader;
mod memory;
mod types;


pub fn hex(v: u64) -> String {
    format!("{:x}", v)
}


fn println(string: String) {
    println!("{}", string);
}


pub fn env (name: String) -> Option<String> {
    match std::env::var(&name) {
        Ok(v) => Some(v),
        Err(_) => None
    }
}


pub fn bindings (vm: gluon::RootedThread) -> gluon::RootedThread {
    vm.define_global("falcon_prim", record! {
        hex => primitive!(1 hex),
        println => primitive!(1 println),
        env => primitive!(1 env)
    }).unwrap();
    
    vm
}


pub fn attach_bindings(vm: gluon::RootedThread) -> gluon::RootedThread {
    // The order is important
    let vm = bindings(vm);
    let vm = il::bindings(vm);
    let vm = memory::bindings(vm);
    let vm = loader::bindings(vm);
    let vm = types::bindings(vm);
    vm
}


pub fn run_code(code: &str) -> gluon::RootedThread {
    let vm = gluon::new_vm();

    let vm = attach_bindings(vm);

    let mut compiler = gluon::Compiler::new();
    match compiler.run_io_expr::<IO<()>>(&vm, "code", code) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            panic!("Compile error");
        }
    };

    vm
}