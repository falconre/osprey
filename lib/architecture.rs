use falcon;
use gluon::vm::api::{Userdata, VmType};
use gluon::vm::thread::{Traverseable};
use gluon;
use std::sync::Arc;

falcon_type_wrapper!(Arc<Box<falcon::architecture::Architecture>>, ArchitectureArchitecture);
falcon_type_wrapper!(falcon::architecture::Endian, ArchitectureEndian);

pub fn bindings (vm: gluon::RootedThread) -> gluon::RootedThread {

    vm.register_type::<ArchitectureArchitecture>("ArchitectureArchitecture", &[]).unwrap();
    vm.register_type::<ArchitectureEndian>("ArchitectureEndian", &[]).unwrap();
    
    vm
}