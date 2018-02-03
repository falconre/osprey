#![recursion_limit="256"]

extern crate gluon;
#[macro_use]
extern crate gluon_vm;
extern crate falcon;


#[macro_use]
macro_rules! falcon_type_wrapper {
    ($p: path, $n: ident) => {
        #[derive(Clone, Debug)] pub struct $n { pub x: $p }
        impl VmType for $n { type Type = $n; }
        impl Traverseable for $n {}
        impl Userdata for $n {}
    }
}


pub mod il;
pub mod loader;
pub mod memory;
pub mod types;


fn hex(v: u64) -> String {
    format!("{:x}", v)
}


fn println(string: String) {
    println!("{}", string);
}


fn env (name: String) -> Option<String> {
    match std::env::var(&name) {
        Ok(v) => Some(v),
        Err(_) => None
    }
}


pub fn bindings (vm: gluon::RootedThread) -> gluon::RootedThread {
    fn falcon_prim_loader(vm: &gluon::Thread)
        -> gluon::vm::Result<gluon::vm::ExternModule> {
        
        gluon::vm::ExternModule::new(vm, record! {
            hex => primitive!(1 hex),
            println => primitive!(1 println),
            env => primitive!(1 env)
        })
    }

    gluon::import::add_extern_module(&vm, "falcon_prim", falcon_prim_loader);
    
    vm
}


pub fn attach_bindings(vm: gluon::RootedThread) -> gluon::RootedThread {
    // The order is important
    let vm = bindings(vm);
    let vm = il::bindings(vm);
    let vm = memory::bindings(vm);
    let vm = types::bindings(vm);
    let vm = loader::bindings(vm);
    vm
}


pub fn run_code(code: &str) -> gluon::RootedThread {
    let vm = gluon::new_vm();

    let vm = attach_bindings(vm);

    let mut compiler = gluon::Compiler::new();
    match compiler.run_expr::<()>(&vm, "code", code) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            panic!("Compile error");
        }
    };

    vm
}