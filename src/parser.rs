use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::instructions::{Destination, Instruction, Label, Source};

pub struct Parser {
    file: PathBuf,
    labels: HashMap<String, usize>,
}

impl Parser {
    pub fn new(file: PathBuf) -> Parser {
        Parser {
            file,
            labels: HashMap::new(),
        }
    }
    pub fn get_instructions(&mut self) -> Result<Vec<Instruction>, ParseError> {
        let file = match File::open(&self.file) {
            Ok(file) => file,
            Err(e) => {
                return Err(ParseError::new(
                    &format!("Failed to open file: {}", e),
                    None,
                ))
            }
        };

        let reader = BufReader::new(file);

        let mut instructions = Vec::new();

        for (line_idx, line_result) in reader.lines().enumerate() {
            let line = match line_result {
                Ok(line) => line,
                Err(e) => {
                    return Err(ParseError::new(
                        &format!("Failed to read line: {}", e),
                        Some(ParseErrorLineDetails {
                            line: line_idx,
                            contents: None,
                        }),
                    ))
                }
            };

            if self.line_is_non_functional(&line) {
                continue;
            }
            if self.line_is_label(&line) {
                let label = line.split(":").collect::<Vec<&str>>()[0];
                let instruction_id = instructions.len();
                self.labels.insert(label.to_string(), instruction_id);
                continue;
            }

            match self.parse_instruction(&line) {
                Ok(instruction) => instructions.push(instruction),
                Err(e) => return Err(e),
            }
        }

        self.resolve_labels(&mut instructions)?;

        return Ok(instructions);
    }

    fn parse_instruction(&mut self, line: &str) -> Result<Instruction, ParseError> {
        // TODO: Clean up
        let sanitized_line = if let Some(index) = line.find("//") {
            &line[..index].trim()
        } else {
            line.trim()
        };

        let chunks = sanitized_line.split_whitespace().collect::<Vec<&str>>();
        let instruction_id = chunks[0].to_uppercase();

        // Use a match expression for direct mapping
        let i = match instruction_id.as_str() {
            "RET" => Instruction::Return,
            "SET" => Instruction::Set(
                self.parse_source(chunks[1])?,
                self.parse_destination(chunks[2])?,
            ),
            "IN" => Instruction::Input(self.parse_destination(chunks[1])?),
            "OUT" => Instruction::Output(self.parse_source(chunks[1])?),
            "ADD" => Instruction::Add(
                self.parse_source(chunks[1])?,
                self.parse_source(chunks[2])?,
                self.parse_destination(chunks[3])?,
            ),
            "SUB" => Instruction::Subtract(
                self.parse_source(chunks[1])?,
                self.parse_source(chunks[2])?,
                self.parse_destination(chunks[3])?,
            ),
            "MUL" => Instruction::Multiply(
                self.parse_source(chunks[1])?,
                self.parse_source(chunks[2])?,
                self.parse_destination(chunks[3])?,
            ),
            "DIV" => Instruction::Divide(
                self.parse_source(chunks[1])?,
                self.parse_source(chunks[2])?,
                self.parse_destination(chunks[3])?,
            ),
            "MOD" => Instruction::Modulo(
                self.parse_source(chunks[1])?,
                self.parse_source(chunks[2])?,
                self.parse_destination(chunks[3])?,
            ),
            "JMP" => Instruction::Jump(self.parse_label(chunks[1])?),
            "JGT" => Instruction::JumpGreaterThan(
                self.parse_source(chunks[1])?,
                self.parse_source(chunks[2])?,
                self.parse_label(chunks[3])?,
            ),
            "JEQ" => Instruction::JumpEqual(
                self.parse_source(chunks[1])?,
                self.parse_source(chunks[2])?,
                self.parse_label(chunks[3])?,
            ),
            "JLT" => Instruction::JumpLessThan(
                self.parse_source(chunks[1])?,
                self.parse_source(chunks[2])?,
                self.parse_label(chunks[3])?,
            ),
            "CALL" => Instruction::Call(self.parse_label(chunks[1])?),
            "TIME" => Instruction::Time(self.parse_destination(chunks[1])?),
            "SYS" => Instruction::Syscall(
                self.parse_destination(chunks[1])?,
                self.parse_source(chunks[2])?,
                self.parse_optional_source(chunks.get(3))?,
                self.parse_optional_source(chunks.get(4))?,
                self.parse_optional_source(chunks.get(5))?,
                self.parse_optional_source(chunks.get(6))?,
                self.parse_optional_source(chunks.get(7))?,
                self.parse_optional_source(chunks.get(8))?,
            ),
            _ => {
                return Err(ParseError::new(
                    &format!("Unknown instruction: {}", instruction_id),
                    None,
                ))
            }
        };

        Ok(i)
    }

