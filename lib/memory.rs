use falcon::memory;
use gluon::vm::api::{Userdata, VmType};
use gluon::vm::thread::{Traverseable};
use gluon;

falcon_type_wrapper!(memory::backing::Memory, BackingMemory);


pub fn bindings (vm: gluon::RootedThread) -> gluon::RootedThread {

    vm.register_type::<BackingMemory>("BackingMemory", &[]).unwrap();

    vm.define_global("falcon_memory_prim", record! {
    }).unwrap();
    
    vm
}