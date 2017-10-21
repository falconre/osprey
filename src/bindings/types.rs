use falcon;
use gluon::vm::api::{Userdata, VmType};
use gluon::vm::thread::{Traverseable};
use gluon;


falcon_type_wrapper!(falcon::types::Architecture, TypesArchitecture);
falcon_type_wrapper!(falcon::types::Endian, TypesEndian);



pub fn bindings (vm: gluon::RootedThread) -> gluon::RootedThread {

    vm.register_type::<TypesEndian>("TypesEndian", &[]).unwrap();
    vm.register_type::<TypesArchitecture>("TypesArchitecture", &[]).unwrap();
    vm.define_global("falcon_types_prim", record! {
    }).unwrap();
    
    vm
}