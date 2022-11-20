mod game;
mod action;
mod vector_env;
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("c", "cuda", "enable CUDA");
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("s", "seed", "desired seed to the application (default: 0)", "SEED");
    opts.optopt("n", "num_envs", "number of environments to run (default: 4)", "NUM_ENVS");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("c") {
        tch::maybe_init_cuda();
        println!("Cuda available: {}", tch::Cuda::is_available());
    } else {
        println!("CUDA disabled");
    }
    let seed = matches.opt_str("s").unwrap_or("0".to_string()).parse::<i64>().unwrap();
    let num_envs = matches.opt_str("n").unwrap_or("4".to_string()).parse::<usize>().unwrap();
    tch::manual_seed(seed);
    game::start_game(seed, num_envs);
}
