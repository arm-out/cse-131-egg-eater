use crate::error;
use crate::parser::Definition;
use crate::parser::Expr;
use crate::parser::Op1;
use crate::parser::Op2;
use crate::parser::Program;
use crate::parser::RESERVED_WORDS;

use im::HashMap;
use std::collections::HashSet;

const WORD_SIZE: i32 = 8;
pub const ERROR_LABEL: &str = "throw_error";

#[derive(Debug, Clone)]
pub enum Val {
    Reg(Reg),
    Imm(i64),
    RegOffset(Reg, i32),
}

#[derive(Debug, Clone)]
pub enum Reg {
    RAX, // return value
    RBX, // temp register
    RSP, // stack pointer
    RDI, // "input" keyword
    R15, // curr heap location
}
use Reg::*;

#[derive(Debug, Clone)]
pub enum Instr {
    // Moves
    IMov(Val, Val),
    ICMove(Val, Val),
    ICMoveLess(Val, Val),
    ICMoveLessEq(Val, Val),
    ICMoveGreater(Val, Val),
    ICMoveGreaterEq(Val, Val),

    // Arithmetic
    IAdd(Val, Val),
    ISub(Val, Val),
    IMul(Val, Val),

    // Bitwise operations
    IAnd(Val, Val),
    IXor(Val, Val),
    IOr(Val, Val),
    IArithShiftRight(i32, Val),

    // Comparisons
    ICmp(Val, Val),
    ITest(Val, Val),

    // Jumps
    IJmp(String),
    IJmpEq(String),
    IJmpNotEq(String),
    IJmpOverflow(String),
    ILabel(String),

    // Call
    ICall(String),
}

// Assembly snippets that are used throughout the compiler
impl Instr {
    // Throws a dynamic error if the values stored in reg1 and reg2 are not
    // either both bools or both numbers
    fn test_same_type(reg1: Val, reg2: Val) -> Vec<Instr> {
        vec![
            Instr::IMov(Val::Reg(RBX), reg1),
            Instr::IXor(Val::Reg(RBX), reg2),
            Instr::ITest(Val::Reg(RBX), Val::Imm(1)),
            Instr::IMov(Val::Reg(RBX), Val::Imm(error::INVALID_ARGUMENT)),
            Instr::IJmpNotEq(ERROR_LABEL.to_string()),
        ]
    }

    // Throws a dynamic error if the value stored in reg is not a number
    fn test_num(reg: Val) -> Vec<Instr> {
        vec![
            Instr::IMov(Val::Reg(RBX), reg),
            Instr::IAnd(Val::Reg(RBX), Val::Imm(1)),
            Instr::ICmp(Val::Reg(RBX), Val::Imm(1)),
            Instr::IMov(Val::Reg(RBX), Val::Imm(error::INVALID_ARGUMENT)),
            Instr::IJmpEq(ERROR_LABEL.to_string()),
        ]
    }

    // - Throws a dynamic error if both reg values are not numbers
    fn test_both_nums(reg1: Val, reg2: Val) -> Vec<Instr> {
        vec![
            Instr::IMov(Val::Reg(RBX), reg1),
            Instr::IOr(Val::Reg(RBX), reg2),
            Instr::IAnd(Val::Reg(RBX), Val::Imm(1)),
            Instr::ICmp(Val::Reg(RBX), Val::Imm(1)),
            Instr::IMov(Val::Reg(RBX), Val::Imm(error::INVALID_ARGUMENT)),
            Instr::IJmpEq(ERROR_LABEL.to_string()),
        ]
    }

    // - Dynamic error if overflow bit is set
    fn test_overflow() -> Vec<Instr> {
        vec![
            Instr::IMov(Val::Reg(RBX), Val::Imm(error::OVERFLOW)),
            Instr::IJmpOverflow(ERROR_LABEL.to_string()),
        ]
    }

    // Evaluate comparison operators
    fn eval_comparison(op: &Op2, reg_left: Val, reg_right: Val) -> Vec<Instr> {
        vec![
            Instr::ICmp(reg_left, reg_right),
            Instr::IMov(Val::Reg(RBX), Val::Imm(to_bool63(true))),
            Instr::IMov(Val::Reg(RAX), Val::Imm(to_bool63(false))),
            match op {
                Op2::Equal => Instr::ICMove,
                Op2::Less => Instr::ICMoveLess,
                Op2::LessEqual => Instr::ICMoveLessEq,
                Op2::Greater => Instr::ICMoveGreater,
                Op2::GreaterEqual => Instr::ICMoveGreaterEq,
                Op2::Plus | Op2::Times | Op2::Minus => {
                    panic!("INTERNAL COMPILER ERROR: Invalid Op2 passed to eval_comparison")
                }
            }(Val::Reg(RAX), Val::Reg(RBX)),
        ]
    }
}

