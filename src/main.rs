use std::env;
use std::fs::File;
use std::io::prelude::*;

use eggeater::compiler;
use eggeater::parser;

// Project structure inspired by Compiler_01
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    // Create Program structure
    let prog = "(".to_owned() + &in_contents + ")";

    let program_expr = parser::parse_program(&prog);
    let program_asm = compiler::compile(&program_expr);

    let mut out_file = File::create(out_name)?;
    out_file.write_all(program_asm.as_bytes())?;

    Ok(())
}
