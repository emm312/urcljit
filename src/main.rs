fn main() {
    #[cfg(feature = "ide")]
    urcljit::app::ide_main();

    #[cfg(feature = "cli")]
    urcljit::cli::cli_main();
}
