use miette::Result;
use miette::{Diagnostic, NamedSource, SourceSpan};
use std::process::Termination;
use thiserror::Error;

const PAT: &str = ".unwrap()";

#[derive(Error, Debug, Diagnostic)]
#[error("no unwrap!")]
pub struct UnwrapError {
    #[source_code]
    src: NamedSource<String>,

    #[label("here")]
    pub at: SourceSpan,
}

fn parse(path: &str) -> Vec<Result<()>> {
    let code = std::fs::read_to_string(path).unwrap();
    let mut offset = 0;
    let mut v: Vec<Result<()>> = vec![];
    while offset < code.len() {
        if code.as_bytes()[offset] == b'"' {
            offset += 1;
            while offset < code.len() && code.as_bytes()[offset] != b'"' {
                offset += 1;
            }
        }

        if code[offset..].starts_with("//") {
            while offset < code.len() && code.as_bytes()[offset] != b'\n' {
                offset += 1;
            }
        }
        if code[offset..].starts_with("/*") {
            while offset < code.len() && !code[offset..].starts_with("*/") {
                offset += 1;
            }
        }
        let p = &code[offset..];
        if p.starts_with(PAT) {
            v.push(Err(UnwrapError {
                src: NamedSource::new(path, code.to_owned()),
                at: SourceSpan::new((offset).into(), PAT.len()),
            }
            .into()));
            offset += PAT.len()
        }
        offset += 1;
    }

    v
}

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        println!("usage: no-unwrap <FILE>");
        return;
    };
    let errors = parse(&path);
    // report all .unwrap()
    /*  report all .unwrap() */
    for e in errors {
        e.report();
    }
}
