use std::fs;
use crate::gui::DneprGuiApp;
use crate::asm::Assembler;
use crate::cpu::MEMORY_SIZE;

impl DneprGuiApp {
    pub fn open_and_load_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Ассемблер Днепр", &["asm", "txt"])
            .pick_file()
        {
            self.cpu.log_message(format!("[Система] Чтение файла: {}", path.display()));
            match fs::read_to_string(&path) {
                Ok(asm_code) => {
                    match Assembler::compile(&asm_code) {
                        Ok(compiled_result) => {
                            // Полный аппаратный сброс перед записью новой программы
                            self.cpu.accumulator = 0;
                            self.cpu.program_counter = 0;
                            self.cpu.cycles = 0;
                            self.cpu.is_running = false;
                            self.cpu.memory = [0; MEMORY_SIZE];
                            self.cpu.memory_is_code = [false; MEMORY_SIZE];

                            // Запись бинарного кода и точной карты типов в память процессора
                            for (i, &instruction) in compiled_result.binary.iter().enumerate() {
                                if i < MEMORY_SIZE {
                                    self.cpu.memory[i] = instruction;
                                    self.cpu.memory_is_code[i] = compiled_result.is_code[i];
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
