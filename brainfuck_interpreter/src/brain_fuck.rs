use std::io;
use crate::code_prep::cleanup;


pub fn analyse(cmd_vec_bf: Vec<char>, mut ptr_bf: usize, bracemap: &[usize]) {
    let commands_bf = cleanup(cmd_vec_bf);
    let mut veccy : Vec<u32> = Vec::from([0]);
    let mut cmd_ptr = 0;
    loop {
        if cmd_ptr == commands_bf.len() {
             break;
        }
        match commands_bf.get(cmd_ptr) {

            //Increment byte value
            Some('+') => incr_bf(&mut veccy, &ptr_bf),

            // Decrement byte value
            Some('-') => decr_bf(&mut veccy, &ptr_bf),


            // Right pointer shift
            Some('>') => r_shft_bf(&mut veccy, &mut ptr_bf),

            // Left Pointer shift
            Some('<') => l_shft_bf(&mut veccy, &mut ptr_bf),

            // Left brace (Loop Starter)
            Some('[') => {
                if veccy[ptr_bf] == 0 {
                    cmd_ptr = bracemap[cmd_ptr];
                }
            },

            // Right brace (Looper / loop terminator)
            Some(']') => {
                if veccy[ptr_bf] > 0 {
                    cmd_ptr = bracemap[cmd_ptr];
                }
            },

            //  Print character
            Some('.') => prnt_bf(&mut veccy, &ptr_bf),

            // Read character
            Some(',') => inp_bf(&mut veccy, &ptr_bf),

            _ => {
                match commands_bf.get(cmd_ptr) {
                    Some(_) => {
                        panic!("Command not recognised even after filter! \
                        Command index --> {} | Command --> {}", cmd_ptr, commands_bf[cmd_ptr]);
                    },
                    None => {
                        panic!("Command not recognised even after filter! Command index ----> {}", cmd_ptr);
                    }
                }
            },
        }
        cmd_ptr += 1;
    }
}

// Operation functions
// Legacy functions

fn incr_bf(veccy_bf: &mut [u32], ptr_bf: &usize){
    if let Some(val) = veccy_bf.get_mut(*ptr_bf) {
        if *val >= 255 {
            *val = 0;
        }
        else {
            *val += 1;
        }
    }
}

fn decr_bf(veccy_bf: &mut [u32], ptr_bf: &usize){
    if let Some(val) = veccy_bf.get_mut(*ptr_bf) {
        if *val == 0{
            *val = 255;
        }
        else {
            *val -= 1;
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

fn l_shft_bf(_veccy_bf: &mut [u32], ptr_bf: &mut usize){
    if *ptr_bf == 0 {
        *ptr_bf = 0;
    }
    else {
        *ptr_bf -= 1;
    }
}


fn prnt_bf(veccy_bf: &mut [u32], ptr_bf: &usize){
    if let Some(val) = char::from_u32(veccy_bf[*ptr_bf]) {
        print!("{}",val);
    }
    else {
        panic!("INVALID PRINT REQUEST");
    }
}

fn inp_bf(veccy: &mut [u32], ptr_bf: &usize){
    let mut input_bf = String::new();
    io::stdin().read_line(&mut input_bf).expect("Failed to read input");

    if let Some(x) = veccy.get_mut(*ptr_bf) {
        if let Some(y) = input_bf.chars().collect::<Vec<char>>().first() {
            *x += (*y) as u32;
        }
        else {
            panic!("Error occured! Faied to append user input to current memory slot");
        }
    }
    else{
        panic!("Error occured! Failed to retrieve element for read operation!");
    }
}