// Environment type
type Env = HashMap<String, i32>;
// Function Definition type
type FunDef = HashMap<String, i32>;

// Compile arguments
struct Context<'a> {
    si: i32,
    env: &'a Env,
    defs: &'a FunDef,
    is_def: bool,
    break_target: Option<String>,
}

// High level compile function entrypoint
pub fn compile(program: &Program) -> String {
    let defs = compile_defs(&program.defs);
    let main = compile_main(&program);

    format!(
        "section .text
global our_code_starts_here
extern snek_error
extern snek_print

{}:
    mov rdi, rbx
    push rsp
    and rsp, -16
    call snek_error
    ret

{}

our_code_starts_here:
    mov r15, rsi
{}
    ret
",
        ERROR_LABEL, defs, main
    )
}

// Recursively get depth to store temproary variables and let bindings
fn depth(e: &Expr) -> i32 {
    match e {
        Expr::Number(_) | Expr::Boolean(_) | Expr::Id(_) => 0,
        Expr::Let(bindings, body) => {
            let biggest_binding = bindings
                .iter()
                .map(|(_, expr)| depth(expr))
                .max()
                .unwrap_or(0);

            let bindings_depth = biggest_binding + bindings.len() as i32;
            let body_depth = depth(body);
            bindings_depth + body_depth + 1
        }
        Expr::UnOp(_, expr) => depth(expr),
        Expr::BinOp(_, left, right) => depth(left).max(depth(right) + 1),
        Expr::If(cond, then_expr, else_expr) => {
            depth(cond).max(depth(then_expr)).max(depth(else_expr))
        }
        Expr::Loop(expr) => depth(expr),
        Expr::Break(expr) => depth(expr),
        Expr::Set(_, expr) => depth(expr),
        Expr::Block(exprs) => exprs.iter().map(|expr| depth(expr)).max().unwrap_or(0),
        Expr::Fun(_, exprs) => {
            // Stack space for each of the arguments + depth of arg expressions
            exprs.iter().map(|expr| depth(expr)).max().unwrap_or(0) + exprs.len() as i32
        }
        Expr::Tuple(exprs) => {
            exprs.iter().map(|expr| depth(expr)).max().unwrap_or(0) + exprs.len() as i32
        }
    }
}

fn get_defs(defs: &Vec<Definition>) -> HashMap<String, i32> {
    let mut def_list = HashMap::new();
    for def in defs {
        def_list.insert(def.0.to_string(), def.1.len() as i32);
    }
    def_list
}

fn compile_defs(defs: &Vec<Definition>) -> String {
    let mut asm = String::new();
    for def in defs {
        match def {
            (name, args, body) => {
                // Check for duplicate arguments
                let mut arg_names = HashSet::new();
                for arg in args {
                    if arg_names.contains(arg) {
                        panic!("Duplicate argument {}", arg);
                    } else if RESERVED_WORDS.contains(&arg.as_str()) {
                        panic!("Cannot use reserved word {arg} as argument name");
                    }
                    arg_names.insert(arg);
                }

                let depth = depth(body);
                let offset = depth * WORD_SIZE;

                // Create function environment
                let mut body_env = HashMap::new();
                for (i, arg) in args.iter().enumerate() {
                    body_env.insert(arg.to_string(), (depth + 1 + i as i32) * 8);
                }

                let body_instrs = compile_to_instrs(
                    body,
                    &Context {
                        si: 0,
                        env: &body_env,
                        defs: &get_defs(defs),
                        is_def: true,
                        break_target: None,
                    },
                )
                .iter()
                .fold(String::new(), |accum, instr| {
                    accum + "    " + &instr_to_str(instr) + "\n"
                });

                asm.push_str(
                    format!(
                        "{name}:
    sub rsp, {offset}

{body_instrs}
    add rsp, {offset}
    ret

"
                    )
                    .as_str(),
                )
            }
        }
    }
    asm
}

