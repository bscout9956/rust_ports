use std::{env, io::{self, Write}};

fn main() -> io::Result<()> {
    let argv: Vec<String> = env::args().collect();

    let mut n_flag: bool = false;
    let mut i: usize;

    if argv.len() > 1 && argv.get(0).is_some() && argv.get(0).unwrap() == "-n" {
        n_flag = true;
    }

    let mut echo_str: String = String::new();
    let mut i: usize = if n_flag { 0 } else { 1 };
    while i <= argv.len() {
        echo_str.push_str(argv.get(i).unwrap());
        if i < argv.len() - 1 {
            echo_str.push_str(" ");
            i += 1;
        }
        i += 1;
    }

    if n_flag == false {
        echo_str.push_str("\n");
        // The C code would increment the pointer here but we don't really need to that as we won't read anything else...
    }

    io::stdout().write_all(echo_str.as_bytes())?;
    Ok(())
}
