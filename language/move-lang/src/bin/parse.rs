use anyhow::Result;

use move_lang::{parser, strip_comments_and_verify};
use move_lang::errors::Errors;
use move_lang::parser::syntax::parse_file_string;

fn parse_module(
    src: &str,
    name: &'static str,
) -> Result<(Option<parser::ast::FileDefinition>, Errors)> {
    let mut errors: Errors = Vec::new();

    let no_comments_buffer = match strip_comments_and_verify(name, src) {
        Err(err) => {
            errors.push(err);
            return Ok((None, errors));
        }
        Ok(no_comments_buffer) => no_comments_buffer,
    };
    let def_opt = match parse_file_string(name, &no_comments_buffer) {
        Ok(def) => Some(def),
        Err(err) => {
            errors.push(err);
            None
        }
    };
    Ok((def_opt, errors))
}

fn main() {
    let sources = r#"
    module EarmarkedLibraCoin {
        use Wx0::LibraCoin;
    }
    "#;
    let parsed = parse_module(sources, "MyModule").unwrap();
    dbg!(parsed);
}
