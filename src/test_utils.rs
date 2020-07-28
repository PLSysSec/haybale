use crate::backend::DefaultBackend;
use crate::{BBInstrIndex, Config, Location, Project, State};
use llvm_ir::*;
use std::collections::HashMap;

/// utility to initialize a `State` out of a `Project` and a function name
pub fn blank_state<'p>(project: &'p Project, funcname: &str) -> State<'p, DefaultBackend> {
    let (func, module) = project
        .get_func_by_name(funcname)
        .expect("Failed to find function");
    let start_loc = Location {
        module,
        func,
        bb: func
            .basic_blocks
            .get(0)
            .expect("Function must contain at least one basic block"),
        instr: BBInstrIndex::Instr(0),
        source_loc: None,
    };
    State::new(project, start_loc, Config::default())
}

/// Utility that creates a simple `Project` for testing.
/// The `Project` will contain a single `Module` (with the given name) which contains
/// a single function (given).
pub fn blank_project(modname: impl Into<String>, func: Function) -> Project {
    Project::from_module(Module {
        name: modname.into(),
        source_file_name: String::new(),
        data_layout: String::new(),
        target_triple: None,
        functions: vec![func],
        global_vars: vec![],
        global_aliases: vec![],
        named_struct_types: HashMap::new(),
        inline_assembly: String::new(),
    })
}

/// utility that creates a technically valid (but functionally useless)
/// `Function` for testing
///
/// the `Function` will contain technically valid (but functionally useless)
/// `BasicBlock`s, one per name provided in `bbnames`
pub fn blank_function(name: impl Into<String>, bbnames: Vec<Name>) -> Function {
    let mut func = Function::new(name);
    for bbname in bbnames {
        func.basic_blocks.push(BasicBlock::new(bbname));
    }
    func
}
