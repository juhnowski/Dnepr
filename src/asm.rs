use crate::types::WORD_MASK;
use std::collections::HashMap;

/// Ошибки, которые могут возникнуть при сборке (ассемблировании)
#[derive(Debug)]
pub enum AsmError {
    MissingArgument(String),
    InvalidArgument(String),
    UnknownOpcode(String),
    DuplicateLabel(String),
    UnknownLabel(String),
}

pub struct Assembler;

impl Assembler {
    /// Компиляция исходного текста программы в бинарный код для процессора
    pub fn compile(source: &str) -> Result<Vec<u32>, AsmError> {
        // Первый проход: собираем метки и их адреса
        let labels = Self::build_label_table(source)?;

        // Второй проход: генерируем машинный код
        Self::generate_binary(source, &labels)
    }

    /// Очистка строки от комментариев и разбиение на значимые слова (токены)
    fn tokenize(line: &str) -> Vec<&str> {
        line.split(';')
            .next()
            .unwrap_or("")
            .trim()
            .split_whitespace()
            .collect()
    }

    /// ПЕРВЫЙ ПРОХОД: Сбор меток и определение их адресов в ОЗУ
    fn build_label_table(source: &str) -> Result<HashMap<String, u32>, AsmError> {
        let mut labels = HashMap::new();
        let mut instruction_count = 0u32;

        for (line_num, line) in source.lines().enumerate() {
            let tokens = Self::tokenize(line);
            if tokens.is_empty() { continue; }

            let first_token = tokens[0];
            if first_token.ends_with(':') {
                let label_name = first_token[..first_token.len() - 1].to_uppercase();
                if labels.contains_key(&label_name) {
                    return Err(AsmError::DuplicateLabel(format!(
                        "Строка {}: Метка '{}' уже определена", line_num + 1, label_name
                    )));
                }
                labels.insert(label_name, instruction_count);

                // Если за меткой в той же строке идет команда, она занимает ячейку
                if tokens.len() > 1 { instruction_count += 1; }
            } else {
                instruction_count += 1;
            }
        }
        Ok(labels)
    }

    /// ВТОРОЙ ПРОХОД: Парсинг команд и генерация бинарного кода
    fn generate_binary(source: &str, labels: &HashMap<String, u32>) -> Result<Vec<u32>, AsmError> {
        let mut binary = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            let mut tokens = Self::tokenize(line);
            if tokens.is_empty() { continue; }

            // Пропускаем метку, если она присутствует в начале строки
            if tokens[0].ends_with(':') {
                tokens.remove(0);
                if tokens.is_empty() { continue; }
            }

            let mnemonic = tokens[0].to_uppercase();
            let instruction = match mnemonic.as_str() {
                // Безадресные команды (только опкод)
                "HALT"     => Self::encode_op(0x00),
                "READ_ADC" => Self::encode_op(0x11),

                // Одноадресные команды (опкод + addr1)
                "STORE"    => Self::encode_r1(0x01, &tokens, 1, line_num, labels)?,
                "JUMP"     => Self::encode_r1(0x04, &tokens, 1, line_num, labels)?,
                "JZ"       => Self::encode_r1(0x05, &tokens, 1, line_num, labels)?,
                "SEL_CH"   => Self::encode_r1(0x10, &tokens, 1, line_num, labels)?,

                // Двухадресные команды (опкод + addr1 + addr2)
                "ADD"       => Self::encode_r2(0x02, &tokens, 1, 2, line_num, labels)?,
                "SUB"       => Self::encode_r2(0x03, &tokens, 1, 2, line_num, labels)?,
                "MULT"      => Self::encode_r2(0x06, &tokens, 1, 2, line_num, labels)?,
                "WRITE_DAC" => Self::encode_r2(0x12, &tokens, 1, 2, line_num, labels)?,

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

    fn encode_r1(opcode: u32, tokens: &[&str], i1: usize, line: usize, labels: &HashMap<String, u32>) -> Result<u32, AsmError> {
        let addr1 = Self::parse_arg_or_label(tokens, i1, line, labels)?;
        Ok((opcode << 20) | ((addr1 & 0x3FF) << 10))
    }

    fn encode_r2(opcode: u32, tokens: &[&str], i1: usize, i2: usize, line: usize, labels: &HashMap<String, u32>) -> Result<u32, AsmError> {
        let addr1 = Self::parse_arg_or_label(tokens, i1, line, labels)?;
        let addr2 = Self::parse_arg_or_label(tokens, i2, line, labels)?;
        Ok((opcode << 20) | ((addr1 & 0x3FF) << 10) | (addr2 & 0x3FF))
    }

    /// Извлечение аргумента (числа или адреса метки)
    fn parse_arg_or_label(tokens: &[&str], index: usize, line_num: usize, labels: &HashMap<String, u32>) -> Result<u32, AsmError> {
        if index >= tokens.len() {
            return Err(AsmError::MissingArgument(format!(
                "Строка {}: Отсутствует аргумент на позиции {}", line_num + 1, index
            )));
        }

        let arg = tokens[index];
        if let Ok(num) = arg.parse::<u32>() {
            return Ok(num);
        }

        let token_upper = arg.to_uppercase();
        labels.get(&token_upper).cloned().ok_or_else(|| {
            AsmError::UnknownLabel(format!(
                "Строка {}: Неизвестная метка или неверный аргумент '{}'", line_num + 1, arg
            ))
        })
    }
}
