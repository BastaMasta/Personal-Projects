use std::fs::File;
use std::io::Read;

pub fn extract(path: String) -> Vec<char>{
    let fwilee = File::open(path);
    match fwilee {
        Ok(mut fwile) => {
            let mut fwile_cont_1 = String::new();
            fwile.read_to_string(&mut fwile_cont_1).ok().expect("Failed to read contents");
            let fwile_cont_com : Vec<char> = fwile_cont_1.replace("\r","").chars().collect();
            let mut fwile_cont : Vec<char> = Vec::new();
            let mut com : bool = false;
            for i in fwile_cont_com.iter() {
                if *i == '/' {
                    com = true;
                    continue;
                }
                else if *i == '\n' {
                    com = false;
                    continue;
                }
                if !com {
                    fwile_cont.push(i.clone());
                }
            }
            fwile_cont
        },
        Err(_) => {
            panic!("Failed to open file!");
        },
    }
}

pub fn cleanup(code: Vec<char>) -> Vec<char>{
    let filter_arr = ['+','-','>','<','[',']','.',','];
    let cleaned_code :Vec<char> = code.into_iter()
        .filter(|x| filter_arr.contains(x))
        .collect();
    cleaned_code
}


pub fn build_bracemap(code: &Vec<char>) -> Vec<usize> {
    let mut temp_bracestack :Vec<usize> = Vec::new();
    let mut bracemap :Vec<usize> = vec![0;code.len()];
    for (position,command) in code.iter().enumerate(){
        if *command == '[' {
            temp_bracestack.push(position);
        }
        if *command == ']' {
            if let Some(start) = temp_bracestack.pop(){
                bracemap[start] = position;
                bracemap[position] = start;
            }
        }
    }
    bracemap
}

pub fn print_usage_and_exit() {
    println!("Usage:");
    println!("      Run command:");
    println!("          cargo run <optional --release tag> <filename with extension>");
    println!("Help (Shows this message) :");
    println!("          cargo run help");
    println!("if there was an error, then the program will most likely panic, but there is no");
    println!("need for you to panic, just run your code step-by-step through your head, or you");
    println!("can do it on paper or sumn, idk thta ususally helps me a lot");
    println!(" ");
    println!("this BrainFuck ineterpreter was made by Sameed Ahmed, enjoy!");
    std::process::exit(-1);
}
