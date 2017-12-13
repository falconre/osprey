use falcon;
use falcon::loader::Loader;
use gluon::vm::api::{Userdata, VmType};
use gluon::vm::thread::{Traverseable};
use gluon;
use std::path::Path;

use il;
use memory;
use types;

falcon_type_wrapper!(falcon::loader::Elf, LoaderElf);

fn elf_from_file(filename: String) -> LoaderElf {
    let path = Path::new(&filename);
    LoaderElf {
        x: falcon::loader::Elf::from_file(&path).unwrap()
    }
}

fn elf_base_address(elf: &LoaderElf) -> u64 {
    elf.x.base_address()
}

fn elf_function_entries(elf: &LoaderElf) -> Vec<LoaderFunctionEntry> {
    elf.x
        .function_entries()
        .unwrap()
        .iter()
        .map(|fe| LoaderFunctionEntry { x: fe.clone() })
        .collect::<Vec<LoaderFunctionEntry>>()
}

fn elf_memory(elf: &LoaderElf) -> memory::BackingMemory {
    memory::BackingMemory { x: elf.x.memory().unwrap() }
}

fn elf_function(elf: &LoaderElf, address: u64) -> il::IlFunction {
    il::IlFunction { x: elf.x.function(address).unwrap() }
}

fn elf_architecture(elf: &LoaderElf) -> types::TypesArchitecture {
    types::TypesArchitecture { x: elf.x.architecture().unwrap() }
}

falcon_type_wrapper!(falcon::loader::FunctionEntry, LoaderFunctionEntry);

fn function_entry_name(function_entry: &LoaderFunctionEntry) -> Option<String> {
    function_entry.x.name().clone()
}

fn function_entry_address(function_entry: &LoaderFunctionEntry) -> u64 {
    function_entry.x.address()
}

fn function_entry_str(function_entry: &LoaderFunctionEntry) -> String {
    format!("{}", function_entry.x)
}


pub fn bindings(vm: gluon::RootedThread) -> gluon::RootedThread {

    vm.register_type::<LoaderElf>("LoaderElf", &[]).unwrap();
    vm.register_type::<LoaderFunctionEntry>("LoaderFunctionEntry", &[]).unwrap();

    vm.define_global("falcon_loader_prim", record! {
        elf_architecture => primitive!(1 elf_architecture),
        elf_base_address => primitive!(1 elf_base_address),
        elf_from_file => primitive!(1 elf_from_file),
        elf_function_entries => primitive!(1 elf_function_entries),
        elf_function => primitive!(2 elf_function),
        elf_memory => primitive!(1 elf_memory),
        function_entry_name => primitive!(1 function_entry_name),
        function_entry_address => primitive!(1 function_entry_address),
        function_entry_str => primitive!(1 function_entry_str)
    }).unwrap();
    
    vm
}