use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg()]
    file: String,
}

pub fn main() {
    let file = Args::parse().file;
    let ast = urcljit::parser::parse(&std::fs::read_to_string(&file).unwrap(), &file);
    #[cfg(feature = "llvm")]
    urcljit::llvm::llvm_jit(ast.0, ast.1);

    urcljit::app::ide_main();
}