// Compile main asm with offset from depth function
fn compile_main(program: &Program) -> String {
    let context = Context {
        si: 0,
        env: &HashMap::new(),
        defs: &get_defs(&program.defs),
        is_def: false,
        break_target: None,
    };

    let offset = depth(&program.main) * WORD_SIZE;
    let program_asm = compile_to_instrs(&program.main, &context)
        .iter()
        .fold(String::new(), |accum, instr| {
            accum + "    " + &instr_to_str(instr) + "\n"
        });

    format!(
        "    sub rsp, {offset}

{program_asm}
    add rsp, {offset}"
    )
}

// High level call which recursively compiles all of the subexpressions
// in `e`, with the specified `args`.
fn compile_to_instrs(e: &Expr, args: &Context) -> Vec<Instr> {
    match e {
        Expr::Number(n) => vec![Instr::IMov(Val::Reg(RAX), Val::Imm(to_num63(*n)))],
        Expr::Boolean(b) => vec![Instr::IMov(Val::Reg(RAX), Val::Imm(to_bool63(*b)))],
        Expr::Id(name) => compile_id(name, args),
        Expr::Tuple(exprs) => compile_tuple(exprs, args),
        Expr::UnOp(unop, e) => compile_unop(unop, e, args),
        Expr::BinOp(binop, e1, e2) => compile_binop(binop, e1, e2, args),
        Expr::Let(binds, body) => compile_let(binds, body, args),
        Expr::If(e1, e2, e3) => compile_if(e1, e2, e3, args),
        Expr::Loop(e) => compile_loop(e, args),
        Expr::Break(e) => compile_break(e, args),
        Expr::Set(name, e) => compile_set(name, e, args),
        Expr::Block(exps) => compile_block(exps, args),
        Expr::Fun(name, fun_args) => compile_fun(name, fun_args, args),
    }
}

// Helper function which panics if binding list has multiple vars with the same name
fn check_duplicate_binds(binds: &Vec<(String, Expr)>) {
    let mut set = HashSet::new();

    for (name, _) in binds {
        if set.contains(name) {
            panic!("Duplicate binding");
        }

        set.insert(name);
    }
}

// Applies the bindings to the env and compiles `body`
fn compile_let(binds: &Vec<(String, Expr)>, body: &Expr, args: &Context) -> Vec<Instr> {
    check_duplicate_binds(binds);

    let mut nenv = args.env.clone();
    let mut curr_si = args.si;
    [
        binds
            .iter()
            // (name: String, expr: Expr) -> (offset: i32, instrs: Vec<Instr>)
            // go from bindings to instructions and the stack offset on where to
            // store result of said instructions
            .map(|(name, expr)| {
                let curr_instrs = compile_to_instrs(
                    expr,
                    &Context {
                        si: curr_si,
                        env: &nenv,
                        break_target: args.break_target.clone(),
                        ..*args
                    },
                );
                let curr_offset = curr_si * WORD_SIZE;
                nenv = nenv.update(name.to_string(), curr_offset);
                curr_si += 1;
                (curr_offset, curr_instrs)
            })
            // (offset: i32, instrs: Vec<Instr>) -> Vec<Instr>
            // Combine instrs for expr with instruction to store result in designated
            // stack offset spot
            .map(|(offset, instrs)| {
                [
                    &instrs[..],
                    &[Instr::IMov(Val::RegOffset(RSP, offset), Val::Reg(RAX))],
                ]
                .concat()
            })
            // turn iterator of Vec<Instr> into one long vector, essentially concatenating
            // all of the separate Vec<Instr> we made from the previous map
            .fold(Vec::new(), |accum, instrs: Vec<Instr>| {
                [&accum[..], &instrs].concat()
            }),
        // Now that we have all the assembly instructions to calculate and store the bindings
        // on the stack, we can compile the instructions for the body expression while taking
        // into account the new environment and stack index.
        compile_to_instrs(
            body,
            &Context {
                si: curr_si,
                env: &nenv,
                break_target: args.break_target.clone(),
                ..*args
            },
        ),
    ]
    .concat()
}

