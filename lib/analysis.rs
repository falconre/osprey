use falcon;
use gluon::vm::api::{Userdata, VmType};
use gluon::vm::thread::{Traverseable};
use gluon;

use il;


fn dead_code_elimination(function: &il::IlFunction) -> il::IlFunction {
    let function = falcon::analysis::dead_code_elimination(&function.x).unwrap();
    il::IlFunction { x: function }
}


falcon_type_wrapper!(falcon::analysis::constants::Constants, AnalysisConstants);

fn constants_analysis(function: &il::IlFunction)
    -> Vec<(il::IlFunctionLocation, AnalysisConstants)> {

    falcon::analysis::constants::constants(&function.x).unwrap()
        .into_iter()
        .map(|(rpl, constants)| (
            il::IlFunctionLocation{ x: rpl.function_location().clone().into() },
            AnalysisConstants { x: constants }
        ))
        .collect()
}

fn constants_scalar(constants: &AnalysisConstants, scalar: &il::IlScalar)
    -> Option<il::IlConstant> {

    constants.x.scalar(&scalar.x)
        .map(|constant| il::IlConstant{x: constant.clone()})
}

fn constants_eval(constants: &AnalysisConstants, expression: &il::IlExpression)
    -> Option<il::IlConstant> {

    constants.x.eval(&expression.x)
        .map(|constant| il::IlConstant{x: constant.clone()})
}


pub fn bindings(vm: gluon::RootedThread) -> gluon::RootedThread {

    vm.register_type::<AnalysisConstants>("AnalysisConstants", &[]).unwrap();

    fn falcon_loader_prim_loader(vm: &gluon::Thread)
        -> gluon::vm::Result<gluon::vm::ExternModule> {
        
        gluon::vm::ExternModule::new(vm, record! {
            dead_code_elimination => primitive!(1, dead_code_elimination),
            constants_analysis => primitive!(1, constants_analysis),
            constants_eval => primitive!(2, constants_eval),
            constants_scalar => primitive!(2, constants_scalar)
        })
    }
    
    gluon::import::add_extern_module(
        &vm, "falcon_analysis_prim", falcon_loader_prim_loader);

    vm
}