use falcon;
use falcon::loader::Loader;
use falcon::translator::Arch;
use gluon;
use gluon::vm::api::{IO, Userdata, VmType};
use gluon::vm::thread::{Traverseable};
use std::path::Path;


macro_rules! falcon_type_wrapper {
    ($p: path, $n: ident) => {
        #[derive(Clone, Debug)] struct $n {x: $p}
        impl VmType for $n { type Type = $n; }
        impl Traverseable for $n {}
        impl Userdata for $n {}
    }
}


falcon_type_wrapper!(falcon::il::Constant, IlConstant);

fn constant_new(value: u64, bits: usize) -> IlConstant {
    IlConstant { x: falcon::il::Constant::new(value, bits) }
}

fn constant_value(constant: &IlConstant) -> u64 {
    constant.x.value()
}

fn constant_bits(constant: &IlConstant) -> usize {
    constant.x.bits()
}


falcon_type_wrapper!(falcon::il::Scalar, IlScalar);

fn scalar_new(name: String, bits: usize) -> IlScalar {
    IlScalar { x: falcon::il::Scalar::new(name, bits) }
}

fn scalar_name(scalar: &IlScalar) -> &str {
    scalar.x.name()
}

fn scalar_bits(scalar: &IlScalar) -> usize {
    scalar.x.bits()
}


falcon_type_wrapper!(falcon::il::Expression, IlExpression);

fn expression_scalar(scalar: &IlScalar) -> IlExpression {
    IlExpression {
        x: falcon::il::Expression::Scalar(scalar.x.clone())
    }
}

fn expression_constant(constant: &IlConstant) -> IlExpression {
    IlExpression {
        x: falcon::il::Expression::Constant(constant.x.clone())
    }
}

macro_rules! falcon_expression {
    ($name: ident, $func: path) => {
        fn $name(lhs: &IlExpression, rhs: &IlExpression) -> IlExpression {
            IlExpression {
                x: $func(lhs.x.clone(), rhs.x.clone()).unwrap()
            }
        }
    }
}

falcon_expression!(expression_add, falcon::il::Expression::add);
falcon_expression!(expression_sub, falcon::il::Expression::sub);
falcon_expression!(expression_mul, falcon::il::Expression::mul);
falcon_expression!(expression_divu, falcon::il::Expression::divu);
falcon_expression!(expression_modu, falcon::il::Expression::modu);
falcon_expression!(expression_divs, falcon::il::Expression::divs);
falcon_expression!(expression_mods, falcon::il::Expression::mods);
falcon_expression!(expression_and, falcon::il::Expression::and);
falcon_expression!(expression_or, falcon::il::Expression::or);
falcon_expression!(expression_xor, falcon::il::Expression::xor);
falcon_expression!(expression_shl, falcon::il::Expression::shl);
falcon_expression!(expression_shr, falcon::il::Expression::shr);
falcon_expression!(expression_cmpeq, falcon::il::Expression::cmpeq);
falcon_expression!(expression_cmpneq, falcon::il::Expression::cmpneq);
falcon_expression!(expression_cmplts, falcon::il::Expression::cmplts);
falcon_expression!(expression_cmpltu, falcon::il::Expression::cmpltu);

fn expression_zext(bits: usize, expr: &IlExpression) -> IlExpression {
    IlExpression { x: falcon::il::Expression::zext(bits, expr.x.clone()).unwrap() }
}

fn expression_sext(bits: usize, expr: &IlExpression) -> IlExpression {
    IlExpression { x: falcon::il::Expression::zext(bits, expr.x.clone()).unwrap() }
}

fn expression_trun(bits: usize, expr: &IlExpression) -> IlExpression {
    IlExpression { x: falcon::il::Expression::zext(bits, expr.x.clone()).unwrap() }
}


