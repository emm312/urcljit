use lalrpop_util::lalrpop_mod;

use crate::opcodes::Program;

lalrpop_mod!(pub urcl);

// TODO: errors
pub fn parse(code: &str) -> (Vec<Program>, (usize, usize, usize, usize)) {
    urcl::ProgParser::new().parse(code).unwrap()
}
