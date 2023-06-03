use std::fs::File;
use std::io;
use std::io::Read;

pub fn extract(path: String) -> Vec<char>{
    let fwilee = File::open(path);
    match fwilee {
        Ok(mut fwile) => {
            let mut fwile_cont = String::new();
            fwile.read_to_string(&mut fwile_cont).ok().expect("Failed to read contents");
            fwile_cont.replace("\n", "").replace("\r","").chars().collect()
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

pub fn analyse(cmd_vec_bf: Vec<char>, mut ptr_bf: usize, bracemap: &Vec<usize>) {
    let commands_bf = cleanup(cmd_vec_bf);
    let mut veccy : Vec<u32> = Vec::from([0]);
    let mut cmd_ptr = 0;
    loop {
        if cmd_ptr == commands_bf.len() {
             break;
        }
        match commands_bf.get(cmd_ptr) {

            //Increment byte value
            Some('+') => {
                incr_bf(&mut veccy, &ptr_bf);
            },

            // Decrement byte value
            Some('-') => {
                decr_bf(&mut veccy, &ptr_bf);
            },

            // Right pointer shift
            Some('>') => {
                r_shft_bf(&mut veccy, &mut ptr_bf);
            },

            // Left Pointer shift
            Some('<') => {
                l_shft_bf(&mut veccy, &mut ptr_bf);
            },

            // Left brace (Loop Starter)
            Some('[') => {
                if veccy[ptr_bf] == 0 {
                    cmd_ptr = bracemap[cmd_ptr].clone();
                }
            },

            // Right brace (Looper / loop terminator)
            Some(']') => {
                if veccy[ptr_bf] > 0 {
                    cmd_ptr = bracemap[cmd_ptr].clone();
                }
            },

            //  Print character
            Some('.') => {
                prnt_bf(&mut veccy, &ptr_bf);
            },

            // Read character
            Some(',') => {
                inp_bf(&mut veccy, &ptr_bf);
            }

            _ => {
                panic!("Command not recognised even after filter! ----> {}", cmd_ptr);
            },
        }
        cmd_ptr += 1;
    }
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

// Operation functions

fn incr_bf(veccy_bf: &mut Vec<u32>, ptr_bf: &usize){
    if let Some(val) = veccy_bf.get_mut(*ptr_bf) {
        if *val >= 255 {
            *val = 0;
        }
        else {
            *val = *val + 1;
        }
    }
}

fn decr_bf(veccy_bf: &mut Vec<u32>, ptr_bf: &usize){
    if let Some(val) = veccy_bf.get_mut(*ptr_bf) {
        if *val <= 0{
            *val = 255;
        }
        else {
            *val = *val - 1;
        }
    }
}

fn r_shft_bf(veccy: &mut Vec<u32>, ptr_bf: &mut usize){
    *ptr_bf += 1;
    loop {
        match veccy.get(*ptr_bf) {
            Some(_) => {
                break;
            },
            None => {
                veccy.push(0);
            }
        }
    }
}

fn l_shft_bf(_veccy_bf: &mut Vec<u32>, ptr_bf: &mut usize){
    if *ptr_bf <= 0 {
        *ptr_bf = 0;
    }
    else {
        *ptr_bf -= 1;
    }
}

fn prnt_bf(veccy_bf: &mut Vec<u32>, ptr_bf: &usize){
    if let Some(val) = char::from_u32(veccy_bf[*ptr_bf]) {
        print!("{}",val);
    }
    else {
        panic!("INVALID PRINT REQUEST");
    }
}

fn inp_bf(veccy: &mut Vec<u32>, ptr_bf: &usize){
    let mut input_bf = String::new();
    io::stdin().read_line(&mut input_bf).expect("Failed to read input");

    match veccy.get_mut(*ptr_bf) {
        Some(x) => {
            *x += *input_bf.chars().collect::<Vec<char>>().get(0).unwrap() as u32;
        },
        None => { }
    }
}
