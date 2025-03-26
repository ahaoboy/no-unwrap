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
    let mut p = code.as_str();

    let mut v: Vec<Result<()>> = vec![];
    let mut offset = 0;
    while let Some(index) = p.find(PAT) {
        v.push(Err(UnwrapError {
            src: NamedSource::new(path, code.to_owned()),
            at: SourceSpan::new((index + offset).into(), PAT.len()),
        }
        .into()));
        p = &p[index + PAT.len()..];
        offset += index + PAT.len();
    }

    v
}

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        println!("usage: no-unwrap <FILE>");
        return;
    };
    let errors = parse(&path);
    for e in errors {
        e.report();
    }
}
