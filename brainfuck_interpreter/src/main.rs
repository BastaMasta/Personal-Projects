use brainfuck_interpreter::brain_fuck::{analyse,
                                        build_bracemap,
                                        cleanup, extract,
                                        print_usage_and_exit};

#[allow(unused_assignments)]
fn main() {
    let mut code: Vec<char> = Vec::new();
    {
        let mut args: Vec<String> = std::env::args().skip(1).collect();
        if args.is_empty() {
            print_usage_and_exit();
        }
        let infile = args.remove(0);
        code = cleanup(extract(infile));
    }
    let bracemap: Vec<usize> = build_bracemap(&code);

    let ptr_bf :usize = 0;

    analyse(code, ptr_bf, &bracemap);

}
