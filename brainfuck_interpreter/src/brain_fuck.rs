use std::io;
use crate::code_prep::cleanup;


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
                match veccy.get_mut(ptr_bf) {
                    Some(x) => {
                        if *x >= 255 {
                            *x = 0;
                        } else {
                            *x += 1;
                        }
                    },
                    None => {
                        panic!("Error occured! Failed to retrieve element for increment operation!");
                    },
                }
            },

            // Decrement byte value
            Some('-') => {
                match veccy.get_mut(ptr_bf) {
                    Some(x) => {
                        if *x <= 0 {
                            *x = 255;
                        } else {
                            *x -= 1;
                        }
                    },
                    None => {}
                }
            },

            // Right pointer shift
            Some('>') => {
                ptr_bf += 1;
                loop {
                    match veccy.get(ptr_bf) {
                        Some(_) => {
                            break;
                        },
                        None => {
                            veccy.push(0);
                        }
                    }
                }
            },

            // Left Pointer shift
            Some('<') => {
                if ptr_bf <= 0 {
                    ptr_bf = 0;
                }
                else {
                    ptr_bf -= 1;
                }
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
                match veccy.get(ptr_bf) {
                    Some(x) => {
                        match char::from_u32(*x) {
                            Some(y) => {
                                print!("{}",y);
                            },
                            None => {
                                panic!("INVALID PRINT REQUEST!");
                            },
                        }
                    },
                    None => {
                        panic!("Error occured! Failed to retrieve element for print operation!");
                    },
                }
            },

            // Read character
            Some(',') => {
                let mut inp_bf = String::new();
                match io::stdin().read_line(&mut inp_bf) {
                    Ok(_) => {
                        match veccy.get_mut(ptr_bf) {
                            Some(x) => {
                                if let Some(y) = inp_bf.chars().collect::<Vec<char>>().get(0) {
                                    *x += (*y) as u32;
                                }
                                else {
                                    panic!("Error occured! Faied to append user input to current memory slot");
                                }
                            },
                            None => {
                                panic!("Error occured! Failed to retrieve element for read operation!");
                            },
                        }
                    },
                    Err(_) => {
                        panic!("Error occured! Failed to read user input");
                    },
                }
            },

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

/*
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
 */
