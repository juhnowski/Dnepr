pub mod error;
mod parser;

pub use error::AsmError;
use parser::AsmParser;

use crate::types::{WORD_MASK, DneprWord};
use std::collections::HashMap;

pub struct Assembler;

impl Assembler {
    /// Компиляция исходного текста программы в машинный код
    pub fn compile(source: &str) -> Result<Vec<u32>, AsmError> {
        let (labels, defines) = AsmParser::parse_labels_and_defines(source)?;
        Self::generate_binary(source, &labels, &defines)
    }

    /// ВТОРОЙ ПРОХОД: Генерация бинарных инструкций
    fn generate_binary(source: &str, labels: &HashMap<String, u32>, defines: &HashMap<String, u32>) -> Result<Vec<u32>, AsmError> {
        let mut binary = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            let mut tokens = AsmParser::tokenize(line);
            if tokens.is_empty() || tokens[0].to_uppercase() == "DEFINE" { continue; }

            if tokens[0].ends_with(':') {
                tokens.remove(0);
                if tokens.is_empty() { continue; }
            }

            let mnemonic = tokens[0].to_uppercase();

            if mnemonic == "DATA" {
                if tokens.len() < 2 {
                    return Err(AsmError::MissingArgument(format!(
                        "Строка {}: Директива DATA требует числовое значение", line_num + 1
                    )));
                }
                let val_f64 = tokens[1].parse::<f64>().map_err(|_| AsmError::InvalidArgument(format!(
                    "Строка {}: Неверный формат вещественного числа '{}' в DATA", line_num + 1, tokens[1]
                )))?;

                let word = DneprWord::from_float(val_f64);
                binary.push(word.0 & WORD_MASK);
                continue;
            }

            let instruction = match mnemonic.as_str() {
                "HALT"     => Self::encode_op(0x00),
                "READ_ADC" => Self::encode_op(0x11),

                "STORE"    => Self::encode_r1(0x01, &tokens, 1, line_num, labels, defines)?,
                "JUMP"     => Self::encode_r1(0x04, &tokens, 1, line_num, labels, defines)?,
                "JZ"       => Self::encode_r1(0x05, &tokens, 1, line_num, labels, defines)?,
                "SEL_CH"   => Self::encode_r1(0x10, &tokens, 1, line_num, labels, defines)?,

                "ADD"       => Self::encode_r2(0x02, &tokens, 1, 2, line_num, labels, defines)?,
                "SUB"       => Self::encode_r2(0x03, &tokens, 1, 2, line_num, labels, defines)?,
                "MULT"      => Self::encode_r2(0x06, &tokens, 1, 2, line_num, labels, defines)?,
                "JPS"       => Self::encode_r2(0x07, &tokens, 1, 2, line_num, labels, defines)?,
                "WRITE_DAC" => Self::encode_r2(0x12, &tokens, 1, 2, line_num, labels, defines)?,

                "SHL" => Self::encode_r2(0x08, &tokens, 1, 2, line_num, labels, defines)?,
                "SHR" => Self::encode_r2(0x09, &tokens, 1, 2, line_num, labels, defines)?,

                _ => return Err(AsmError::UnknownOpcode(format!(
                    "Строка {}: Неизвестная команда '{}'", line_num + 1, mnemonic
                ))),
            };

            binary.push(instruction & WORD_MASK);
        }
        Ok(binary)
    }

    fn encode_op(opcode: u32) -> u32 {
        opcode << 20
    }

    fn encode_r1(opcode: u32, tokens: &[&str], i1: usize, line: usize, labels: &HashMap<String, u32>, defines: &HashMap<String, u32>) -> Result<u32, AsmError> {
        let addr1 = AsmParser::parse_token(tokens, i1, line, labels, defines)?;
        Ok((opcode << 20) | ((addr1 & 0x3FF) << 10))
    }

    fn encode_r2(opcode: u32, tokens: &[&str], i1: usize, i2: usize, line: usize, labels: &HashMap<String, u32>, defines: &HashMap<String, u32>) -> Result<u32, AsmError> {
        let addr1 = AsmParser::parse_token(tokens, i1, line, labels, defines)?;
        let addr2 = AsmParser::parse_token(tokens, i2, line, labels, defines)?;
        Ok((opcode << 20) | ((addr1 & 0x3FF) << 10) | (addr2 & 0x3FF))
    }
}
