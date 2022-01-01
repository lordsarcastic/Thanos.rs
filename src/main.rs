use structopt::StructOpt;

mod stones;
pub mod utils;
use stones::mind::mind;

#[derive(StructOpt)]
struct Cli {
    stone: String,

    #[structopt(parse(from_os_str))]
    directory: std::path::PathBuf,
}

fn validate_directory(path: &std::path::Path) -> Result<(), std::io::Error> {
    if !path.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Directory not found",
        ));
    }

    Ok(())
}

fn main() {
    println!("Hello, world!");
    let args = Cli::from_args();
    validate_directory(&args.directory).unwrap();
    mind(args.directory).unwrap();
    // let content = utils::get_directory_content(&args.directory).unwrap();
    // println!("{:?}", &content);
    // let files = content.get("dirs").unwrap();
    // shuffle_path_content(files.to_owned()).unwrap();
}
