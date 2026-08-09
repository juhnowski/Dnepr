pub mod error;
mod parser;

pub use error::AsmError;
use parser::AsmParser;

use crate::types::{WORD_MASK, DneprWord};
use std::collections::HashMap;

/// Результат компиляции, содержащий бинарный код и карту типов ячеек
pub struct AsmResult {
    pub binary: Vec<u32>,
    pub is_code: Vec<bool>,
}

pub struct Assembler;

impl Assembler {
    /// Компиляция исходного текста программы в машинный код и карту типов
    pub fn compile(source: &str) -> Result<AsmResult, AsmError> {
        let (labels, defines) = AsmParser::parse_labels_and_defines(source)?;
        Self::generate_binary(source, &labels, &defines)
    }

    /// ВТОРОЙ ПРОХОД: Генерация бинарных инструкций
    fn generate_binary(source: &str, labels: &HashMap<String, u32>, defines: &HashMap<String, u32>) -> Result<AsmResult, AsmError> {
        let mut binary = Vec::new();
        let mut is_code = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            let mut tokens = AsmParser::tokenize(line);
            if tokens.is_empty() { continue; }

            // Пропускаем директиву ОПР
            if tokens[0].to_uppercase() == "ОПР" { continue; }

            // Отрезаем метку
            if tokens[0].ends_with(':') {
                tokens.remove(0);
                if tokens.is_empty() { continue; }
            }

            let mnemonic = tokens[0].to_uppercase();

            // Обработка директивы ДАННЫЕ
            if mnemonic == "ДАННЫЕ" {
                if tokens.len() < 2 {
                    return Err(AsmError::MissingArgument(format!(
                        "Строка {}: Директива ДАННЫЕ требует числовое значение", line_num + 1
                    )));
                }
                let val_f64 = tokens[1].parse::<f64>().map_err(|_| AsmError::InvalidArgument(format!(
                    "Строка {}: Неверный формат вещественного числа '{}' в ДАННЫЕ", line_num + 1, tokens[1]
                )))?;

                let word = DneprWord::from_float(val_f64);
                binary.push(word.0 & WORD_MASK);
                is_code.push(false); // Это точно данные, а не исполняемый код
                continue;
            }

            // Кодирование операций
            let instruction = match mnemonic.as_str() {
                "ОСТ"    => Self::encode_op(0x00),      // Останов
                "ЧТ_АЦП" => Self::encode_op(0x11),      // Чтение АЦП

                "ЗП"     => Self::encode_r1(0x01, &tokens, 1, line_num, labels, defines)?, // Запись в память
                "БП"     => Self::encode_r1(0x04, &tokens, 1, line_num, labels, defines)?, // Безусловный переход
                "ПЗ"     => Self::encode_r1(0x05, &tokens, 1, line_num, labels, defines)?, // Переход по нулю
                "ВК"     => Self::encode_r1(0x10, &tokens, 1, line_num, labels, defines)?, // Выбор канала АЦП

                "СЛ"     => Self::encode_r2(0x02, &tokens, 1, 2, line_num, labels, defines)?, // Сложение
                "ВЫЧ"    => Self::encode_r2(0x03, &tokens, 1, 2, line_num, labels, defines)?, // Вычитание
                "УМН"    => Self::encode_r2(0x06, &tokens, 1, 2, line_num, labels, defines)?, // Умножение
                "ПК"     => Self::encode_r2(0x07, &tokens, 1, 2, line_num, labels, defines)?, // Переход по Ключу пульта
                "СДЛ"    => Self::encode_r2(0x08, &tokens, 1, 2, line_num, labels, defines)?, // Сдвиг влево
                "СДП"    => Self::encode_r2(0x09, &tokens, 1, 2, line_num, labels, defines)?, // Сдвиг вправо
                "ЗП_ЦАП" => Self::encode_r2(0x12, &tokens, 1, 2, line_num, labels, defines)?, // Запись в ЦАП

                _ => return Err(AsmError::UnknownOpcode(format!(
                    "Строка {}: Неизвестная команда '{}'", line_num + 1, mnemonic
                ))),
            };

            binary.push(instruction & WORD_MASK);
            is_code.push(true); // Это валидная команда, помечаем флагом кода
        }

        Ok(AsmResult { binary, is_code })
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
