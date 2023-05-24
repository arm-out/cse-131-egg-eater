use sexp::Atom::*;
use sexp::*;

use lazy_static::lazy_static;
use std::collections::HashSet;

// General snek program structure
pub struct Program {
    pub defs: Vec<Definition>,
    pub main: Expr,
}

// (name (arg1 arg2 ...) body)
pub type Definition = (String, Vec<String>, Expr);

// Abstract syntax for unary operators
#[derive(Debug)]
pub enum Op1 {
    Add1,
    Sub1,
    IsNum,
    IsBool,
    Print,
}

// Abtract syntax for binary operators
#[derive(Debug)]
pub enum Op2 {
    Plus,
    Minus,
    Times,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

// Abstract Syntax Tree for the snek language.
#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Boolean(bool),
    Id(String),
    Tuple(Vec<Expr>),
    Nil(i64),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Set(String, Box<Expr>),
    Block(Vec<Expr>),
    Fun(String, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
}

// Create a static reference to a HashSet of reserved words
lazy_static! {
    pub static ref RESERVED_WORDS: HashSet<&'static str> = {
        let data = [
            "add1", "sub1", "isnum", "isbool", "let", "set!", "loop", "break", "block", "if",
            "print", "+", "-", "*", ">", "<", "=", "<=", ">=", "tuple", "index", "fun", "nil",
        ];
        let mut set = HashSet::new();
        for &s in &data {
            set.insert(s);
        }
        set
    };
}

// Parser entrypoint from file contents to Expr
pub fn parse_program(file_contents: &str) -> Program {
    let prog = &parse(&file_contents).unwrap_or_else(|_| {
        panic!("Invalid");
    });

    // Create Program struct
    match prog {
        Sexp::List(vec) => {
            let mut defs: Vec<Definition> = vec![];
            for def_or_exp in vec {
                if is_def(def_or_exp) {
                    defs.push(parse_definition(def_or_exp, &defs));
                } else {
                    // Reached main expression
                    return Program {
                        defs: defs,
                        main: parse_expr(def_or_exp),
                    };
                }
            }
            panic!("Only found definitions");
        }
        _ => panic!("Program should be a list"),
    }
}

// Check if Sexp is a function definition
fn is_def(s: &Sexp) -> bool {
    match s {
        Sexp::List(def_vec) => match &def_vec[..] {
            [Sexp::Atom(S(keyword)), Sexp::List(_), _] if keyword == "fun" => true,
            _ => false,
        },
        _ => false,
    }
}

// Chack that function call is not a reserved word
fn is_valid_call(target: &str) -> bool {
    if RESERVED_WORDS.contains(target) {
        false
    } else {
        true
    }
}

fn is_duplicate_fun(target: &str, defs: &Vec<Definition>) -> bool {
    for tuple in defs {
        if tuple.0.eq(target) {
            return true;
        }
    }
    false
}

// Parse definition Sexp to Expr
fn parse_definition(s: &Sexp, defs: &Vec<Definition>) -> Definition {
    match s {
        Sexp::List(def_vec) => match &def_vec[..] {
            [Sexp::Atom(S(keyword)), Sexp::List(arg_vec), body] if keyword == "fun" => {
                // Invalid function definition
                if arg_vec.len() == 0 {
                    panic!("Invalid: Function definition must be of the form (fun (args) body)")
                }

                let mut args: Vec<String> = vec![];

                // parse function name
                let name = match &arg_vec[0] {
                    Sexp::Atom(S(s))
                    // Check that function name is not a reserved word and is not a duplicate
                        if !RESERVED_WORDS.contains(&s.as_str()) && !is_duplicate_fun(s, defs) =>
                    {
                        s.clone()
                    }
                    _ => panic!("Invalid function name"),
                };

                // parse function args
                for arg in arg_vec.iter().skip(1) {
                    match arg {
                        Sexp::Atom(S(s))
                        // Check that function args are not reserved words and are not duplicates
                            if !RESERVED_WORDS.contains(&s.as_str())
                                && !args.contains(&s.to_string()) =>
                        {
                            args.push(s.clone())
                        }
                        _ => panic!("Invalid function args"),
                    }
                }

                // finally parse function body
                (name, args, parse_expr(body))
            }
            _ => panic!("Invalid: Function definition must be of the form (fun (args) body)"),
        },
        _ => panic!("INTERNAL PARSER ERROR"),
    }
}

