use std::collections::HashMap;
use crate::asm::error::AsmError;

pub struct AsmParser;

impl AsmParser {
    /// Очистка строки от комментариев и разбиение на токены
    pub fn tokenize(line: &str) -> Vec<&str> {
        line.split(';')
            .next()
            .unwrap_or("")
            .trim()
            .split_whitespace()
            .collect()
    }

    /// ПЕРВЫЙ ПРОХОД: Сбор макроопределений DEFINE, меток команд и данных DATA
    pub fn parse_labels_and_defines(source: &str) -> Result<(HashMap<String, u32>, HashMap<String, u32>), AsmError> {
        let mut labels = HashMap::new();
        let mut defines = HashMap::new();
        let mut instruction_count = 0u32;

        for (line_num, line) in source.lines().enumerate() {
            let tokens = Self::tokenize(line);
            if tokens.is_empty() { continue; }

            if tokens[0].to_uppercase() == "DEFINE" {
                if tokens.len() < 3 {
                    return Err(AsmError::MissingArgument(format!(
                        "Строка {}: Директива DEFINE требует имя и значение", line_num + 1
                    )));
                }
                let name = tokens[1].to_uppercase();
                let val = tokens[2].parse::<u32>().map_err(|_| AsmError::InvalidArgument(format!(
                    "Строка {}: Ошибка парсинга значения DEFINE '{}'", line_num + 1, tokens[2]
                )))?;
                defines.insert(name, val);
                continue;
            }

            let first_token = tokens[0];
            if first_token.ends_with(':') {
                let label_name = first_token[..first_token.len() - 1].to_uppercase();
                if labels.contains_key(&label_name) {
                    return Err(AsmError::DuplicateLabel(format!(
                        "Строка {}: Метка '{}' уже определена", line_num + 1, label_name
                    )));
                }
                labels.insert(label_name, instruction_count);

                if tokens.len() > 1 { instruction_count += 1; }
            } else {
                instruction_count += 1;
            }
        }
        Ok((labels, defines))
    }

    /// Разрешение текстового токена в числовой адрес или значение константы
    pub fn parse_token(tokens: &[&str], index: usize, line_num: usize, labels: &HashMap<String, u32>, defines: &HashMap<String, u32>) -> Result<u32, AsmError> {
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
        if let Some(&val) = defines.get(&token_upper) {
            return Ok(val);
        }

        labels.get(&token_upper).cloned().ok_or_else(|| {
            AsmError::UnknownLabel(format!(
                "Строка {}: Неизвестный идентификатор '{}'", line_num + 1, arg
            ))
        })
    }
}
