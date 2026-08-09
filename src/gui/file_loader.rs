use std::fs;
use crate::gui::DneprGuiApp;
use crate::asm::Assembler;
use crate::cpu::MEMORY_SIZE;

impl DneprGuiApp {
    /// Вызов диалогового окна и загрузка .asm файла в ОЗУ симулятора
    // Обновите метод open_and_load_file в файле src/gui/file_loader.rs

        pub fn open_and_load_file(&mut self) {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Ассемблер Днепр", &["asm", "txt"])
                .pick_file()
            {
                self.cpu.log_message(format!("[Система] Чтение файла: {}", path.display()));
                match fs::read_to_string(&path) {
                    Ok(asm_code) => {
                        match Assembler::compile(&asm_code) {
                            Ok(binary_program) => {
                                self.cpu.accumulator = 0;
                                self.cpu.program_counter = 0;
                                self.cpu.cycles = 0;
                                self.cpu.is_running = false;
                                self.cpu.memory = [0; MEMORY_SIZE];
                                self.cpu.memory_is_code = [false; MEMORY_SIZE]; // Сброс карты

                                // Записываем бинарный код
                                for (i, &instruction) in binary_program.iter().enumerate() {
                                    if i < MEMORY_SIZE {
                                        self.cpu.memory[i] = instruction;
                                    }
                                }

                                // Интеллектуальное заполнение карты типов на основе исходного текста
                                let mut mem_idx = 0;
                                for line in asm_code.lines() {
                                    let tokens = line.split(';').next().unwrap_or("").trim();
                                    if tokens.is_empty() || tokens.to_uppercase().starts_with("DEFINE") {
                                        continue;
                                    }

                                    let clean_tokens: Vec<&str> = tokens.split_whitespace().collect();
                                    if clean_tokens.is_empty() { continue; }

                                    // Пропускаем метку в начале строки, если она есть
                                    let mut cmd_token = clean_tokens[0];
                                    if cmd_token.ends_with(':') && clean_tokens.len() > 1 {
                                        cmd_token = clean_tokens[1];
                                    }

                                    if mem_idx < MEMORY_SIZE && cmd_token.to_uppercase() != "DATA" && !cmd_token.ends_with(':') {
                                        self.cpu.memory_is_code[mem_idx] = true;
                                    }
                                    if mem_idx < MEMORY_SIZE {
                                        mem_idx += 1;
                                    }
                                }

                                self.cpu.log_message("[Система] Программа успешно скомпилирована и загружена в ОЗУ.".to_string());
                            }
                            Err(e) => {
                                self.cpu.log_message(format!("[Ошибка Ассемблера] {:?}", e));
                            }
                        }
                    }
                    Err(e) => {
                        self.cpu.log_message(format!("[Ошибка Файла] Не удалось прочесть: {}", e));
                    }
                }
            }
        }

}