// Compile assembly instructions for the operation `binop` `e1` `e2`
fn compile_binop(binop: &Op2, e1: &Expr, e2: &Expr, args: &Context) -> Vec<Instr> {
    // closure to manufacture the offset_reg as many times as we need without repeated boilerplate
    let offset_reg = || Val::RegOffset(RSP, args.si * WORD_SIZE);
    [
        // Compute first instr and store in RAX
        compile_to_instrs(e1, args),
        // Move RAX to [RSP - stack_offset]
        vec![Instr::IMov(offset_reg(), Val::Reg(RAX))],
        // Compute second instr and store in RAX, making sure to increase stack index
        compile_to_instrs(
            e2,
            &Context {
                si: args.si + 1,
                break_target: args.break_target.clone(),
                ..*args
            },
        ),
        // Do specific operation with [RSP - stack_offset] and RAX
        match binop {
            Op2::Plus => [
                Instr::test_both_nums(Val::Reg(RAX), offset_reg()),
                vec![Instr::IAdd(Val::Reg(RAX), offset_reg())],
                Instr::test_overflow(),
            ]
            .concat(),
            Op2::Minus => [
                Instr::test_both_nums(Val::Reg(RAX), offset_reg()),
                vec![
                    // Ensure that ordering is correct for subtraction
                    // offset_reg() is a temporary value, so it's okay
                    // to mangle it
                    Instr::ISub(offset_reg(), Val::Reg(RAX)),
                    Instr::IMov(Val::Reg(RAX), offset_reg()),
                ],
                Instr::test_overflow(),
            ]
            .concat(),
            Op2::Times => [
                Instr::test_both_nums(Val::Reg(RAX), offset_reg()),
                vec![
                    // Because numbers are left shifted by 1, need to invert
                    // this for one of the operand registers to make sure the
                    // answer is also only left shifted by 1
                    Instr::IArithShiftRight(1, Val::Reg(RAX)),
                    Instr::IMul(Val::Reg(RAX), offset_reg()),
                ],
                Instr::test_overflow(),
            ]
            .concat(),
            Op2::Equal => [
                Instr::test_same_type(Val::Reg(RAX), offset_reg()),
                Instr::eval_comparison(&Op2::Equal, offset_reg(), Val::Reg(RAX)),
            ]
            .concat(),
            Op2::Greater | Op2::GreaterEqual | Op2::Less | Op2::LessEqual => [
                Instr::test_both_nums(Val::Reg(RAX), offset_reg()),
                Instr::eval_comparison(binop, offset_reg(), Val::Reg(RAX)),
            ]
            .concat(),
        },
    ]
    .concat()
}

// Compile assembly instructions for operation `unop` `e`
fn compile_unop(unop: &Op1, e: &Expr, args: &Context) -> Vec<Instr> {
    [
        // Compile e and store in RAX
        compile_to_instrs(e, args),
        // Do operation on RAX
        match unop {
            Op1::Add1 => [
                Instr::test_num(Val::Reg(RAX)),
                vec![Instr::IAdd(Val::Reg(RAX), Val::Imm(to_num63(1)))],
                Instr::test_overflow(),
            ]
            .concat(),
            Op1::Sub1 => [
                Instr::test_num(Val::Reg(RAX)),
                vec![Instr::ISub(Val::Reg(RAX), Val::Imm(to_num63(1)))],
                Instr::test_overflow(),
            ]
            .concat(),
            Op1::IsBool => vec![
                // Check if the 2 rightmost bit is 11 (bool)
                Instr::IAnd(Val::Reg(RAX), Val::Imm(3)),
                Instr::ICmp(Val::Reg(RAX), Val::Imm(3)),
                Instr::IMov(Val::Reg(RBX), Val::Imm(to_bool63(true))),
                Instr::IMov(Val::Reg(RAX), Val::Imm(to_bool63(false))),
                Instr::ICMove(Val::Reg(RAX), Val::Reg(RBX)),
            ],
            Op1::IsNum => vec![
                // Check if rightmost bit is 0 (num)
                Instr::IAnd(Val::Reg(RAX), Val::Imm(1)),
                Instr::ICmp(Val::Reg(RAX), Val::Imm(0)),
                Instr::IMov(Val::Reg(RBX), Val::Imm(to_bool63(true))),
                Instr::IMov(Val::Reg(RAX), Val::Imm(to_bool63(false))),
                Instr::ICMove(Val::Reg(RAX), Val::Reg(RBX)),
            ],
            Op1::Print => {
                let offset = if args.si % 2 == 1 {
                    (args.si + 2) * WORD_SIZE
                } else {
                    (args.si + 1) * WORD_SIZE
                };

                vec![
                    // Align stack pointer to make room for RDI
                    Instr::ISub(Val::Reg(RSP), Val::Imm(offset as i64)),
                    Instr::IMov(Val::RegOffset(RSP, 0), Val::Reg(RDI)),
                    Instr::IMov(Val::Reg(RDI), Val::Reg(RAX)),
                    Instr::ICall("snek_print".to_string()),
                    Instr::IMov(Val::Reg(RDI), Val::RegOffset(RSP, 0)),
                    Instr::IAdd(Val::Reg(RSP), Val::Imm(offset as i64)),
                ]
            }
        },
    ]
    .concat()
}

