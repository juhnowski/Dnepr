use crate::types::WORD_MASK;

/// Ошибки, которые могут возникнуть при сборке (ассемблировании)
#[derive(Debug)]
pub enum AsmError {
    InvalidInstruction(String),
    MissingArgument(String),
    InvalidArgument(String),
    UnknownOpcode(String),
}

pub struct Assembler;

impl Assembler {
    /// Компиляция исходного текста программы в бинарный код для процессора
    pub fn compile(source: &str) -> Result<Vec<u32>, AsmError> {
        let mut binary = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            // Удаляем комментарии и лишние пробелы
            let clean_line = match line.split(';').next() {
                Some(code) => code.trim(),
                None => continue,
            };

            // Пропускаем пустые строки
            if clean_line.is_empty() {
                continue;
            }

            // Разбиваем строку на токены (мнемоника и аргументы)
            let tokens: Vec<&str> = clean_line.split_whitespace().collect();
            // Важно: переводим саму мнемонику команды в верхний регистр
            let mnemonic = tokens[0].to_uppercase();

            let instruction = match mnemonic.as_str() {
                "HALT" => {
                    0x00 << 20
                }
                "STORE" => {
                    let addr1 = Self::parse_arg(&tokens, 1, line_num)?;
                    (0x01 << 20) | ((addr1 & 0x3FF) << 10)
                }
                "ADD" => {
                    let addr1 = Self::parse_arg(&tokens, 1, line_num)?;
                    let addr2 = Self::parse_arg(&tokens, 2, line_num)?;
                    (0x02 << 20) | ((addr1 & 0x3FF) << 10) | (addr2 & 0x3FF)
                }
                "SUB" => {
                    let addr1 = Self::parse_arg(&tokens, 1, line_num)?;
                    let addr2 = Self::parse_arg(&tokens, 2, line_num)?;
                    (0x03 << 20) | ((addr1 & 0x3FF) << 10) | (addr2 & 0x3FF)
                }
                "SEL_CH" => {
                    let channel = Self::parse_arg(&tokens, 1, line_num)?;
                    (0x10 << 20) | ((channel & 0x3FF) << 10)
                }
                "READ_ADC" => {
                    0x11 << 20
                }
                "WRITE_DAC" => {
                    let dac_ch = Self::parse_arg(&tokens, 1, line_num)?;
                    let mem_addr = Self::parse_arg(&tokens, 2, line_num)?;
                    (0x12 << 20) | ((dac_ch & 0x3FF) << 10) | (mem_addr & 0x3FF)
                }
                "JUMP" => {
                    let target_addr = Self::parse_arg(&tokens, 1, line_num)?;
                    (0x04 << 20) | ((target_addr & 0x3FF) << 10)
                }
                "JZ" => {
                    let target_addr = Self::parse_arg(&tokens, 1, line_num)?;
                    (0x05 << 20) | ((target_addr & 0x3FF) << 10)
                }
                _ => {
                    return Err(AsmError::UnknownOpcode(format!(
                        "Строка {}: Неизвестная команда '{}'", line_num + 1, mnemonic
                    )));
                }
            };

            binary.push(instruction & WORD_MASK);
        }

        Ok(binary)
    }

    /// Вспомогательная функция для безопасного извлечения и парсинга числовых аргументов
    fn parse_arg(tokens: &[&str], index: usize, line_num: usize) -> Result<u32, AsmError> {
        if index >= tokens.len() {
            return Err(AsmError::MissingArgument(format!(
                "Строка {}: Отсутствует аргумент на позиции {}", line_num + 1, index
            )));
        }

        tokens[index]
            .parse::<u32>()
            .map_err(|_| AsmError::InvalidArgument(format!(
                "Строка {}: Ошибка парсинга числа '{}'", line_num + 1, tokens[index]
            )))
    }
}
