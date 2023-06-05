use image::DynamicImage;

#[allow(unused_variables)]
fn main(){

    let mut args: Vec < String > = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let infile = args.remove(0);
    let outfile = args.remove(0);

    let mut img = image::open(infile).expect("Failed to read infile");

    figure_lots(&mut args, &mut img, &outfile);

}

fn figure_lots(args: &mut Vec<String>, img: &mut DynamicImage, outfile: &String) //-> (Vec<String>, Vec<String>)
{
    let mut processes :Vec<String> = Vec::new();
    let mut arguments :Vec<String> = Vec::new();
    while args.len() > 0 {
        let t1 = args.remove(0);
        match t1.as_str() {
            "blur" => {
                arguments.push(args.remove(0));
                processes.push(t1.clone());
            }
            "brighten" => {
                arguments.push(args.remove(0));
                processes.push(t1.clone());
            }
            "crop" => {
                arguments.push(args.remove(0));
                arguments.push(args.remove(0));
                arguments.push(args.remove(0));
                arguments.push(args.remove(0));
                processes.push(t1.clone());
            }
            "rotate" => {
                arguments.push(args.remove(0));
                processes.push(t1.clone());
            }
            _ => {
                processes.push(t1.clone());
            }
        }
    }

    loop {
        if processes.len() > 0{
            do_processes(&mut processes, &mut arguments, img);
        }
        else {
            img.save(outfile).expect("Failed to write to outfile");
            break;
        }
    }
}

fn do_processes(param: &mut Vec<String>,args: &mut Vec<String>, img: &mut DynamicImage) {
    let t1 = param.remove(0);
    match t1.as_str() {
        "brighten" => {
            let amt :i32 = match args.remove(0).parse(){
                Ok(n) => n,
                Err(_) => {print_usage_and_exit(); 0}
            };
            *img = img.brighten(amt);
            println!("Image brightened by {}", amt);
        },

        "blur" => {
            let factor :f32  = match args.remove(0).parse(){
                Ok(n) => n,
                Err(_) => {print_usage_and_exit(); 0.0}
            };
            *img = img.blur(factor);
            println!("Image blurred by {}", factor);
        },

        "crop" => {
            let x :u32 = match args.remove(0).parse(){
                Ok(n) => n,
                Err(_) => {print_usage_and_exit(); 0}
            };
            let y :u32 = match args.remove(0).parse(){
                Ok(n) => n,
                Err(_) => {print_usage_and_exit(); 0}
            };
            let width :u32 = match args.remove(0).parse(){
                Ok(n) => n,
                Err(_) => {print_usage_and_exit(); 0}
            };
            let height :u32 = match args.remove(0).parse(){
                Ok(n) => n,
                Err(_) => {print_usage_and_exit(); 0}
            };
            *img = img.crop(x, y, width, height);
            println!("Image cropped to x:{} y:{} wodth:{} Height:{}", x,y,width,height);
        },

        "rotate" => {
            let amt_r :i32 = match args.remove(0).parse(){
                Ok(n) => n,
                Err(_) => {print_usage_and_exit(); 0}
            };
            *img = match amt_r {
                90 => img.rotate90(),
                180 => img.rotate180(),
                270 => img.rotate270(),
                _ => {
                    print_usage_and_exit();
                    img.clone()
                },
            };
            println!("Image rotated by {}", amt_r);
        },

        "invert" => {
            img.invert();
            println!("Image inverted");
        },

        "grayscale" => {
            *img = img.grayscale();
            println!("Image converted to Grayscale")
        },

        _ => {
            print_usage_and_exit();
        },
    }
}

fn print_usage_and_exit(){
    println!("An Error Occured!\nmake sure you are using the proper syntax for the program!");
    println!("Basic syntax: \n cargo run --release <infile> <outfile> <processes ans parameters>");
    println!("Processes and parameters:");
    println!("a.  blur <blur factor>");
    println!("b.  brighten <brighten factor>");
    println!("c.  crop <xOrigin> <yOrigin> <width> <height>");
    println!("d.  rotate <amount in degrees [90, 180 or 270]>");
    println!("e.  grayscale [no parameters]");
    println!("f.  invert [no parameters]");
    std::process::exit(-1);
}