falcon_type_wrapper!(falcon::il::Operation, IlOperation);
falcon_type_wrapper!(falcon::il::Instruction, IlInstruction);
falcon_type_wrapper!(falcon::il::Block, IlBlock);


falcon_type_wrapper!(falcon::il::ControlFlowGraph, IlControlFlowGraph);
    
fn control_flow_graph_blocks(control_flow_graph: &IlControlFlowGraph)
    -> Vec<IlBlock> {

    control_flow_graph.x
                      .blocks()
                      .iter()
                      .map(|b| IlBlock { x: (*b).clone() })
                      .collect::<Vec<IlBlock>>()
}

fn control_flow_graph_dot_graph(control_flow_graph: &IlControlFlowGraph) -> String {
    control_flow_graph.x.graph().dot_graph()
}

falcon_type_wrapper!(falcon::il::Function, IlFunction);

fn function_control_flow_graph(function: &IlFunction) -> IlControlFlowGraph {
    IlControlFlowGraph {
        x: function.x.control_flow_graph().clone()
    }
}

falcon_type_wrapper!(falcon::il::Program, IlProgram);

fn program_new() -> IlProgram {
    IlProgram { x: falcon::il::Program::new() }
}



falcon_type_wrapper!(falcon::il::ProgramLocation, IlProgramLocation);


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

fn elf_function(elf: &LoaderElf, address: u64) -> IlFunction {
    IlFunction { x: elf.x.function(address).unwrap() }
}

falcon_type_wrapper!(falcon::loader::FunctionEntry, LoaderFunctionEntry);

fn function_entry_name(function_entry: &LoaderFunctionEntry) -> String {
    function_entry.x.name().to_string()
}

fn function_entry_address(function_entry: &LoaderFunctionEntry) -> u64 {
    function_entry.x.address()
}

falcon_type_wrapper!(falcon::loader::memory::Memory, LoaderMemory);


falcon_type_wrapper!(falcon::executor::engine::Engine, ExecutorEngine);
falcon_type_wrapper!(falcon::executor::memory::Memory, ExecutorMemory);

// falcon_type_wrapper!(falcon::translator::mips::Mips, TranslatorMips);

// fn mips_new() -> TranslatorMips {
//     TranslatorMips { x: falcon::translator::mips::Mips::new() }
// }


fn mips_translate_function(
    memory: &LoaderMemory,
    address: u64
) -> IlFunction {
        let mips = falcon::translator::mips::Mips::new();
        IlFunction { x: mips.translate_function(&memory.x, address).unwrap() }
}


fn hex(u: usize) -> String {
    format!("{:x}", u)
}


fn println(string: String) {
    println!("{}", string);
}


