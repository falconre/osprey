use falcon;
use falcon::loader::Loader;
use gluon::vm::api::{Userdata, VmType};
use gluon::vm::thread::{Traverseable};
use gluon;
use std::path::Path;

use bindings::il;

falcon_type_wrapper!(falcon::loader::elf::Elf, LoaderElf);

fn elf_from_file(filename: String) -> LoaderElf {
    let path = Path::new(&filename);
    LoaderElf {
        x: falcon::loader::elf::Elf::from_file(&path).unwrap()
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

fn elf_memory(elf: &LoaderElf) -> LoaderMemory {
    LoaderMemory { x: elf.x.memory().unwrap() }
}

fn elf_function(elf: &LoaderElf, address: u64) -> il::IlFunction {
    il::IlFunction { x: elf.x.function(address).unwrap() }
}

falcon_type_wrapper!(falcon::loader::FunctionEntry, LoaderFunctionEntry);

fn function_entry_name(function_entry: &LoaderFunctionEntry) -> String {
    function_entry.x.name().to_string()
}

fn function_entry_address(function_entry: &LoaderFunctionEntry) -> u64 {
    function_entry.x.address()
}

fn function_entry_str(function_entry: &LoaderFunctionEntry) -> String {
    format!("{}", function_entry.x)
}

falcon_type_wrapper!(falcon::loader::memory::Memory, LoaderMemory);


falcon_type_wrapper!(falcon::executor::engine::Engine, ExecutorEngine);
falcon_type_wrapper!(falcon::executor::memory::Memory, ExecutorMemory);


pub fn bindings (vm: gluon::RootedThread) -> gluon::RootedThread {

    vm.register_type::<LoaderElf>("LoaderElf", &[]).unwrap();
    vm.register_type::<LoaderFunctionEntry>("LoaderFunctionEntry", &[]).unwrap();
    vm.register_type::<LoaderMemory>("LoaderMemory", &[]).unwrap();

    vm.define_global("falcon_loader_prim", record! {
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