// Compile instructions to get value stored in identifier `name`.
fn compile_id(name: &str, args: &Context) -> Vec<Instr> {
    vec![Instr::IMov(
        Val::Reg(RAX),
        if name == "input" {
            if args.is_def {
                panic!("Invalid: cannot use input in function definition")
            }
            Val::Reg(RDI)
        } else {
            Val::RegOffset(
                RSP,
                *args
                    .env
                    .get(name)
                    .expect(&format!("Unbound variable identifier {name}")),
            )
        },
    )]
}

// Compile instructions for if statement. If `cond_e` is not the 63 bit false value, run
// `then_e`. Otherwise, run `else_e`.
fn compile_if(cond_e: &Expr, then_e: &Expr, else_e: &Expr, args: &Context) -> Vec<Instr> {
    let end_label = unsafe { new_label("ifend") };
    let else_label = unsafe { new_label("ifelse") };

    [
        compile_to_instrs(cond_e, args),
        vec![
            Instr::ICmp(Val::Reg(RAX), Val::Imm(to_bool63(false))),
            Instr::IJmpEq(else_label.clone()),
        ],
        compile_to_instrs(then_e, args),
        vec![Instr::IJmp(end_label.clone()), Instr::ILabel(else_label)],
        compile_to_instrs(else_e, args),
        vec![Instr::ILabel(end_label)],
    ]
    .concat()
}

// Compile instructions for loop on expression `e`. Runs forever unless a break
// is called from within.
fn compile_loop(e: &Expr, args: &Context) -> Vec<Instr> {
    let loop_start = unsafe { new_label("loopstart") };
    let loop_end = unsafe { new_label("loopend") };

    [
        vec![Instr::ILabel(loop_start.clone())],
        // Set break target for any nested break statements
        compile_to_instrs(
            e,
            &Context {
                break_target: Some(loop_end.clone()),
                ..*args
            },
        ),
        vec![Instr::IJmp(loop_start), Instr::ILabel(loop_end)],
    ]
    .concat()
}

// Compile instructions for break statement. Essentially jumps to the current
// `break_target` defined in `args.env` and sets output of `e` in RAX.
fn compile_break(e: &Expr, args: &Context) -> Vec<Instr> {
    match &args.break_target {
        None => panic!("break statement found outside of a loop"),
        Some(target) => [
            compile_to_instrs(e, args),
            vec![Instr::IJmp(target.to_string())],
        ]
        .concat(),
    }
}

// Compile instructions for block statement
fn compile_block(exprs: &Vec<Expr>, args: &Context) -> Vec<Instr> {
    exprs
        .into_iter()
        .map(|expr| compile_to_instrs(expr, args))
        .flatten()
        .collect()
}

// Compile instructions for set! statement. Looks in `args.env` for a variable
// with name `name` and sets value of `e` to it.
fn compile_set(name: &str, e: &Expr, args: &Context) -> Vec<Instr> {
    match args.env.get(name) {
        None => panic!("Unbound variable identifier {name}"),
        Some(offset) => [
            compile_to_instrs(e, args),
            vec![Instr::IMov(Val::RegOffset(RSP, *offset), Val::Reg(RAX))],
        ]
        .concat(),
    }
}

