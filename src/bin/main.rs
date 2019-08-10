use haybale::*;
use std::path::Path;

fn main() {
    env_logger::init();
    // With 0 args, finds zeroes of all the functions in "basic"
    // With 1 arg (a file name), finds zeroes of all the functions in that file
    // With 2 args (file then function), finds zero of that function in that file
    let firstarg = std::env::args().nth(1);
    let secondarg = std::env::args().nth(2);
    let modname = firstarg.unwrap_or_else(|| "basic".to_owned());
    let filepath = Path::new("tests")
        .join(Path::new("bcfiles"))
        .join(Path::new(&modname))
        .with_extension("bc");
    let proj = Project::from_bc_path(&filepath).unwrap_or_else(|e| panic!("Failed to parse module at path {}: {}", filepath.display(), e));
    let functions: Box<Iterator<Item = String>>;
    if let Some(funcname) = secondarg {
        functions = Box::new(std::iter::once(funcname.clone()));
    } else {
        functions = Box::new(proj.all_functions().map(|f| f.name.clone()));
    }
    let ctx = z3::Context::new(&z3::Config::new());
    for funcname in functions {
        println!("Finding zero of function {:?}...", funcname);
        if let Some(args) = find_zero_of_func(&ctx, &funcname, &proj, Config::default()) {
            match args.len() {
                0 => println!("Function returns zero when passed no arguments\n"),
                1 => println!("Function returns zero when passed the argument {:?}\n", args[0]),
                _ => println!("Function returns zero when passed arguments {:?}\n", args),
            }
        } else {
            println!("Function never returns zero for any values of the arguments\n");
        }
    }
}
