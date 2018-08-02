use falcon;
use gluon::vm::api::{Userdata, VmType};
use gluon::vm::thread::{Traverseable};
use gluon;



falcon_type_wrapper!(falcon::il::Constant, IlConstant);

fn constant_new(value: u64, bits: usize) -> IlConstant {
    IlConstant { x: falcon::il::Constant::new(value, bits) }
}

fn constant_format(constant: &IlConstant) -> String {
    format!("{}", constant.x)
}

fn constant_value_u64(constant: &IlConstant) -> Option<u64> {
    constant.x.value_u64()
}

fn constant_bits(constant: &IlConstant) -> usize {
    constant.x.bits()
}

fn constant_str(constant: &IlConstant) -> String {
    format!("{}", constant.x)
}

fn constant_eq(lhs: &IlConstant, rhs: &IlConstant) -> bool {
    lhs.x == rhs.x
}


falcon_type_wrapper!(falcon::il::Scalar, IlScalar);

fn scalar_new(name: String, bits: usize) -> IlScalar {
    IlScalar { x: falcon::il::Scalar::new(name, bits) }
}

fn scalar_format(scalar: &IlScalar) -> String {
    format!("{}", scalar.x)
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

fn scalar_eq(lhs: &IlScalar, rhs: &IlScalar) -> bool {
    lhs.x == rhs.x
}


falcon_type_wrapper!(falcon::il::Expression, IlExpression);

impl<'vm> gluon::vm::api::Getable<'vm> for IlExpression {
    fn from_value(_vm: &'vm gluon::vm::thread::Thread,
                  value: gluon::vm::Variants)
        -> Self {

        match value.as_ref() {
            gluon::vm::api::ValueRef::Userdata(u) => {
                let i: &IlExpression =
                    u.downcast_ref::<IlExpression>().unwrap();
                i.clone()
            },
            _ => panic!("ValueRef is not a Userdata"),
        }
    }
}

fn expression_format(expression: &IlExpression) -> String {
    format!("{}", expression.x)
}

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

fn expression_ite(cond: &IlExpression, then: &IlExpression, else_: &IlExpression)
    -> IlExpression {
    IlExpression {
        x: falcon::il::Expression::ite(
            cond.x.clone(),
            then.x.clone(),
            else_.x.clone()
        ).unwrap()
    }
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
        falcon::il::Expression::Ite(_,_,_) => "ite"
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

fn expression_get_cond(expr: &IlExpression) -> IlExpression {
    match expr.x {
        falcon::il::Expression::Ite(ref cond, _, _) =>
            IlExpression { x: *cond.clone() },
        _ => panic!("expression_get_cond called over non-ite expression")
    }
}

fn expression_get_then(expr: &IlExpression) -> IlExpression {
    match expr.x {
        falcon::il::Expression::Ite(_, ref then, _) =>
            IlExpression { x: *then.clone() },
        _ => panic!("expression_get_then called over non-ite expression")
    }
}

fn expression_get_else(expr: &IlExpression) -> IlExpression {
    match expr.x {
        falcon::il::Expression::Ite(_, _, ref else_) =>
            IlExpression { x: *else_.clone() },
        _ => panic!("expression_get_else called over non-ite expression")
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


falcon_type_wrapper!(falcon::il::Intrinsic, IlIntrinsic);

fn intrinsic_mnemonic(intrinsic: &IlIntrinsic) -> String {
    intrinsic.x.mnemonic().to_string()
}

fn intrinsic_instruction_str(intrinsic: &IlIntrinsic) -> String {
    intrinsic.x.instruction_str().to_string()
}


falcon_type_wrapper!(falcon::il::Operation, IlOperation);

fn operation_format(operation: &IlOperation) -> String {
    format!("{}", operation.x)
}

fn operation_assign(dst: &IlScalar, src: &IlExpression) -> IlOperation {
    IlOperation { x: falcon::il::Operation::assign(dst.x.clone(), src.x.clone()) }
}

fn operation_store(index: &IlExpression, src: &IlExpression) -> IlOperation {
    IlOperation { x: falcon::il::Operation::store(index.x.clone(), src.x.clone()) }
}

fn operation_load(dst: &IlScalar, index: &IlExpression) -> IlOperation {
    IlOperation { x: falcon::il::Operation::load(dst.x.clone(), index.x.clone()) }
}

fn operation_branch(target: &IlExpression) -> IlOperation {
    IlOperation { x: falcon::il::Operation::branch(target.x.clone()) }
}

fn operation_type(operation: &IlOperation) -> String {
    match operation.x {
        falcon::il::Operation::Assign    { .. } => "assign",
        falcon::il::Operation::Store     { .. } => "store",
        falcon::il::Operation::Load      { .. } => "load",
        falcon::il::Operation::Branch    { .. } => "branch",
        falcon::il::Operation::Intrinsic { .. } => "intrinsic",
        falcon::il::Operation::Nop              => "nop"
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

fn operation_branch_target(operation: &IlOperation) -> IlExpression {
    match operation.x {
        falcon::il::Operation::Branch {ref target, ..} =>
            IlExpression { x: target.clone() },
        _ => panic!("operation_brc_target called on non-brc op")
    }
}

fn operation_intrinsic_intrinsic(operation: &IlOperation) -> IlIntrinsic {
    match operation.x {
        falcon::il::Operation::Intrinsic {ref intrinsic } =>
            IlIntrinsic { x: intrinsic.clone() },
        _ => panic!("operation_intrinsic_intrinsic called on non-intrinsic op")
    }
}

fn operation_str(operation: &IlOperation) -> String {
    format!("{}", operation.x)
}


falcon_type_wrapper!(falcon::il::Instruction, IlInstruction);

fn instruction_address(instruction: &IlInstruction) -> Option<u64> {
    instruction.x.address().clone()
}

fn instruction_format(instruction: &IlInstruction) -> String {
    format!("{}", instruction.x)
}

fn instruction_operation(instruction: &IlInstruction) -> IlOperation {
    IlOperation { x: instruction.x.operation().clone() }
}

fn instruction_index(instruction: &IlInstruction) -> usize {
    instruction.x.index()
}

fn instruction_str(instruction: &IlInstruction) -> String {
    format!("{}", instruction.x)
}


falcon_type_wrapper!(falcon::il::Block, IlBlock);

fn block_index(block: &IlBlock) -> usize {
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

fn block_store(block: &IlBlock, index: &IlExpression, src: &IlExpression) -> IlBlock {
    let mut block = block.clone();
    block.x.store(index.x.clone(), src.x.clone());
    block
}

fn block_load(block: &IlBlock, dst: &IlScalar, index: &IlExpression) -> IlBlock {
    let mut block = block.clone();
    block.x.load(dst.x.clone(), index.x.clone());
    block
}

fn block_branch(block: &IlBlock, target: &IlExpression) -> IlBlock {
    let mut block = block.clone();
    block.x.branch(target.x.clone());
    block
}

fn block_str(block: &IlBlock) -> String {
    format!("{}", block.x)
}


falcon_type_wrapper!(falcon::il::Edge, IlEdge);

fn edge_has_condition(edge: &IlEdge) -> bool {
    if let Some(_) = edge.x.condition() {
        true
    }
    else {
        false
    }
}

fn edge_condition(edge: &IlEdge) -> IlExpression {
    IlExpression { x: edge.x.condition().clone().unwrap().clone() }
}

fn edge_head(edge: &IlEdge) -> usize {
    edge.x.head()
}

fn edge_tail(edge: &IlEdge) -> usize {
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

fn control_flow_graph_edges(control_flow_graph: &IlControlFlowGraph)
    -> Vec<IlEdge> {

    control_flow_graph.x
                      .edges()
                      .iter()
                      .map(|e| IlEdge { x: (*e).clone() })
                      .collect::<Vec<IlEdge>>()
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

fn function_index(function: &IlFunction) -> Option<usize> {
    function.x.index()
}

fn function_name(function: &IlFunction) -> String {
    function.x.name()
}

fn function_address(function: &IlFunction) -> u64 {
    function.x.address()
}

fn function_blocks(function: &IlFunction) -> Vec<IlBlock> {
    function.x.blocks().iter().map(|b| IlBlock { x: (*b).clone() }).collect()
}

fn function_block(function: &IlFunction, index: usize) -> Option<IlBlock> {
    match function.x.block(index) {
        Some(block) => Some(IlBlock { x: block.clone() }),
        None => None
    }
}


falcon_type_wrapper!(falcon::il::Program, IlProgram);

// TODO: We pass a dummy argument to get around a gluon bug. Need this bug
// resolved
fn program_new(_: usize) -> IlProgram {
    IlProgram { x: falcon::il::Program::new() }
}

fn program_function_by_name(program: &IlProgram, name: &str) -> Option<IlFunction> {
    for function in program.x.functions() {
        if function.name() == name {
            return Some(IlFunction { x: function.clone() });
        }
    }
    None
}

fn program_functions(program: &IlProgram) -> Vec<IlFunction> {
    program.x
           .functions()
           .into_iter()
           .map(|f| IlFunction { x: f.clone() }).collect()
}

fn program_function_by_address(program: &IlProgram, address: u64)
    -> Option<IlFunction> {

    match program.x.function_by_address(address) {
        Some(function) => Some(IlFunction { x: function.clone() }),
        None => None
    }
}

fn program_add_function(program: &IlProgram, function: &IlFunction)
    -> IlProgram {
    
    println!("In program_add_function");

    let mut program = program.clone();
    program.x.add_function(function.x.clone());
    program
}


falcon_type_wrapper!(falcon::il::ProgramLocation, IlProgramLocation);

fn program_location_format(program_location: &IlProgramLocation) -> String {
    format!("{}", program_location.x)
}

fn program_location_from_address(program: &IlProgram, address: u64)
    -> Option<IlProgramLocation> {

    Some(IlProgramLocation {
        x: falcon::il::RefProgramLocation::from_address(&program.x, address)?.into()
    })
}

fn program_location_function_location(program_location: &IlProgramLocation) -> IlFunctionLocation {
    IlFunctionLocation {
        x: program_location.x.function_location().clone()
    }
}

fn program_location_new(function: &IlFunction, function_location: &IlFunctionLocation)
    -> IlProgramLocation {

    let pl = falcon::il::ProgramLocation::new(function.x.index(), function_location.x.clone());
    IlProgramLocation { x: pl }
}

fn program_location_instruction(
    program_location: &IlProgramLocation,
    program: &IlProgram
) -> Option<IlInstruction> {

    program_location.x.apply(&program.x).and_then(
        |ref_program_location| ref_program_location.instruction().map(
            |instruction| IlInstruction { x: instruction.clone() }))
}


falcon_type_wrapper!(falcon::il::FunctionLocation, IlFunctionLocation);

fn function_location_type(function_location: &IlFunctionLocation) -> String {
    match function_location.x {
        falcon::il::FunctionLocation::Instruction(_, _) => "instruction",
        falcon::il::FunctionLocation::Edge(_, _) => "edge",
        falcon::il::FunctionLocation::EmptyBlock(_) => "empty_block"
    }.to_string()
}

fn function_location_instruction_get(function_location: &IlFunctionLocation, function: &IlFunction)
-> Option<IlInstruction> {

    if let Some(ref_function_location) = function_location.x.apply(&function.x) {
        if let Some(instruction) = ref_function_location.instruction() {
            return Some(IlInstruction { x: instruction.clone() });
        }
    }
    None
}

fn function_location_edge_get(function_location: &IlFunctionLocation, function: &IlFunction)
-> Option<IlEdge> {

    if let Some(ref_function_location) = function_location.x.apply(&function.x) {
        if let Some(edge) = ref_function_location.edge() {
            return Some(IlEdge { x: edge.clone() });
        }
    }
    None
}

fn function_location_block_get(function_location: &IlFunctionLocation, function: &IlFunction)
-> Option<IlBlock> {

    if let Some(ref_function_location) = function_location.x.apply(&function.x) {
        if let Some(block) = ref_function_location.block() {
            return Some(IlBlock { x: block.clone() });
        }
    }
    None
}

fn function_location_instruction(block: &IlBlock, instruction: &IlInstruction)
-> IlFunctionLocation {
    let block = block.x.index();
    let instruction = instruction.x.index();
    let fl = falcon::il::FunctionLocation::Instruction(block, instruction);
    IlFunctionLocation { x: fl }
}

fn function_location_edge(edge: &IlEdge) -> IlFunctionLocation {
    let head = edge.x.head();
    let tail = edge.x.tail();
    IlFunctionLocation { x: falcon::il::FunctionLocation::Edge(head, tail) }
}

fn function_location_empty_block(block: &IlBlock) -> IlFunctionLocation {
    let fl = falcon::il::FunctionLocation::EmptyBlock(block.x.index());
    IlFunctionLocation { x : fl }
}


pub fn bindings (vm: gluon::RootedThread) -> gluon::RootedThread {

    vm.register_type::<IlConstant>("IlConstant", &[]).unwrap();
    vm.register_type::<IlScalar>("IlScalar", &[]).unwrap();
    vm.register_type::<IlExpression>("IlExpression", &[]).unwrap();
    vm.register_type::<IlIntrinsic>("IlIntrinsic", &[]).unwrap();
    vm.register_type::<IlOperation>("IlOperation", &[]).unwrap();
    vm.register_type::<IlInstruction>("IlInstruction", &[]).unwrap();
    vm.register_type::<IlBlock>("IlBlock", &[]).unwrap();
    vm.register_type::<IlEdge>("IlEdge", &[]).unwrap();
    vm.register_type::<IlControlFlowGraph>("IlControlFlowGraph", &[]).unwrap();
    vm.register_type::<IlFunction>("IlFunction", &[]).unwrap();
    vm.register_type::<IlProgram>("IlProgram", &[]).unwrap();
    vm.register_type::<IlProgramLocation>("IlProgramLocation", &[]).unwrap();
    vm.register_type::<IlFunctionLocation>("IlFunctionLocation", &[]).unwrap();

    fn falcon_il_prim_loader(vm: &gluon::Thread)
        -> gluon::vm::Result<gluon::vm::ExternModule> {
        
        gluon::vm::ExternModule::new(vm, record! {
            block_index => primitive!(1 block_index),
            block_instructions => primitive!(1 block_instructions),
            block_assign => primitive!(3 block_assign),
            block_store => primitive!(3 block_store),
            block_load => primitive!(3 block_load),
            block_branch => primitive!(2 block_branch),
            block_str => primitive!(1 block_str),
            constant_bits => primitive!(1 constant_bits),
            constant_eq => primitive!(2 constant_eq),
            constant_format => primitive!(1 constant_format),
            constant_new => primitive!(2 constant_new),
            constant_str => primitive!(1 constant_str),
            constant_value_u64 => primitive!(1 constant_value_u64),
            control_flow_graph_blocks => primitive!(1 control_flow_graph_blocks),
            control_flow_graph_dot_graph => primitive!(1 control_flow_graph_dot_graph),
            control_flow_graph_edges => primitive!(1 control_flow_graph_edges),
            control_flow_graph_str => primitive!(1 control_flow_graph_str),
            edge_has_condition => primitive!(1 edge_has_condition),
            edge_condition => primitive!(1 edge_condition),
            edge_head => primitive!(1 edge_head),
            edge_tail => primitive!(1 edge_tail),
            edge_str => primitive!(1 edge_str),
            expression_format => primitive!(1 expression_format),
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
            expression_ite => primitive!(3 expression_ite),
            expression_type => primitive!(1 expression_type),
            expression_get_scalar => primitive!(1 expression_get_scalar),
            expression_get_constant => primitive!(1 expression_get_constant),
            expression_get_lhs => primitive!(1 expression_get_lhs),
            expression_get_rhs => primitive!(1 expression_get_rhs),
            expression_get_cond => primitive!(1 expression_get_cond),
            expression_get_then => primitive!(1 expression_get_then),
            expression_get_else => primitive!(1 expression_get_else),
            expression_get_bits => primitive!(1 expression_get_bits),
            expression_str => primitive!(1 expression_str),
            function_address => primitive!(1 function_address),
            function_block => primitive!(2 function_block),
            function_blocks => primitive!(1 function_blocks),
            function_control_flow_graph => primitive!(1 function_control_flow_graph),
            function_index => primitive!(1 function_index),
            function_name => primitive!(1 function_name),
            function_location_type => primitive!(1 function_location_type),
            function_location_instruction => primitive!(2 function_location_instruction),
            function_location_edge => primitive!(1 function_location_edge),
            function_location_empty_block => primitive!(1 function_location_empty_block),
            function_location_instruction_get => primitive!(2 function_location_instruction_get),
            function_location_edge_get => primitive!(2 function_location_edge_get),
            function_location_block_get => primitive!(2 function_location_block_get),
            instruction_address => primitive!(1 instruction_address),
            instruction_format => primitive!(1 instruction_format),
            instruction_index => primitive!(1 instruction_index),
            instruction_operation => primitive!(1 instruction_operation),
            instruction_str => primitive!(1 instruction_str),
            intrinsic_mnemonic => primitive!(1 intrinsic_mnemonic),
            intrinsic_instruction_str => primitive!(1 intrinsic_instruction_str),
            operation_format => primitive!(1 operation_format),
            operation_assign => primitive!(2 operation_assign),
            operation_store => primitive!(2 operation_store),
            operation_load => primitive!(2 operation_load),
            operation_branch => primitive!(1 operation_branch),
            operation_type => primitive!(1 operation_type),
            operation_assign_src => primitive!(1 operation_assign_src),
            operation_assign_dst => primitive!(1 operation_assign_dst),
            operation_store_index => primitive!(1 operation_store_index),
            operation_store_src => primitive!(1 operation_store_src),
            operation_load_dst => primitive!(1 operation_load_dst),
            operation_load_index => primitive!(1 operation_load_index),
            operation_branch_target => primitive!(1 operation_branch_target),
            operation_intrinsic_intrinsic => primitive!(1 operation_intrinsic_intrinsic),
            operation_str => primitive!(1 operation_str),
            program_add_function => primitive!(2 program_add_function),
            program_function_by_address => primitive!(2 program_function_by_address),
            program_function_by_name => primitive!(2 program_function_by_name),
            program_functions => primitive!(1 program_functions),
            program_new => primitive!(1 program_new),
            program_location_format => primitive!(1 program_location_format),
            program_location_from_address => primitive!(2 program_location_from_address),
            program_location_function_location => primitive!(1 program_location_function_location),
            program_location_instruction => primitive!(2 program_location_instruction),
            program_location_new => primitive!(2 program_location_new),
            scalar_bits => primitive!(1 scalar_bits),
            scalar_eq => primitive!(2 scalar_eq),
            scalar_format => primitive!(1 scalar_format),
            scalar_name => primitive!(1 scalar_name),
            scalar_new => primitive!(2 scalar_new),
            scalar_str => primitive!(1 scalar_str)
        })
    }
    
    gluon::import::add_extern_module(&vm, "falcon_il_prim", falcon_il_prim_loader);

    vm
}