// Compile Function call.
// Calling Convention: C/Rust style
fn compile_fun(name: &str, args: &Vec<Expr>, ctx: &Context) -> Vec<Instr> {
    let mut nenv = ctx.env.clone();
    let mut curr_si = ctx.si;

    let rsp_offset = (args.len() as i32 + 1) * WORD_SIZE;
    let arg_offset = args.len() as i32 + 1 + ctx.si;
    let rdi_pos = rsp_offset - WORD_SIZE;

    // Check if function exists in environment
    if !ctx.defs.contains_key(name) {
        panic!("Invalid function {name}");
    }

    // Check if function is a RESERVED WORD
    if RESERVED_WORDS.contains(&name) {
        panic!("Cannot use reserved word {name} as function name");
    }

    // Check number of args
    if args.len() as i32 != ctx.defs[name] {
        panic!(
            "Invalid number of arguments for function {name}: expected {}, got {}",
            ctx.defs[name],
            args.len()
        );
    }

    [
        // Push arguments onto temp stack variable
        args.iter()
            // (expr: Expr) -> (offset: i32, instrs: Vec<Instr>)
            .map(|expr| {
                let curr_instrs = compile_to_instrs(
                    expr,
                    &Context {
                        si: curr_si,
                        env: &nenv,
                        break_target: ctx.break_target.clone(),
                        ..*ctx
                    },
                );
                let curr_offset = curr_si * WORD_SIZE;
                nenv = nenv.update(name.to_string(), curr_offset);
                curr_si += 1;
                (curr_offset, curr_instrs)
            })
            // (offset: i32, instrs: Vec<Instr>) -> Vec<Instr>
            // Combine instrs for expr with instruction to store result in designated
            // stack offset spot
            .map(|(offset, instrs)| {
                [
                    &instrs[..],
                    &[Instr::IMov(Val::RegOffset(RSP, offset), Val::Reg(RAX))],
                ]
                .concat()
            })
            // turn iterator of Vec<Instr> into one long vector, essentially concatenating
            // all of the separate Vec<Instr> we made from the previous map
            .fold(Vec::new(), |accum, instrs: Vec<Instr>| {
                [&accum[..], &instrs].concat()
            }),
        // Move RSP to make room for args and RDI
        vec![Instr::ISub(Val::Reg(RSP), Val::Imm(rsp_offset as i64))],
        // Move args from temp stack variable to arg positions acc to calling conv
        args.iter()
            .enumerate()
            .map(|(i, _)| {
                vec![
                    Instr::IMov(
                        Val::Reg(RBX),
                        Val::RegOffset(RSP, (arg_offset + i as i32) * WORD_SIZE),
                    ),
                    Instr::IMov(Val::RegOffset(RSP, i as i32 * WORD_SIZE), Val::Reg(RBX)),
                ]
            })
            .flatten()
            .collect(),
        // Save RDI
        vec![Instr::IMov(Val::RegOffset(RSP, rdi_pos), Val::Reg(RDI))],
        // Call function
        vec![Instr::ICall(name.to_string())],
        // Restore RDI
        vec![Instr::IMov(Val::Reg(RDI), Val::RegOffset(RSP, rdi_pos))],
        // Restore RSP
        vec![Instr::IAdd(Val::Reg(RSP), Val::Imm(rsp_offset as i64))],
    ]
    .concat()
}

// Compile intructions for tuple allocation
fn compile_tuple(args: &Vec<Expr>, ctx: &Context) -> Vec<Instr> {
    let mut curr_si = ctx.si;
    let size = args.len() as i32;

    let mut instrs = args
        .iter()
        // args: Expr -> (offset: i32, instrs: Vec<Instr>)
        .map(|expr| {
            let curr_instrs = compile_to_instrs(
                expr,
                &Context {
                    si: curr_si,
                    env: &ctx.env.clone(),
                    break_target: ctx.break_target.clone(),
                    ..*ctx
                },
            );
            let curr_offset = curr_si * WORD_SIZE;
            curr_si += 1;
            (curr_offset, curr_instrs)
        })
        // (offset: i32, instrs: Vec<Instr>) -> Vec<Instr>
        // Combine instrs for expr with instruction to store result in designated
        // heap location
        .map(|(offset, instrs)| {
            [
                &instrs[..],
                &[Instr::IMov(Val::RegOffset(RSP, offset), Val::Reg(RAX))],
            ]
            .concat()
        })
        // turn iterator of Vec<Instr> into one long vector, essentially concatenating
        // all of the separate Vec<Instr> we made from the previous map
        .fold(Vec::new(), |accum, instrs: Vec<Instr>| {
            [&accum[..], &instrs].concat()
        });

    let mut heap_offset = size;
    // move stack values onto heap
    for i in (ctx.si..curr_si).rev() {
        // Move value to RAX
        instrs.push(Instr::IMov(
            Val::Reg(RAX),
            Val::RegOffset(RSP, i * WORD_SIZE),
        ));
        // Store in corresponding heap location
        instrs.push(Instr::IMov(
            Val::RegOffset(R15, heap_offset * WORD_SIZE),
            Val::Reg(RAX),
        ));
        heap_offset -= 1;
    }

    // Store size of tuple in first heap location
    instrs.push(Instr::IMov(Val::Reg(RAX), Val::Imm(to_num63(size as i64))));
    instrs.push(Instr::IMov(Val::RegOffset(R15, 0), Val::Reg(RAX)));

    // Return R15 to RAX
    instrs.push(Instr::IMov(Val::Reg(RAX), Val::Reg(R15)));
    // TAG RAX
    instrs.push(Instr::IAdd(Val::Reg(RAX), Val::Imm(1)));
    // Increment R15
    instrs.push(Instr::IAdd(
        Val::Reg(R15),
        Val::Imm(((size + 1) * WORD_SIZE) as i64),
    ));

    instrs
}

