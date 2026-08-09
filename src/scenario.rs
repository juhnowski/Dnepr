use std::fs;
use std::path::Path;
use crate::uso::PeripheralUso;

#[derive(Debug, Clone)]
pub struct ScenarioEvent {
    pub iteration: usize,
    pub channel: usize,
    pub value: f64,
}

// Макрос автоматически создаст метод Scenario::default() с пустым вектором внутри
#[derive(Default)]
pub struct Scenario {
    events: Vec<ScenarioEvent>,
}

impl Scenario {
    /// Загрузка файла сценария. Возвращает ошибку только при неверном синтаксисе существующего файла.
    pub fn load<P: AsRef<Path>>(file_path: P) -> Result<Option<Self>, String> {
        let path = file_path.as_ref();

        // Если файла нет — возвращаем Ok(None), это не ошибка
        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(path)
            .map_err(|e| format!("Не удалось прочитать файл сценария: {}", e))?;

        let mut events = Vec::new();

        for (line_idx, line) in content.lines().enumerate() {
            let clean_line = match line.split(';').next() {
                Some(code) => code.trim(),
                None => continue,
            };

            if clean_line.is_empty() {
                continue;
            }

            let tokens: Vec<&str> = clean_line.split_whitespace().collect();
            if tokens.len() != 3 {
                return Err(format!(
                    "Ошибка в сценарии (строка {}): должно быть 3 аргумента (итерация, канал, значение)",
                    line_idx + 1
                ));
            }

            let iteration = tokens[0].parse::<usize>().map_err(|_| format!("Строка {}: неверная итерация", line_idx + 1))?;
            let channel = tokens[1].parse::<usize>().map_err(|_| format!("Строка {}: неверный канал", line_idx + 1))?;
            let value = tokens[2].parse::<f64>().map_err(|_| format!("Строка {}: неверное аналоговое значение", line_idx + 1))?;

            events.push(ScenarioEvent { iteration, channel, value });
        }

        Ok(Some(Self { events }))
    }

    pub fn update_environment(&self, current_iteration: usize, uso: &mut PeripheralUso) {
        for event in &self.events {
            if event.iteration == current_iteration {
                if event.channel < uso.adc_inputs.len() {
                    uso.adc_inputs[event.channel] = event.value;
                    println!(
                        "[Сценарий Среды] Шаг {}: На канал АЦП {} подано значение {:.4}",
                        current_iteration, event.channel, event.value
                    );
                }
            }
        }
    }
}
