mod args;
use args::ClArgs;

fn main() {
    let args : ClArgs = argh::from_env();

    println!("Hello, world!");
    println!("{:#?}", args);
}
