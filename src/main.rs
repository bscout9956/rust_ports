use std::{
    env,
    io::{self, Write},
};

fn main() -> io::Result<()> {
    let argv: Vec<String> = env::args().skip(1).collect();

    let mut n_flag: bool = false;
    
    if !argv.is_empty() && argv.first().unwrap() == "-n" {
        n_flag = true;
    }

    let start_index: usize = usize::from(n_flag);
    
    let mut echo_str: String = argv[start_index..].join(" ");

    if !n_flag {
        echo_str.push('\n');
    }

    io::stdout().write_all(echo_str.as_bytes())?;
    Ok(())
}
