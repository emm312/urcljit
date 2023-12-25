use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg()]
    file: String
}

pub fn cli_main() {
    let file = Args::parse().file;
    let ast = crate::parser::parse(&std::fs::read_to_string(file).unwrap());
    println!("{:#?}", ast);
}