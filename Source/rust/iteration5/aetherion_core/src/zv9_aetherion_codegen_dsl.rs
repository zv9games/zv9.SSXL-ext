#[allow(unused_imports)]
use aetherion_shared::zv9_prelude::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_while1},
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::{all_consuming, map},
    error::context,
    multi::{many0, separated_list1},
    sequence::{delimited, pair, terminated, tuple},
    IResult, Parser, // Parser trait for .parse(...)
};
use std::borrow::Cow;

// --- Tokens ---
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DslToken<'a> {
    Identifier(Cow<'a, str>),
    Number(i64),
    StringLiteral(Cow<'a, str>),
    Symbol(char),
    Keyword(Cow<'a, str>),
}

impl<'a> From<&'a str> for DslToken<'a> {
    fn from(s: &'a str) -> Self {
        DslToken::Identifier(Cow::Borrowed(s))
    }
}

/// Parsed command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslCommand<'a> {
    pub name: String,
    pub args: Vec<DslToken<'a>>,
    pub span: Option<(usize, usize)>,
}


/// Script
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslScript<'a> {
    pub commands: Vec<DslCommand<'a>>,
}

impl<'a> DslScript<'a> {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self { commands: Vec::with_capacity(cap) }
    }
    pub fn push_command(&mut self, cmd: DslCommand<'a>) {
        self.commands.push(cmd);
    }
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
    pub fn builder() -> DslScriptBuilder<'a> {
        DslScriptBuilder::new()
    }
}




/// Fluent builder
#[derive(Debug, Default)]
pub struct DslScriptBuilder<'a> {
    commands: Vec<DslCommand<'a>>,
}

impl<'a> DslScriptBuilder<'a> {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }
    pub fn command(mut self, name: impl Into<String>, args: Vec<DslToken<'a>>) -> Self {
        self.commands.push(DslCommand { name: name.into(), args, span: None });
        self
    }
    pub fn build(self) -> DslScript<'a> {
        DslScript { commands: self.commands }
    }
}

// --- Parser errors ---
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Syntax error at line {line}, col {col}: {msg}")]
    Syntax { line: usize, col: usize, msg: String },

    #[error("Unexpected EOF")]
    Eof,

    #[error("Nom error")]
    Nom,
}

impl<'a> From<nom::Err<nom::error::Error<&'a str>>> for ParseError {
    fn from(e: nom::Err<nom::error::Error<&'a str>>) -> Self {
        match e {
            nom::Err::Error(err) | nom::Err::Failure(err) => ParseError::Syntax {
                line: 1,
                col: 0,
                msg: format!("Nom error: {:?}", err.code),
            },
            nom::Err::Incomplete(_) => ParseError::Eof,
        }
    }
}

/// Top‑level parser (nom 8: use .parse)
pub fn parse_dsl(source: &str) -> Result<DslScript, ParseError> {
    let mut parser = all_consuming(terminated(
		separated_list1(tag(";"), parse_command),
		multispace0,
	));
	let (_, script) = parser.parse(source)?;


    // Promote to 'static
    let mut commands = Vec::with_capacity(script.len());
    for cmd in script {
        let owned_args = cmd
            .args
            .into_iter()
            .map(|t| match t {
                DslToken::Identifier(s) => DslToken::Identifier(Cow::Owned(s.into_owned())),
                DslToken::StringLiteral(s) => DslToken::StringLiteral(Cow::Owned(s.into_owned())),
                DslToken::Keyword(s) => DslToken::Keyword(Cow::Owned(s.into_owned())),
                _ => t,
            })
            .collect();
        commands.push(DslCommand { args: owned_args, ..cmd });
    }

    Ok(DslScript { commands })
}

/// One command parser
fn parse_command(input: &str) -> IResult<&str, DslCommand> {
    context(
        "command",
        tuple((
            terminated(take_while1(|c: char| c.is_alphabetic() || c == '_'), multispace0),
            many0(pair(
                terminated(take_while1(|c: char| c.is_alphanumeric() || c == '_'), char('=')),
                alt((
                    // Use explicit closure to avoid Cow being inferred as input type
                    map(
                        delimited(char('"'), take_till1(|c| c == '"'), char('"')),
                        |s: &str| DslToken::StringLiteral(Cow::Borrowed(s)),
                    ),
                    map(digit1, |s: &str| DslToken::Number(s.parse().unwrap())),
                    map(alpha1, DslToken::from),
                )),
            )),
        )),
    )
    .parse(input)
    .map(|(i, (name, arg_pairs))| {
        let args = arg_pairs
            .into_iter()
            .flat_map(|(k, v)| {
				vec![
					DslToken::Keyword(Cow::Owned(k.to_string())),
					v,
				]
			})

            .collect();
        (i, DslCommand { name: name.to_string(), args, span: None })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_cmd() {
        let src = "spawn_tile x=10 y=20";
        let script = parse_dsl(src).unwrap();
        assert_eq!(script.commands.len(), 1);
        assert_eq!(script.commands[0].name, "spawn_tile");
    }

    #[test]
    fn parse_multi_with_err() {
        let src = "spawn_tile x=10; invalid_cmd";
        let res = parse_dsl(src);
        assert!(res.is_err());
    }
}
