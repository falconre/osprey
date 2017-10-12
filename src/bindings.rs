use falcon;
use falcon::loader::Loader;
use falcon::translator::Arch;
use gluon;
use gluon::vm::api::{IO, Userdata, VmType};
use gluon::vm::thread::{Traverseable};
use std::path::Path;


macro_rules! falcon_type_wrapper {
    ($p: path, $n: ident) => {
        #[derive(Clone, Debug)] struct $n { x: $p }
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

fn constant_str(constant: &IlConstant) -> String {
    format!("{}", constant.x)
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

fn scalar_str(scalar: &IlScalar) -> String {
    format!("{}", scalar.x)
}


falcon_type_wrapper!(falcon::il::Array, IlArray);

fn array_new(name: String, size: u64) -> IlArray {
    IlArray { x: falcon::il::Array::new(name, size) }
}

fn array_name(array: &IlArray) -> &str {
    array.x.name()
}

fn array_size(array: &IlArray) -> u64 {
    array.x.size()
}

fn array_str(array: &IlArray) -> String {
    format!("{}", array.x)
}


falcon_type_wrapper!(falcon::il::MultiVar, IlMultiVar);


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

fn expression_type(expr: &IlExpression) -> String {
    match expr.x {
        falcon::il::Expression::Scalar(_) => "scalar",
        falcon::il::Expression::Constant(_) => "constant",
        falcon::il::Expression::Add(_,_) => "add",
        falcon::il::Expression::Sub(_,_) => "sub",
        falcon::il::Expression::Mul(_,_) => "mul",
        falcon::il::Expression::Divu(_,_) => "divu",
        falcon::il::Expression::Modu(_,_) => "modu",
        falcon::il::Expression::Divs(_,_) => "divs",
        falcon::il::Expression::Mods(_,_) => "mods",
        falcon::il::Expression::And(_,_) => "and",
        falcon::il::Expression::Or(_,_) => "or",
        falcon::il::Expression::Xor(_,_) => "xor",
        falcon::il::Expression::Shl(_,_) => "shl",
        falcon::il::Expression::Shr(_,_) => "shr",
        falcon::il::Expression::Cmpeq(_,_) => "cmpeq",
        falcon::il::Expression::Cmpneq(_,_) => "cmpneq",
        falcon::il::Expression::Cmplts(_,_) => "cmplts",
        falcon::il::Expression::Cmpltu(_,_) => "cmpltu",
        falcon::il::Expression::Zext(_,_) => "zext",
        falcon::il::Expression::Sext(_,_) => "sext",
        falcon::il::Expression::Trun(_,_) => "trun",
    }.to_string()
}

fn expression_get_scalar(expr: &IlExpression) -> IlScalar {
    match expr.x {
        falcon::il::Expression::Scalar(ref scalar) =>
            IlScalar { x: scalar.clone() },
        _ => panic!("expression_get_scalar called on non-scalar")
    }
}

fn expression_get_constant(expr: &IlExpression) -> IlConstant {
    match expr.x {
        falcon::il::Expression::Constant(ref constant) =>
            IlConstant { x: constant.clone() },
        _ => panic!("expression_get_constant called on non-constant")
    }
}

fn expression_get_lhs(expr: &IlExpression) -> IlExpression {
    match expr.x {
        falcon::il::Expression::Add(ref lhs, _) |
        falcon::il::Expression::Sub(ref lhs, _) |
        falcon::il::Expression::Mul(ref lhs, _) |
        falcon::il::Expression::Divu(ref lhs, _) |
        falcon::il::Expression::Modu(ref lhs, _) |
        falcon::il::Expression::Divs(ref lhs, _) |
        falcon::il::Expression::Mods(ref lhs, _) |
        falcon::il::Expression::And(ref lhs, _) |
        falcon::il::Expression::Or(ref lhs, _) |
        falcon::il::Expression::Xor(ref lhs, _) |
        falcon::il::Expression::Shl(ref lhs, _) |
        falcon::il::Expression::Shr(ref lhs, _) |
        falcon::il::Expression::Cmpeq(ref lhs, _) |
        falcon::il::Expression::Cmpneq(ref lhs, _) |
        falcon::il::Expression::Cmplts(ref lhs, _) |
        falcon::il::Expression::Cmpltu(ref lhs, _) =>
            IlExpression { x: *lhs.clone() },
        _ => panic!("expression_get_lhs called on expr without lhs")
    }
}

fn expression_get_rhs(expr: &IlExpression) -> IlExpression {
    match expr.x {
        falcon::il::Expression::Add(_, ref rhs) |
        falcon::il::Expression::Sub(_, ref rhs) |
        falcon::il::Expression::Mul(_, ref rhs) |
        falcon::il::Expression::Divu(_, ref rhs) |
        falcon::il::Expression::Modu(_, ref rhs) |
        falcon::il::Expression::Divs(_, ref rhs) |
        falcon::il::Expression::Mods(_, ref rhs) |
        falcon::il::Expression::And(_, ref rhs) |
        falcon::il::Expression::Or(_, ref rhs) |
        falcon::il::Expression::Xor(_, ref rhs) |
        falcon::il::Expression::Shl(_, ref rhs) |
        falcon::il::Expression::Shr(_, ref rhs) |
        falcon::il::Expression::Cmpeq(_, ref rhs) |
        falcon::il::Expression::Cmpneq(_, ref rhs) |
        falcon::il::Expression::Cmplts(_, ref rhs) |
        falcon::il::Expression::Cmpltu(_, ref rhs) |
        falcon::il::Expression::Zext(_, ref rhs) |
        falcon::il::Expression::Sext(_, ref rhs) |
        falcon::il::Expression::Trun(_, ref rhs) =>
            IlExpression { x: *rhs.clone() },
        _ => panic!("expression_get_rhs called on expr without rhs")
    }
}

fn expression_get_bits(expr: &IlExpression) -> usize {
    match expr.x {
        falcon::il::Expression::Zext(bits, _) |
        falcon::il::Expression::Sext(bits, _) |
        falcon::il::Expression::Trun(bits, _) => bits,
        _ => panic!("expression_get_bits called on expr without bits")
    }
}

fn expression_str(expr: &IlExpression) -> String {
    format!("{}", expr.x)
}


falcon_type_wrapper!(falcon::il::Operation, IlOperation);

fn operation_assign(dst: &IlScalar, src: &IlExpression) -> IlOperation {
    IlOperation { x: falcon::il::Operation::assign(dst.x.clone(), src.x.clone()) }
}

fn operation_store(dst: &IlArray, index: &IlExpression, src: &IlExpression) -> IlOperation {
    IlOperation { x: falcon::il::Operation::store(dst.x.clone(), index.x.clone(), src.x.clone()) }
}

fn operation_load(dst: &IlScalar, index: &IlExpression, src: &IlArray) -> IlOperation {
    IlOperation { x: falcon::il::Operation::load(dst.x.clone(), index.x.clone(), src.x.clone()) }
}

fn operation_brc(target: &IlExpression, condition: &IlExpression) -> IlOperation {
    IlOperation { x: falcon::il::Operation::brc(target.x.clone(), condition.x.clone()) }
}

fn operation_raise(expr: &IlExpression) -> IlOperation {
    IlOperation { x: falcon::il::Operation::raise(expr.x.clone())}
}

fn operation_type(operation: &IlOperation) -> String {
    match operation.x {
        falcon::il::Operation::Assign { .. } => "assign",
        falcon::il::Operation::Store  { .. } => "store",
        falcon::il::Operation::Load   { .. } => "load",
        falcon::il::Operation::Brc    { .. } => "brc",
        falcon::il::Operation::Raise  { .. } => "raise",
        falcon::il::Operation::Phi    { .. } => "phi"
    }.to_string()
}

fn operation_assign_dst(operation: &IlOperation) -> IlScalar {
    match operation.x {
        falcon::il::Operation::Assign {ref dst, ..} => IlScalar { x: dst.clone() },
        _ => panic!("operation_assign_dst called on non-assign op")
    }
}

fn operation_assign_src(operation: &IlOperation) -> IlExpression {
    match operation.x {
        falcon::il::Operation::Assign {ref src, ..} => IlExpression { x: src.clone() },
        _ => panic!("operation_assign_src called on non-assign op")
    }
}

fn operation_store_dst(operation: &IlOperation) -> IlArray {
    match operation.x {
        falcon::il::Operation::Store {ref dst, ..} => IlArray { x: dst.clone() },
        _ => panic!("operation_store_dst called on non-store op")
    }
}

fn operation_store_index(operation: &IlOperation) -> IlExpression {
    match operation.x {
        falcon::il::Operation::Store {ref index, ..} => IlExpression { x: index.clone() },
        _ => panic!("operation_store_index called on non-store op")
    }
}

fn operation_store_src(operation: &IlOperation) -> IlExpression {
    match operation.x {
        falcon::il::Operation::Store {ref src, ..} => IlExpression { x: src.clone() },
        _ => panic!("operation_store_src called on non-store op")
    }
}

fn operation_load_dst(operation: &IlOperation) -> IlScalar {
    match operation.x {
        falcon::il::Operation::Load {ref dst, ..} => IlScalar { x: dst.clone() },
        _ => panic!("operation_load_dst called on non-load op")
    }
}

fn operation_load_index(operation: &IlOperation) -> IlExpression {
    match operation.x {
        falcon::il::Operation::Load {ref index, ..} => IlExpression { x: index.clone() },
        _ => panic!("operation_load_index called on non-load op")
    }
}

fn operation_load_src(operation: &IlOperation) -> IlArray {
    match operation.x {
        falcon::il::Operation::Load {ref src, ..} => IlArray { x: src.clone() },
        _ => panic!("operation_load_src called on non-load op")
    }
}

fn operation_brc_target(operation: &IlOperation) -> IlExpression {
    match operation.x {
        falcon::il::Operation::Brc {ref target, ..} => IlExpression { x: target.clone() },
        _ => panic!("operation_brc_target called on non-brc op")
    }
}

fn operation_brc_condition(operation: &IlOperation) -> IlExpression {
    match operation.x {
        falcon::il::Operation::Brc {ref condition, ..} => IlExpression { x: condition.clone() },
        _ => panic!("operation_brc_condition called on non-brc op")
    }
}

fn operation_raise_expr(operation: &IlOperation) -> IlExpression {
    match operation.x {
        falcon::il::Operation::Raise {ref expr } => IlExpression { x: expr.clone() },
        _ => panic!("operation_raise_expr called on non-raise op")
    }
}

fn operation_str(operation: &IlOperation) -> String {
    format!("{}", operation.x)
}


falcon_type_wrapper!(falcon::il::Instruction, IlInstruction);

fn instruction_operation(instruction: &IlInstruction) -> IlOperation {
    IlOperation { x: instruction.x.operation().clone() }
}

fn instruction_index(instruction: &IlInstruction) -> u64 {
    instruction.x.index()
}

fn instruction_str(instruction: &IlInstruction) -> String {
    format!("{}", instruction.x)
}


falcon_type_wrapper!(falcon::il::Block, IlBlock);

fn block_index(block: &IlBlock) -> u64 {
    block.x.index()
}

fn block_instructions(block: &IlBlock) -> Vec<IlInstruction> {
    block.x
         .instructions()
         .iter()
         .map(|i| IlInstruction {x: i.clone()})
         .collect::<Vec<IlInstruction>>()
}

fn block_assign(block: &IlBlock, dst: &IlScalar, src: &IlExpression) -> IlBlock {
    let mut block = block.clone();
    block.x.assign(dst.x.clone(), src.x.clone());
    block
}

fn block_store(block: &IlBlock, dst: &IlArray, index: &IlExpression, src: &IlExpression) -> IlBlock {
    let mut block = block.clone();
    block.x.store(dst.x.clone(), index.x.clone(), src.x.clone());
    block
}

fn block_load(block: &IlBlock, dst: &IlScalar, index: &IlExpression, src: &IlArray) -> IlBlock {
    let mut block = block.clone();
    block.x.load(dst.x.clone(), index.x.clone(), src.x.clone());
    block
}

fn block_brc(block: &IlBlock, target: &IlExpression, condition: &IlExpression) -> IlBlock {
    let mut block = block.clone();
    block.x.brc(target.x.clone(), condition.x.clone());
    block
}

fn block_raise(block: &IlBlock, expr: &IlExpression) -> IlBlock {
    let mut block = block.clone();
    block.x.raise(expr.x.clone());
    block
}

fn block_str(block: &IlBlock) -> String {
    format!("{}", block.x)
}


falcon_type_wrapper!(falcon::il::Edge, IlEdge);

fn edge_has_condition(edge: &IlEdge) -> bool {
    if let Some(_) = *edge.x.condition() {
        true
    }
    else {
        false
    }
}

fn edge_condition(edge: &IlEdge) -> IlExpression {
    IlExpression { x: edge.x.condition().clone().unwrap() }
}

fn edge_head(edge: &IlEdge) -> u64 {
    edge.x.head()
}

fn edge_tail(edge: &IlEdge) -> u64 {
    edge.x.tail()
}

fn edge_str(edge: &IlEdge) -> String {
    format!("{}", edge.x)
}


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

fn control_flow_graph_str(control_flow_graph: &IlControlFlowGraph) -> String {
    format!("{}", control_flow_graph.x)
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

fn function_entry_str(function_entry: &LoaderFunctionEntry) -> String {
    format!("{}", function_entry.x)
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
    vm.register_type::<IlArray>("IlArray", &[]).unwrap();
    vm.register_type::<IlMultiVar>("IlMultiVar", &[]).unwrap();
    vm.register_type::<IlExpression>("IlExpression", &[]).unwrap();
    vm.register_type::<IlOperation>("IlOperation", &[]).unwrap();
    vm.register_type::<IlInstruction>("IlInstruction", &[]).unwrap();
    vm.register_type::<IlBlock>("IlBlock", &[]).unwrap();
    vm.register_type::<IlEdge>("IlEdge", &[]).unwrap();
    vm.register_type::<IlControlFlowGraph>("IlControlFlowGraph", &[]).unwrap();
    vm.register_type::<IlFunction>("IlFunction", &[]).unwrap();
    vm.register_type::<IlProgram>("IlProgram", &[]).unwrap();
    vm.register_type::<IlProgramLocation>("IlProgramLocation", &[]).unwrap();
    vm.register_type::<LoaderElf>("LoaderElf", &[]).unwrap();
    vm.register_type::<LoaderFunctionEntry>("LoaderFunctionEntry", &[]).unwrap();
    vm.register_type::<LoaderMemory>("LoaderMemory", &[]).unwrap();

    vm.define_global("falcon_prim", record! {
        array_new => primitive!(2 array_new),
        array_name => primitive!(1 array_name),
        array_size => primitive!(1 array_size),
        array_str => primitive!(1 array_str),
        block_index => primitive!(1 block_index),
        block_instructions => primitive!(1 block_instructions),
        block_assign => primitive!(3 block_assign),
        block_store => primitive!(4 block_store),
        block_load => primitive!(4 block_load),
        block_brc => primitive!(3 block_brc),
        block_raise => primitive!(2 block_raise),
        block_str => primitive!(1 block_str),
        constant_new => primitive!(2 constant_new),
        constant_bits => primitive!(1 constant_bits),
        constant_value => primitive!(1 constant_value),
        constant_str => primitive!(1 constant_str),
        control_flow_graph_blocks => primitive!(1 control_flow_graph_blocks),
        control_flow_graph_dot_graph => primitive!(1 control_flow_graph_dot_graph),
        control_flow_graph_str => primitive!(1 control_flow_graph_str),
        edge_has_condition => primitive!(1 edge_has_condition),
        edge_condition => primitive!(1 edge_condition),
        edge_head => primitive!(1 edge_head),
        edge_tail => primitive!(1 edge_tail),
        edge_str => primitive!(1 edge_str),
        elf_base_address => primitive!(1 elf_base_address),
        elf_from_file => primitive!(1 elf_from_file),
        elf_function_entries => primitive!(1 elf_function_entries),
        elf_function => primitive!(2 elf_function),
        elf_memory => primitive!(1 elf_memory),
        expression_scalar => primitive!(1 expression_scalar),
        expression_constant => primitive!(1 expression_constant),
        expression_add => primitive!(2 expression_add),
        expression_sub => primitive!(2 expression_sub),
        expression_mul => primitive!(2 expression_mul),
        expression_divu => primitive!(2 expression_divu),
        expression_modu => primitive!(2 expression_modu),
        expression_divs => primitive!(2 expression_divs),
        expression_mods => primitive!(2 expression_mods),
        expression_and => primitive!(2 expression_and),
        expression_or => primitive!(2 expression_or),
        expression_xor => primitive!(2 expression_xor),
        expression_shl => primitive!(2 expression_shl),
        expression_shr => primitive!(2 expression_shr),
        expression_cmpeq => primitive!(2 expression_cmpeq),
        expression_cmpneq => primitive!(2 expression_cmpneq),
        expression_cmplts => primitive!(2 expression_cmplts),
        expression_cmpltu => primitive!(2 expression_cmpltu),
        expression_zext => primitive!(2 expression_zext),
        expression_sext => primitive!(2 expression_sext),
        expression_trun => primitive!(2 expression_trun),
        expression_type => primitive!(1 expression_type),
        expression_get_scalar => primitive!(1 expression_get_scalar),
        expression_get_constant => primitive!(1 expression_get_constant),
        expression_get_lhs => primitive!(1 expression_get_lhs),
        expression_get_rhs => primitive!(1 expression_get_rhs),
        expression_get_bits => primitive!(1 expression_get_bits),
        expression_str => primitive!(1 expression_str),
        function_control_flow_graph => primitive!(1 function_control_flow_graph),
        function_entry_name => primitive!(1 function_entry_name),
        function_entry_address => primitive!(1 function_entry_address),
        function_entry_str => primitive!(1 function_entry_str),
        instruction_index => primitive!(1 instruction_index),
        instruction_operation => primitive!(1 instruction_operation),
        instruction_str => primitive!(1 instruction_str),
        hex => primitive!(1 hex),
        mips_translate_function => primitive!(2 mips_translate_function),
        operation_assign => primitive!(2 operation_assign),
        operation_store => primitive!(3 operation_store),
        operation_load => primitive!(3 operation_load),
        operation_brc => primitive!(2 operation_brc),
        operation_raise => primitive!(1 operation_raise),
        operation_type => primitive!(1 operation_type),
        operation_assign_src => primitive!(1 operation_assign_src),
        operation_assign_dst => primitive!(1 operation_assign_dst),
        operation_store_dst => primitive!(1 operation_store_dst),
        operation_store_index => primitive!(1 operation_store_index),
        operation_store_src => primitive!(1 operation_store_src),
        operation_load_dst => primitive!(1 operation_load_dst),
        operation_load_index => primitive!(1 operation_load_index),
        operation_load_src => primitive!(1 operation_load_src),
        operation_brc_target => primitive!(1 operation_brc_target),
        operation_brc_condition => primitive!(1 operation_brc_condition),
        operation_raise_expr => primitive!(1 operation_raise_expr),
        operation_str => primitive!(1 operation_str),
        println => primitive!(1 println),
        program_new => primitive!(0 program_new),
        scalar_new => primitive!(2 scalar_new),
        scalar_name => primitive!(1 scalar_name),
        scalar_bits => primitive!(1 scalar_bits),
        scalar_str => primitive!(1 scalar_str)
    }).unwrap();
    
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