use falcon;
use gluon::vm::api::{Userdata, VmType};
use gluon::vm::thread::{Traverseable};
use gluon;
use std::sync::Arc;

falcon_type_wrapper!(Arc<Box<falcon::architecture::Architecture>>, ArchitectureArchitecture);

fn architecture_endian(architecture: &ArchitectureArchitecture) -> ArchitectureEndian {
    ArchitectureEndian { x: architecture.x.endian() }
}

falcon_type_wrapper!(falcon::architecture::Endian, ArchitectureEndian);

pub fn bindings (vm: gluon::RootedThread) -> gluon::RootedThread {

    vm.register_type::<ArchitectureArchitecture>("ArchitectureArchitecture", &[]).unwrap();
    vm.register_type::<ArchitectureEndian>("ArchitectureEndian", &[]).unwrap();
        
    fn falcon_architecture_prim_loader(vm: &gluon::Thread)
        -> gluon::vm::Result<gluon::vm::ExternModule> {

        gluon::vm::ExternModule::new(vm, record! {
            architecture_endian => primitive!(1 architecture_endian)
        })
    }
    
    gluon::import::add_extern_module(
        &vm, "falcon_architecture_prim", falcon_architecture_prim_loader);
    
    vm
}