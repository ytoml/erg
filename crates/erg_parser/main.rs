extern crate erg_common;
extern crate erg_parser;

use std::process;

use erg_common::config::{ErgConfig, ErgMode::*};
use erg_common::spawn::exec_new_thread;
use erg_common::traits::{ExitStatus, Runnable};

use erg_parser::build_ast::ASTBuilder;
use erg_parser::lex::LexerRunner;
use erg_parser::ParserRunner;

fn run() {
    let cfg = ErgConfig::parse();
    let stat = match cfg.mode {
        Lex => LexerRunner::run(cfg),
        Parse => ParserRunner::run(cfg),
        Desugar | Execute => ASTBuilder::run(cfg),
        other => {
            eprintln!("invalid mode: {other}");
            ExitStatus::ERR1
        }
    };
    process::exit(stat.code);
}

fn main() {
    exec_new_thread(run);
}
