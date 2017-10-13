use gluon;
use gluon::vm::api::{IO};


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


fn hex(u: usize) -> String {
    format!("{:x}", u)
}


fn println(string: String) {
    println!("{}", string);
}


pub fn run_code(code: &str) -> gluon::RootedThread {
    let vm = gluon::new_vm();

    let vm = il::bindings(vm);
    let vm = loader::bindings(vm);

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