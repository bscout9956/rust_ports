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

    let mut echo_str: String = String::new();
    let start_index: usize = if n_flag { 1 } else { 0 };
    
    for str in &argv[start_index..] {
        echo_str.push_str(str);
        echo_str.push_str(" ");
    }

    if n_flag == false {
        echo_str.push_str("\n");
    }

    io::stdout().write_all(echo_str.as_bytes())?;
    Ok(())
}