// Parse main Sexp to Expr
fn parse_expr(s: &Sexp) -> Expr {
    match s {
        Sexp::Atom(I(n)) => parse_num(n),
        Sexp::Atom(S(b)) if b == "true" => Expr::Boolean(true),
        Sexp::Atom(S(b)) if b == "false" => Expr::Boolean(false),
        Sexp::Atom(S(n)) if n == "nil" => Expr::Nil(0),
        Sexp::Atom(S(s)) => parse_id(s),
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(constr)), args @ ..] if constr == "tuple" => parse_tuple(args),
            [Sexp::Atom(S(op)), e] if op == "add1" => parse_unop(Op1::Add1, e),
            [Sexp::Atom(S(op)), e] if op == "sub1" => parse_unop(Op1::Sub1, e),
            [Sexp::Atom(S(op)), e] if op == "isnum" => parse_unop(Op1::IsNum, e),
            [Sexp::Atom(S(op)), e] if op == "isbool" => parse_unop(Op1::IsBool, e),
            [Sexp::Atom(S(op)), e] if op == "print" => parse_unop(Op1::Print, e),
            [Sexp::Atom(S(op)), e1, e2] if op == "+" => parse_binop(Op2::Plus, e1, e2),
            [Sexp::Atom(S(op)), e1, e2] if op == "-" => parse_binop(Op2::Minus, e1, e2),
            [Sexp::Atom(S(op)), e1, e2] if op == "*" => parse_binop(Op2::Times, e1, e2),
            [Sexp::Atom(S(op)), e1, e2] if op == "=" => parse_binop(Op2::Equal, e1, e2),
            [Sexp::Atom(S(op)), e1, e2] if op == "<" => parse_binop(Op2::Less, e1, e2),
            [Sexp::Atom(S(op)), e1, e2] if op == "<=" => parse_binop(Op2::LessEqual, e1, e2),
            [Sexp::Atom(S(op)), e1, e2] if op == ">" => parse_binop(Op2::Greater, e1, e2),
            [Sexp::Atom(S(op)), e1, e2] if op == ">=" => parse_binop(Op2::GreaterEqual, e1, e2),
            [Sexp::Atom(S(op)), Sexp::List(binds), e] if op == "let" => parse_let(binds, e),
            [Sexp::Atom(S(op)), e1, e2, e3] if op == "if" => parse_if(e1, e2, e3),
            [Sexp::Atom(S(op)), e] if op == "loop" => parse_loop(e),
            [Sexp::Atom(S(op)), e] if op == "break" => parse_break(e),
            [Sexp::Atom(S(op)), Sexp::Atom(S(name)), e] if op == "set!" => parse_set(name, e),
            [Sexp::Atom(S(op)), exps @ ..] if op == "block" => parse_block(exps),
            [Sexp::Atom(S(fun_name)), args @ ..] if is_valid_call(fun_name) => {
                parse_call(fun_name, args)
            }
            [Sexp::Atom(S(op)), e1, e2] if op == "index" => parse_index(e1, e2),
            _ => panic!("Invalid"),
        },
        _ => panic!("Invalid"),
    }
}

// Parse Nummber and expect it to fit i63 format
fn parse_num(n: &i64) -> Expr {
    const I63_MAX: i64 = i64::MAX >> 1;
    const I63_MIN: i64 = i64::MIN >> 1;
    if *n > I63_MAX || *n < I63_MIN {
        panic!("Invalid: number out of range");
    }

    Expr::Number(i64::try_from(*n).unwrap())
}

// Parse Id and makes sure it is not a reserved word
fn parse_id(s: &str) -> Expr {
    if RESERVED_WORDS.contains(&s) {
        panic!("Invalid id {} overlaps with keyword", s);
    }
    Expr::Id(s.to_string())
}

// Parse unary operator expression into abstract syntax form
fn parse_unop(unop: Op1, e: &Sexp) -> Expr {
    Expr::UnOp(unop, Box::new(parse_expr(e)))
}

// Parse binary operator expression into abstract syntax form
fn parse_binop(binop: Op2, e1: &Sexp, e2: &Sexp) -> Expr {
    Expr::BinOp(binop, Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
}

// Parse let expressions into abstract syntax form
fn parse_let(binds: &Vec<Sexp>, e: &Sexp) -> Expr {
    if binds.len() == 0 {
        panic!("Invalid");
    }

    Expr::Let(
        binds.iter().map(|bind| parse_bind(bind)).collect(),
        Box::new(parse_expr(e)),
    )
}

// Parse let bindings into abstract syntax form
fn parse_bind(s: &Sexp) -> (String, Expr) {
    if let Sexp::List(vec) = s {
        match &vec[..] {
            [Sexp::Atom(S(name)), _]
                if RESERVED_WORDS.contains(name.as_str()) || name == "input" =>
            {
                panic!("binding overlaps with reserved keyword {}", name)
            }
            [Sexp::Atom(S(name)), e] => (name.to_string(), parse_expr(e)),
            _ => panic!("Invalid"),
        }
    } else {
        panic!("Invalid")
    }
}

// Parse if expression into abstract syntax format
fn parse_if(e1: &Sexp, e2: &Sexp, e3: &Sexp) -> Expr {
    Expr::If(
        Box::new(parse_expr(e1)),
        Box::new(parse_expr(e2)),
        Box::new(parse_expr(e3)),
    )
}

// Parse loop expression into abstract syntax format
fn parse_loop(e: &Sexp) -> Expr {
    Expr::Loop(Box::new(parse_expr(e)))
}

// Parse break expresion into abstract syntax format
fn parse_break(e: &Sexp) -> Expr {
    Expr::Break(Box::new(parse_expr(e)))
}

// Parse set expression into abstract syntax format
fn parse_set(name: &str, e: &Sexp) -> Expr {
    Expr::Set(name.to_string(), Box::new(parse_expr(e)))
}

// Parse block expression into abstract syntax format.
fn parse_block(exps: &[Sexp]) -> Expr {
    if exps.len() == 0 {
        panic!("Invalid: block expression must have at least one expression");
    }

    Expr::Block(exps.into_iter().map(|e| parse_expr(e)).collect())
}

// Parse function call expression into abstract syntax format
fn parse_call(fun_name: &str, args: &[Sexp]) -> Expr {
    Expr::Fun(
        fun_name.to_string(),
        args.iter().map(|arg| parse_expr(arg)).collect(),
    )
}

fn parse_tuple(args: &[Sexp]) -> Expr {
    Expr::Tuple(args.iter().map(|arg| parse_expr(arg)).collect())
}

fn parse_index(e1: &Sexp, e2: &Sexp) -> Expr {
    Expr::Index(Box::new(parse_expr(e1)), Box::new(parse_expr(e2)))
}
