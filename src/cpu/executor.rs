use crate::cpu::{DneprCPU, MEMORY_SIZE};
use crate::uso::{ADC_CHANNELS, DAC_CHANNELS};
use crate::types::DneprWord;

impl DneprCPU {
    /// Исполнение декодированной операции
    pub fn execute(&mut self, opcode: u32, addr1: usize, addr2: usize) {
        let op_cycles = match opcode {
            0x00 => 1, 0x01 => 2, 0x02 => 2, 0x03 => 2,
            0x04 => 1, 0x05 => 1, 0x06 => 6, 0x07 => 1,
            0x08 => 2,
            0x09 => 2,
            0x10 => 1, 0x11 => 4, 0x12 => 3, _ => 1,
        };

        self.cycles += op_cycles;

        match opcode {
            0x00 => { // HALT
                self.is_running = false;
                self.log_message(format!("[ЦП] ОСТ (Останов): Выполнение остановлено (ячейка {})", self.program_counter - 1));
            }
            0x01 => { // STORE
                if addr1 < MEMORY_SIZE {
                    self.memory[addr1] = self.accumulator;
                    self.log_message(format!(
                        "[ЦП] ЗП (Запись): Значение {:#010X} ({:.4}) сохранено в ячейку {}",
                        self.accumulator, DneprWord(self.accumulator).to_float(), addr1
                    ));
                }
            }
            0x02 => { // ADD
                if addr1 < MEMORY_SIZE && addr2 < MEMORY_SIZE {
                    let w1 = DneprWord(self.memory[addr1]);
                    let w2 = DneprWord(self.memory[addr2]);
                    let res = w1.add(w2);
                    self.accumulator = res.0;
                    self.log_message(format!("[ЦП] СЛ (Сложение): {:.4} + {:.4} = {:.4}", w1.to_float(), w2.to_float(), res.to_float()));
                }
            }
            0x03 => { // SUB
                if addr1 < MEMORY_SIZE && addr2 < MEMORY_SIZE {
                    let w1 = DneprWord(self.memory[addr1]);
                    let w2 = DneprWord(self.memory[addr2]);
                    let res = w1.sub(w2);
                    self.accumulator = res.0;
                    self.log_message(format!("[ЦП] ВЫЧ (Вычитание): {:.4} - {:.4} = {:.4}", w1.to_float(), w2.to_float(), res.to_float()));
                }
            }
            0x04 => { // JUMP
                if addr1 < MEMORY_SIZE {
                    self.program_counter = addr1;
                    self.log_message(format!("[ЦП] БП (Безусловный переход) на адрес {}", addr1));
                }
            }
            0x05 => { // JZ
                if self.accumulator == 0 {
                    if addr1 < MEMORY_SIZE {
                        self.program_counter = addr1;
                        self.log_message(format!("[ЦП] ПЗ (Переход по нулю) на адрес {} выполнен (ACC = 0)", addr1));
                    }
                } else {
                    self.log_message(format!("[ЦП] ПЗ (Переход по нулю) на адрес {} пропущен (ACC != 0)", addr1));
                }
            }
            0x06 => { // MULT
                if addr1 < MEMORY_SIZE && addr2 < MEMORY_SIZE {
                    let w1 = DneprWord(self.memory[addr1]);
                    let w2 = DneprWord(self.memory[addr2]);
                    let res = w1.multiply(w2);
                    self.accumulator = res.0;
                    self.log_message(format!("[ЦП] УМН (Умножение): {:.4} * {:.4} = {:.4}", w1.to_float(), w2.to_float(), res.to_float()));
                }
            }
            0x07 => { // JPS: Перейти на адрес addr2, если тумблер под номером addr1 включен
                if addr1 < 5 {
                    if self.program_switches[addr1] {
                        if addr2 < MEMORY_SIZE {
                            // Критично: Меняем счетчик команд на адрес перехода (addr2)
                            self.program_counter = addr2;

                            self.log_message(format!(
                                "[ЦП] ПК (Переход по Ключу): Тумблер П{} ВКЛЮЧЕН. Выполнен переход на адрес ячейки {}",
                                addr1, addr2
                            ));
                        }
                    } else {
                        self.log_message(format!(
                            "[ЦП] ПК (Переход по Ключу): Тумблер П{} ВЫКЛЮЧЕН. Переход на адрес {} пропущен.",
                            addr1, addr2
                        ));
                    }
                } else {
                    self.log_message(format!("[Ошибка] JPS: Неверный номер тумблера П{}", addr1));
                }
            }
            0x08 => { // SHL: Сдвиг знакового числа из [addr1] влево на addr2 бит -> ACC
                if addr1 < MEMORY_SIZE {
                    let w = DneprWord(self.memory[addr1]);
                    let res = w.shl(addr2 as u32);
                    self.accumulator = res.0;
                    self.log_message(format!(
                        "[ЦП] СДЛ (Сдвиг Влево): Выполнен сдвиг влево {:.4} на {} бит. Результат: {:.4}",
                        w.to_float(), addr2, res.to_float()
                    ));
                }
            }
            0x09 => { // SHR: Арифметический сдвиг из [addr1] вправо на addr2 бит -> ACC
                if addr1 < MEMORY_SIZE {
                    let w = DneprWord(self.memory[addr1]);
                    let res = w.shr(addr2 as u32);
                    self.accumulator = res.0;
                    self.log_message(format!(
                        "[ЦП] СДП (Сдвиг Вправо): Выполнен сдвиг вправо {:.4} на {} бит. Результат: {:.4}",
                        w.to_float(), addr2, res.to_float()
                    ));
                }
            }
            0x10 => { // SEL_CH
                if addr1 < ADC_CHANNELS {
                    self.uso.selected_channel = addr1;
                    self.log_message(format!("[УСО] Выбран канал АЦП: {}", addr1));
                }
            }
            0x11 => { // READ_ADC
                let analog_val = self.uso.adc_inputs[self.uso.selected_channel];
                let word = DneprWord::from_float(analog_val);
                self.accumulator = word.0;
                self.log_message(format!("[УСО] ЧТ_АЦП (Чтение) с канала {}: {:.4}", self.uso.selected_channel, analog_val));
            }
            0x12 => { // WRITE_DAC
                if addr1 < DAC_CHANNELS && addr2 < MEMORY_SIZE {
                    let word = DneprWord(self.memory[addr2]);
                    let analog_val = word.to_float();
                    self.uso.dac_outputs[addr1] = analog_val;
                    self.log_message(format!("[УСО] ЗП_ЦАП (Запись) на канал {}: {:.4}", addr1, analog_val));
                }
            }
            _ => {
                self.log_message(format!("[Ошибка] Неизвестный опкод: {:#X}", opcode));
                self.is_running = false;
            }
        }
    }
}
