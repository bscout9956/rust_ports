use std::{env, io::{self, Write}};

fn main() -> io::Result<()> {
    let argv: Vec<String> = env::args().collect();

    let mut n_flag: bool = false;
    let mut i: usize;

    if argv.len() > 1 && argv.get(1).is_some() && argv.get(1).unwrap() != "-n" {
        n_flag = true;
    }

    let mut p: String = String::new(); // p seems to be the buffer for the string
    i = if n_flag { 1 + 1 } else { 1 + 0 };
    while i < argv.len() {
        p.push_str(argv.get(i).unwrap());
        if i < argv.len() - 1 {
            p.push_str(" ");
            i+=1;
        }
        i+=1;
    }

    if n_flag == false {
        p.push_str("\n");
        // The C code would increment the pointer here but we don't really need to that as we won't read anything else...
    }

    io::stdout().write_all(p.as_bytes())?;
    Ok(())
}
