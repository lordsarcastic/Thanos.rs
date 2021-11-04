use structopt::StructOpt;

#[derive(StructOpt)]
struct CLI {
    stone: String,

    #[structopt(parse(from_os_str))]
    directory: std::path::PathBuf
}



fn main() {
    println!("Hello, world!");
    let args = CLI::from_args();
    // println!("{}", first_pattern);
}