    fn parse_optional_source(&self, chunk: Option<&&str>) -> Result<Option<Source>, ParseError> {
        match chunk {
            None => Ok(None),
            Some(chunk) => {
                if *chunk == "_" {
                    return Ok(None);
                }

                let param = self.parse_source(chunk)?;
                Ok(Some(param))
            }
        }
    }

    fn parse_source(&self, chunk: &str) -> Result<Source, ParseError> {
        if chunk == "" {
            return Err(ParseError::new("Parameter should not be empty", None));
        }

        // Is value
        if chunk.starts_with("'") && chunk.ends_with("'") {
            let text = chunk[1..chunk.len() - 1].to_string();
            let value = text.parse::<i64>().unwrap();
            return Ok(Source::Data(value));
        }

        // Reference
        if chunk.starts_with("&") {
            let text = chunk[1..].to_string();
            let value = text.parse::<usize>().unwrap();
            return Ok(Source::Reference(value));
        }

        // Should be an address
        let text = chunk.to_string();
        let value = text.parse::<usize>().unwrap();
        Ok(Source::Address(value))
    }

    fn parse_destination(&self, chunk: &str) -> Result<Destination, ParseError> {
        if chunk == "" {
            return Err(ParseError::new("Parameter should not be empty", None));
        }

        // Reference
        if chunk.starts_with("&") {
            let text = chunk[1..].to_string();
            let value = text.parse::<usize>().unwrap();
            return Ok(Destination::Reference(value));
        }

        // Should be an address
        let text = chunk.to_string();
        let value = text.parse::<usize>().unwrap();
        Ok(Destination::Address(value))
    }

    fn parse_label(&mut self, chunk: &str) -> Result<Label, ParseError> {
        if chunk == "" {
            return Err(ParseError::new("Label should not be empty", None));
        }
        if chunk.starts_with("&") {
            return Err(ParseError::new(
                "Label should not start with an ampersand",
                None,
            ));
        }
        if chunk.ends_with(":") {
            return Err(ParseError::new("Label should not end with a colon", None));
        }
        if chunk.parse::<i32>().is_ok() {
            return Err(ParseError::new("Label should not be a number", None));
        }
        if chunk.starts_with("'") && chunk.ends_with("'") {
            return Err(ParseError::new("Label cannot be a value", None));
        }

        if let Some(address) = self.labels.get(chunk) {
            Ok(Label::Instruction(*address))
        } else {
            Ok(Label::Label(chunk.to_string()))
        }
    }

    fn resolve_labels(&mut self, instructions: &mut Vec<Instruction>) -> Result<(), ParseError> {
        // Resolve unresolved labels
        // TODO: Only loop over unresolved labels, and refactor
        for instruction in instructions.iter_mut() {
            match instruction {
                Instruction::Jump(label)
                | Instruction::JumpGreaterThan(_, _, label)
                | Instruction::JumpEqual(_, _, label)
                | Instruction::JumpLessThan(_, _, label)
                | Instruction::Call(label) => {
                    if let Label::Label(label_name) = label {
                        if let Some(label_id) = self.labels.get(label_name) {
                            *label = Label::Instruction(*label_id);
                        } else {
                            return Err(ParseError::new(
                                &format!("Unresolved label: {}", label_name),
                                None,
                            ));
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn line_is_non_functional(&self, line: &str) -> bool {
        if line == "" {
            return true;
        }

        for (i, char) in line.chars().enumerate() {
            if !char.is_whitespace() {
                if i == line.len() - 1 {
                    return true; // End of line, ergo empty
                }

                // Not empty, let's check if it's a comment
                if char == '/' && line.chars().nth(i + 1).unwrap() == '/' {
                    return true;
                }

                break;
            }
        }

        return false;
    }

    fn line_is_label(&self, line: &str) -> bool {
        for c in line.chars() {
            if c.is_whitespace() {
                return false;
            }
            if c == ':' {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug)]
pub struct ParseError {
    error: String,
    line: Option<ParseErrorLineDetails>,
}

#[derive(Debug)]
pub struct ParseErrorLineDetails {
    line: usize,
    contents: Option<String>,
}

impl ParseError {
    fn new(msg: &str, line: Option<ParseErrorLineDetails>) -> ParseError {
        ParseError {
            error: msg.to_string(),
            line,
        }
    }
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.line.is_some() {
            write!(
                f,
                "{}\nLine: {}\nContents: {}",
                self.error,
                self.line.as_ref().unwrap().line,
                self.line.as_ref().unwrap().contents.as_ref().unwrap()
            )
        } else {
            write!(f, "{}", self.error)
        }
    }
}
impl Error for ParseError {
    fn description(&self) -> &str {
        &self.error
    }
}
