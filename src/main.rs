use std::{
    env,
    io::{self, Write},
};

fn main() -> io::Result<()> {
    let argv: Vec<String> = env::args().skip(1).collect();

    let mut n_flag: bool = false;
    
    if argv.len() > 0 && argv.get(0).is_some() && argv.get(0).unwrap() == "-n" {
        n_flag = true;
    }

    let start_index: usize = if n_flag { 1 } else { 0 };
    
    let mut echo_str= argv[start_index..].join(" ");

    if !n_flag {
        echo_str.push_str("\n");
    }

    io::stdout().write_all(echo_str.as_bytes())?;
    Ok(())
}
