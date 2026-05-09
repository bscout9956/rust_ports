#[allow(unused_imports)]
use std::{
    env,
    fs::{DirBuilder, exists},
    io::Error,
    process::exit,
};

fn usage() {
    println!("usage: mkdir [-p] [-m mode] dir...\n");
    exit(1);
}

#[allow(unused_variables)]
fn makedir(path: &str, mode: u32) -> Result<(), Error> {
    let does_file_exist: bool = exists(path)?;
    if does_file_exist {
        println!("mkdir: {} already exists\n", path);
    } else {
        std::fs::create_dir(path)?;
        // This only exists on unix. We sstill handle Windows just in case.
        // No idea about Mac lmao
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            let mut builder = DirBuilder::new();
            builder.mode(mode);
            builder.create(path)?;
        }
        #[cfg(windows)]
        {
            println!(
                "You're on Windows NT, the directory will be created with default permissions..."
            );
        }
    }
    return Ok(());
}

fn makedirp(path: &str, mode: u32) -> Result<(), Error> {
    let mut path_str: String = String::new();
    let splits: Vec<&str> = path.split("/").collect();

    for sub_str in splits {
        path_str.push('/');
        path_str.push_str(sub_str);

        if !exists(&path_str).unwrap_or(false) {
            makedir(&path_str, mode)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    // We skip the program name
    let argv: Vec<String> = env::args().skip(1).collect();

    let mut p_flag: bool = false;
    let mut mode: u32 = 0o777;

    let mut i: usize = 0;

    while i < argv.len() {
        let arg: &str = argv.get(i).unwrap();
        println!("Argument is {}.", arg);

        if arg.starts_with("-") && arg.len() > 1 && arg != "--" {
            let mut j: usize = 1;

            while j < arg.chars().count() {
                let flag: char = arg.chars().nth(j).unwrap();

                if flag == 'p' {
                    p_flag = true;
                }

                if flag == 'm' {
                    let mut mode_str: String = String::new();

                    // If the next character is alphanumeric
                    // Means we got glued e.g: -m777
                    if arg.chars().nth(j + 1).is_some() {
                        mode_str = arg[j + 1..].to_string();
                    } else {
                        match argv.get(i + 1).is_some() {
                            true => {
                                mode_str = argv.get(i + 1).unwrap().to_string();
                            }
                            false => {
                                usage();
                            }
                        }
                        i += 1;
                    }
                    // Transforming the char into an octal value stored in u32
                    let mode_res = u32::from_str_radix(mode_str.as_str(), 8);
                    if mode_res.is_ok() {
                        mode = u32::from_str_radix(mode_str.as_str(), 8).unwrap();
                        break; // We break, we found the mode
                    } else {
                        // Something went wrong, either the mode is invalid or something else...
                        usage();
                    }
                }
                j += 1;
            }
        } else if arg == "--" {
            i += 1;
            break;
        } else {
            break;
        }
        i += 1;
    }

    let directories: &[String] = &argv[i..];

    if !directories.is_empty() {
        println!("Directories isn't empty");
        for dir in directories {
            println!("Creating directory: {}.", dir);
            if p_flag {
                makedirp(dir, mode)?;
            } else {
                makedir(dir, mode)?;
            }
        }
    } else {
        usage();
    }

    Ok(())
}
