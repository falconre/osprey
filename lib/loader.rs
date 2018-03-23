use falcon;
use falcon::loader::Loader;
use gluon::vm::api::{Userdata, VmType};
use gluon::vm::thread::{Traverseable};
use gluon;
use std::path::Path;
use std::sync::Arc;

use architecture;
use il;
use memory;

falcon_type_wrapper!(Arc<falcon::loader::Elf>, LoaderElf);

fn elf_architecture(elf: &LoaderElf) -> architecture::ArchitectureArchitecture {
    architecture::ArchitectureArchitecture {
        x: Arc::new(elf.x.architecture().box_clone())
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

fn elf_from_file(filename: String) -> LoaderElf {
    let path = Path::new(&filename);
    LoaderElf {
        x: Arc::new(falcon::loader::Elf::from_file(&path).unwrap())
    }
}

fn elf_function(elf: &LoaderElf, address: u64) -> il::IlFunction {
    il::IlFunction { x: elf.x.function(address).unwrap() }
}

fn elf_memory(elf: &LoaderElf) -> memory::BackingMemory {
    memory::BackingMemory { x: elf.x.memory().unwrap() }
}

fn elf_program(elf: &LoaderElf) -> il::IlProgram {
    il::IlProgram { x: elf.x.program().unwrap() }
}

fn elf_program_recursive(elf: &LoaderElf) -> il::IlProgram {
    il::IlProgram { x: elf.x.program_recursive().unwrap() }
}

falcon_type_wrapper!(Arc<falcon::loader::Pe>, LoaderPe);

fn pe_architecture(pe: &LoaderPe) -> architecture::ArchitectureArchitecture {
    architecture::ArchitectureArchitecture {
        x: Arc::new(pe.x.architecture().box_clone())
    }
}

fn pe_from_file(filename: String) -> LoaderPe {
    let path = Path::new(&filename);
    LoaderPe {
        x: Arc::new(falcon::loader::Pe::from_file(&path).unwrap())
    }
}

fn pe_function(pe: &LoaderPe, address: u64) -> il::IlFunction {
    il::IlFunction { x: pe.x.function(address).unwrap() }
}

fn pe_function_entries(pe: &LoaderPe) -> Vec<LoaderFunctionEntry> {
    pe.x
      .function_entries()
      .unwrap()
      .iter()
      .map(|fe| LoaderFunctionEntry { x: fe.clone() })
      .collect::<Vec<LoaderFunctionEntry>>()
}

fn pe_memory(pe: &LoaderPe) -> memory::BackingMemory {
    memory::BackingMemory { x: pe.x.memory().unwrap() }
}

fn pe_program(pe: &LoaderPe) -> il::IlProgram {
    il::IlProgram { x: pe.x.program().unwrap() }
}

fn pe_program_recursive(pe: &LoaderPe) -> il::IlProgram {
    il::IlProgram { x: pe.x.program_recursive().unwrap() }
}

falcon_type_wrapper!(Arc<falcon::loader::Loader>, LoaderLoader);

fn loader_from_file(filename: String) -> Option<LoaderLoader> {
    let path = Path::new(&filename);

    let loader = falcon::loader::Elf::from_file(&path)
        .ok()
        .map(|elf| Arc::new(elf));

    if let Some(loader) = loader {
        return Some(LoaderLoader {x: loader});
    }

    let loader = falcon::loader::Pe::from_file(&path)
            .ok()
            .map(|pe| Arc::new(pe))?;

    Some(LoaderLoader { x: loader })
}

fn loader_architecture(loader: &LoaderLoader) -> architecture::ArchitectureArchitecture {
    architecture::ArchitectureArchitecture {
        x: Arc::new(loader.x.architecture().box_clone())
    }
}

fn loader_function(loader: &LoaderLoader, address: u64) -> il::IlFunction {
    il::IlFunction { x: loader.x.function(address).unwrap() }
}

fn loader_function_entries(loader: &LoaderLoader) -> Vec<LoaderFunctionEntry> {
    loader.x
        .function_entries()
        .unwrap()
        .iter()
        .map(|fe| LoaderFunctionEntry { x: fe.clone() })
        .collect::<Vec<LoaderFunctionEntry>>()
}

fn loader_memory(loader: &LoaderLoader) -> memory::BackingMemory {
    memory::BackingMemory { x: loader.x.memory().unwrap() }
}

fn loader_program(loader: &LoaderLoader) -> il::IlProgram {
    il::IlProgram { x: loader.x.program().unwrap() }
}

fn loader_program_recursive(loader: &LoaderLoader) -> il::IlProgram {
    il::IlProgram { x: loader.x.program_recursive().unwrap() }
}

falcon_type_wrapper!(falcon::loader::FunctionEntry, LoaderFunctionEntry);

fn function_entry_name(function_entry: &LoaderFunctionEntry) -> Option<String> {
    function_entry.x.name().map(|s| s.to_string())
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
    vm.register_type::<LoaderLoader>("LoaderLoader", &[]).unwrap();
    vm.register_type::<LoaderPe>("LoaderPe", &[]).unwrap();

    fn falcon_loader_prim_loader(vm: &gluon::Thread)
        -> gluon::vm::Result<gluon::vm::ExternModule> {
        
        gluon::vm::ExternModule::new(vm, record! {
            elf_architecture => primitive!(1 elf_architecture),
            elf_base_address => primitive!(1 elf_base_address),
            elf_from_file => primitive!(1 elf_from_file),
            elf_function_entries => primitive!(1 elf_function_entries),
            elf_function => primitive!(2 elf_function),
            elf_memory => primitive!(1 elf_memory),
            elf_program => primitive!(1 elf_program),
            elf_program_recursive => primitive!(1 elf_program_recursive),
            function_entry_name => primitive!(1 function_entry_name),
            function_entry_address => primitive!(1 function_entry_address),
            function_entry_str => primitive!(1 function_entry_str),
            loader_architecture => primitive!(1 loader_architecture),
            loader_from_file => primitive!(1 loader_from_file),
            loader_function => primitive!(2 loader_function),
            loader_function_entries => primitive!(1 loader_function_entries),
            loader_memory => primitive!(1 loader_memory),
            loader_program => primitive!(1 loader_program),
            loader_program_recursive => primitive!(1 loader_program_recursive),
            pe_architecture => primitive!(1 pe_architecture),
            pe_from_file => primitive!(1 pe_from_file),
            pe_function_entries => primitive!(1 pe_function_entries),
            pe_function => primitive!(2 pe_function),
            pe_memory => primitive!(1 pe_memory),
            pe_program => primitive!(1 pe_program),
            pe_program_recursive => primitive!(1 pe_program_recursive),
        })
    }
    
    gluon::import::add_extern_module(
        &vm, "falcon_loader_prim", falcon_loader_prim_loader);

    vm
}