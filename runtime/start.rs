use std::convert::TryInto;
use std::env;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64, memory: *mut u64) -> u64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i64) {
    eprintln!("{}", get_err_msg(errcode));
    std::process::exit(1);
}

#[no_mangle]
#[export_name = "\x01snek_print"]
pub extern "C" fn snek_print(val: i64) -> i64 {
    println!("{}", bytes63_to_str(val));
    return val;
}

fn get_err_msg(code: i64) -> String {
    match code {
        7 => format!("invalid argument"),
        8 => format!("overflow"),
        9 => format!("out of bounds"),
        _ => format!("Error {code}: (Unknown Error Code?!)"),
    }
}

fn parse_input(input: &str) -> i64 {
    match input {
        "true" => 7,
        "false" => 3,
        _ => ((input.parse::<i64>().unwrap_or_else(|_| panic!("Invalid"))) << 1)
            .try_into()
            .unwrap(),
    }
}

fn bytes63_to_str(val: i64) -> String {
    match val {
        1 => "nil".to_string(),
        3 => "false".to_string(),
        7 => "true".to_string(),
        n if n % 2 == 0 => (n >> 1).to_string(),
        // TODO: tuple representation
        n if n & 1 == 1 => {
            let addr = (n - 1) as *const i64;
            let size = unsafe { *addr } >> 1;
            let mut val_str = vec![];
            for i in 1..=size {
                val_str.push(unsafe { bytes63_to_str(*addr.offset(i as isize)) });
            }
            let formatted_string = val_str.join(", ");
            format!("[{}]", formatted_string)
        }
        n => panic!("Invalid 63 bit representation: {}", n),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let mut memory = Vec::with_capacity(100000);
    let buffer: *mut u64 = memory.as_mut_ptr();

    let i: u64 = unsafe { our_code_starts_here(input as u64, buffer) };
    println!("{}", bytes63_to_str(i as i64));
}