// Convert abstract assembly instruction into concrete X86_64 String representation
fn instr_to_str(i: &Instr) -> String {
    match i {
        Instr::IMov(dest, src) => format!("mov {}, {}", val_to_str(dest), val_to_str(src)),
        Instr::IAdd(dest, src) => format!("add {}, {}", val_to_str(dest), val_to_str(src)),
        Instr::ISub(dest, src) => format!("sub {}, {}", val_to_str(dest), val_to_str(src)),
        Instr::IMul(dest, src) => format!("imul {}, {}", val_to_str(dest), val_to_str(src)),
        Instr::ICmp(left, right) => format!("cmp {}, {}", val_to_str(left), val_to_str(right)),
        Instr::IJmp(label) => format!("jmp {label}"),
        Instr::IJmpEq(label) => format!("je {label}"),
        Instr::IJmpNotEq(label) => format!("jne {label}"),
        Instr::IAnd(dest, src) => format!("and {}, {}", val_to_str(dest), val_to_str(src)),
        Instr::ILabel(label) => format!("{label}:"),
        Instr::ICMove(dest, src) => format!("cmove {}, {}", val_to_str(dest), val_to_str(src)),
        Instr::ITest(dest, src) => format!("test {}, {}", val_to_str(dest), val_to_str(src)),
        Instr::IXor(dest, src) => format!("xor {}, {}", val_to_str(dest), val_to_str(src)),
        Instr::IOr(dest, src) => format!("or {}, {}", val_to_str(dest), val_to_str(src)),
        Instr::ICMoveLess(dest, src) => format!("cmovl {}, {}", val_to_str(dest), val_to_str(src)),
        Instr::ICMoveLessEq(dest, src) => {
            format!("cmovle {}, {}", val_to_str(dest), val_to_str(src))
        }
        Instr::ICMoveGreater(dest, src) => {
            format!("cmovg {}, {}", val_to_str(dest), val_to_str(src))
        }
        Instr::ICMoveGreaterEq(dest, src) => {
            format!("cmovge {}, {}", val_to_str(dest), val_to_str(src))
        }
        Instr::IArithShiftRight(count, dest) => format!("sar {}, {}", val_to_str(dest), count),
        Instr::IJmpOverflow(label) => format!("jo {label}"),
        Instr::ICall(label) => format!("call {label}"),
    }
}

// Converts abstract assembly value to X86_64 String representation
fn val_to_str(v: &Val) -> String {
    match v {
        Val::Reg(reg) => reg_to_str(reg),
        Val::RegOffset(reg, n) => format!("[{} + {n}]", reg_to_str(reg)),
        Val::Imm(n) => n.to_string(),
    }
}

/// Converts abstract assembly register to X86_64 String representation
fn reg_to_str(reg: &Reg) -> String {
    match reg {
        RAX => "rax".to_string(),
        RSP => "rsp".to_string(),
        RDI => "rdi".to_string(),
        RBX => "rbx".to_string(),
        R15 => "r15".to_string(),
    }
}

// Converts number into assembly 63 bit format, where rightmost bit is type bit.
// Numbers have a type bit of 0.
fn to_num63(n: i64) -> i64 {
    n << 1
}

// Converts bool into assembly 63 bit format, where rightmost bit is type bit.
// Bools have a type bit of 1, so 0b11 is false and 0b111 is true.
fn to_bool63(b: bool) -> i64 {
    match b {
        true => 7,  // 0b111
        false => 3, // 0b011
    }
}

unsafe fn new_label(s: &str) -> String {
    static mut CURR_LABEL_NUM: u64 = 0;
    CURR_LABEL_NUM += 1;
    format!("{s}{CURR_LABEL_NUM}")
}
