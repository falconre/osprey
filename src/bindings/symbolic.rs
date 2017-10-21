use falcon;
use falcon::symbolic;
use gluon::vm::api::{Userdata, VmType};
use gluon::vm::thread::{Traverseable};
use gluon;

use bindings::loader;
use bindings::types;

falcon_type_wrapper!(symbolic::Memory, SymbolicMemory);

fn memory_from_loader_memory(
    loader_memory: &loader::LoaderMemory,
    endian: &types::TypesEndian
) -> SymbolicMemory {

    let mut memory = symbolic::Memory::new(endian.x.clone());
    for (address, segment) in loader_memory.x.segments() {
        let bytes = segment.bytes();
        for i in 0..bytes.len() {
            memory.store(*address + i as u64, falcon::il::expr_const(bytes[i] as u64, 8)).unwrap();
        }
    }
    SymbolicMemory { x: memory }
}


falcon_type_wrapper!(symbolic::Engine, SymbolicEngine);

fn engine_new(memory: &SymbolicMemory) -> SymbolicEngine {
    SymbolicEngine { x: symbolic::Engine::new(memory.x.clone()) }
}



pub fn bindings (vm: gluon::RootedThread) -> gluon::RootedThread {

    vm.register_type::<SymbolicEngine>("SymbolicEngine", &[]).unwrap();
    vm.register_type::<SymbolicMemory>("SymbolicMemory", &[]).unwrap();

    vm.define_global("falcon_symbolic_prim", record! {
        // engine_new => primitive!(1 engine_new),
        // memory_from_loader_memory => primitive!(2 memory_from_loader_memory)
    }).unwrap();
    
    vm
}