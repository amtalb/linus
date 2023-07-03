use std::env;
use std::process;

use linus::Source;

use interpreter;
use lexer;
use parser;

fn main() {
    let source: Source = Source::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {err}");
        process::exit(1);
    });

    let lexed_source = lexer::lex(source.string).unwrap_or_else(|err| {
        eprintln!("Could not complete lexing\n{err}.");
        process::exit(1)
    });

    let ast = parser::parse(&lexed_source).unwrap_or_else(|err| {
        eprintln!("Could not complete parsing\n{err}");
        process::exit(1)
    });

    interpreter::interpret(&ast);
}