pub fn bindings (vm: gluon::RootedThread) -> gluon::RootedThread {

    vm.register_type::<IlConstant>("IlConstant", &[]).unwrap();
    vm.register_type::<IlScalar>("IlScalar", &[]).unwrap();
    vm.register_type::<IlExpression>("IlExpression", &[]).unwrap();
    vm.register_type::<IlOperation>("IlOperation", &[]).unwrap();
    vm.register_type::<IlInstruction>("IlInstruction", &[]).unwrap();
    vm.register_type::<IlBlock>("IlBlock", &[]).unwrap();
    vm.register_type::<IlControlFlowGraph>("IlControlFlowGraph", &[]).unwrap();
    vm.register_type::<IlFunction>("IlFunction", &[]).unwrap();
    vm.register_type::<IlProgram>("IlProgram", &[]).unwrap();
    vm.register_type::<IlProgramLocation>("IlProgramLocation", &[]).unwrap();
    vm.register_type::<LoaderElf>("LoaderElf", &[]).unwrap();
    vm.register_type::<LoaderFunctionEntry>("LoaderFunctionEntry", &[]).unwrap();
    vm.register_type::<LoaderMemory>("LoaderMemory", &[]).unwrap();
    // vm.register_type::<TranslatorMips>("TranslatorMips", &[]).unwrap();
    vm.define_global("constant_new", primitive!(2 constant_new)).unwrap();
    vm.define_global("constant_bits", primitive!(1 constant_bits)).unwrap();
    vm.define_global("constant_value", primitive!(1 constant_value)).unwrap();
    vm.define_global("control_flow_graph_blocks", primitive!(1 control_flow_graph_blocks)).unwrap();
    vm.define_global("control_flow_graph_dot_graph", primitive!(1 control_flow_graph_dot_graph)).unwrap();
    vm.define_global("elf_base_address", primitive!(1 elf_base_address)).unwrap();
    vm.define_global("elf_from_file", primitive!(1 elf_from_file)).unwrap();
    vm.define_global("elf_function_entries", primitive!(1 elf_function_entries)).unwrap();
    vm.define_global("elf_function", primitive!(2 elf_function)).unwrap();
    vm.define_global("elf_memory", primitive!(1 elf_memory)).unwrap();
    vm.define_global("expression_scalar", primitive!(1 expression_scalar)).unwrap();
    vm.define_global("expression_constant", primitive!(1 expression_constant)).unwrap();
    vm.define_global("expression_add", primitive!(2 expression_add)).unwrap();
    vm.define_global("expression_sub", primitive!(2 expression_sub)).unwrap();
    vm.define_global("expression_mul", primitive!(2 expression_mul)).unwrap();
    vm.define_global("expression_divu", primitive!(2 expression_divu)).unwrap();
    vm.define_global("expression_modu", primitive!(2 expression_modu)).unwrap();
    vm.define_global("expression_divs", primitive!(2 expression_divs)).unwrap();
    vm.define_global("expression_mods", primitive!(2 expression_mods)).unwrap();
    vm.define_global("expression_and", primitive!(2 expression_and)).unwrap();
    vm.define_global("expression_or", primitive!(2 expression_or)).unwrap();
    vm.define_global("expression_xor", primitive!(2 expression_xor)).unwrap();
    vm.define_global("expression_shl", primitive!(2 expression_shl)).unwrap();
    vm.define_global("expression_shr", primitive!(2 expression_shr)).unwrap();
    vm.define_global("expression_cmpeq", primitive!(2 expression_cmpeq)).unwrap();
    vm.define_global("expression_cmpneq", primitive!(2 expression_cmpneq)).unwrap();
    vm.define_global("expression_cmplts", primitive!(2 expression_cmplts)).unwrap();
    vm.define_global("expression_cmpltu", primitive!(2 expression_cmpltu)).unwrap();
    vm.define_global("expression_zext", primitive!(2 expression_zext)).unwrap();
    vm.define_global("expression_sext", primitive!(2 expression_sext)).unwrap();
    vm.define_global("expression_trun", primitive!(2 expression_trun)).unwrap();
    vm.define_global("function_control_flow_graph", primitive!(1 function_control_flow_graph)).unwrap();
    vm.define_global("function_entry_name", primitive!(1 function_entry_name)).unwrap();
    vm.define_global("function_entry_address", primitive!(1 function_entry_address)).unwrap();
    vm.define_global("hex", primitive!(1 hex)).unwrap();
    // vm.define_global("mips_new", primitive!(0 mips_new)).unwrap();
    vm.define_global("mips_translate_function", primitive!(2 mips_translate_function)).unwrap();
    vm.define_global("println", primitive!(1 println)).unwrap();
    vm.define_global("program_new", primitive!(0 program_new)).unwrap();
    vm.define_global("scalar_new", primitive!(2 scalar_new)).unwrap();
    vm.define_global("scalar_name", primitive!(1 scalar_name)).unwrap();
    vm.define_global("scalar_bits", primitive!(1 scalar_bits)).unwrap();
    
    vm
}


pub fn run_code(code: &str) -> gluon::RootedThread {
    let vm = gluon::new_vm();

    let vm = bindings(vm);

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