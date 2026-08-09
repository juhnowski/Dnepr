use crate::uso::{PeripheralUso, ADC_CHANNELS, DAC_CHANNELS};
use crate::types::DneprWord;

pub const MEMORY_SIZE: usize = 512;
const MAX_LOGS: usize = 100; // Лимит строк в консоли GUI, чтобы не забивать память

pub struct DneprCPU {
    pub accumulator: u32,
    pub program_counter: usize,
    pub memory: [u32; MEMORY_SIZE],
    pub is_running: bool,
    pub uso: PeripheralUso,
    pub cycles: u64,
    // Накопитель логов для графического интерфейса
    pub logs: Vec<String>,
}

impl DneprCPU {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            program_counter: 0,
            memory: [0; MEMORY_SIZE],
            is_running: false,
            uso: PeripheralUso::new(),
            cycles: 0,
            logs: vec!["[Система] Симулятор ЭВМ «Днепр» готов к работе.".to_string()],
        }
    }

    /// Добавление записи в историю логов
    pub fn log_message(&mut self, msg: String) {
        // Выводим в стандартный терминал для дублирования
        println!("{}", msg);

        self.logs.push(msg);
        if self.logs.len() > MAX_LOGS {
            self.logs.remove(0);
        }
    }

    pub fn step(&mut self) {
        if !self.is_running || self.program_counter >= MEMORY_SIZE {
            if self.is_running {
                self.log_message("[Система] Останов: Достигнут конец памяти ОЗУ.".to_string());
                self.is_running = false;
            }
            return;
        }

        let instruction = self.memory[self.program_counter];
        self.program_counter += 1;

        let opcode = (instruction >> 20) & 0x3F;
        let addr1 = ((instruction >> 10) & 0x3FF) as usize;
        let addr2 = (instruction & 0x3FF) as usize;

        self.execute(opcode, addr1, addr2);
    }

    fn execute(&mut self, opcode: u32, addr1: usize, addr2: usize) {
        let op_cycles = match opcode {
            0x00 => 1, 0x01 => 2, 0x02 => 2, 0x03 => 2,
            0x04 => 1, 0x05 => 1, 0x06 => 6, 0x10 => 1,
            0x11 => 4, 0x12 => 3, _ => 1,
        };

        self.cycles += op_cycles;

        match opcode {
            0x00 => { // HALT
                self.is_running = false;
                self.log_message(format!("[ЦП] HALT: Выполнение остановлено (ячейка {})", self.program_counter - 1));
            }
            0x01 => { // STORE
                if addr1 < MEMORY_SIZE {
                    self.memory[addr1] = self.accumulator;
                    self.log_message(format!(
                        "[ЦП] STORE: Значение {:#010X} ({:.4}) сохранено в ячейку {}",
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
                    self.log_message(format!("[ЦП] ADD: {:.4} + {:.4} = {:.4}", w1.to_float(), w2.to_float(), res.to_float()));
                }
            }
            0x03 => { // SUB
                if addr1 < MEMORY_SIZE && addr2 < MEMORY_SIZE {
                    let w1 = DneprWord(self.memory[addr1]);
                    let w2 = DneprWord(self.memory[addr2]);
                    let res = w1.sub(w2);
                    self.accumulator = res.0;
                    self.log_message(format!("[ЦП] SUB: {:.4} - {:.4} = {:.4}", w1.to_float(), w2.to_float(), res.to_float()));
                }
            }
            0x04 => { // JUMP
                if addr1 < MEMORY_SIZE {
                    self.program_counter = addr1;
                    self.log_message(format!("[ЦП] JUMP на адрес {}", addr1));
                }
            }
            0x05 => { // JZ
                if self.accumulator == 0 {
                    if addr1 < MEMORY_SIZE {
                        self.program_counter = addr1;
                        self.log_message(format!("[ЦП] JZ на адрес {} выполнен (ACC = 0)", addr1));
                    }
                } else {
                    self.log_message(format!("[ЦП] JZ на адрес {} пропущен (ACC != 0)", addr1));
                }
            }
            0x06 => { // MULT
                if addr1 < MEMORY_SIZE && addr2 < MEMORY_SIZE {
                    let w1 = DneprWord(self.memory[addr1]);
                    let w2 = DneprWord(self.memory[addr2]);
                    let res = w1.multiply(w2);
                    self.accumulator = res.0;
                    self.log_message(format!("[ЦП] MULT: {:.4} * {:.4} = {:.4}", w1.to_float(), w2.to_float(), res.to_float()));
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
                self.log_message(format!("[УСО] READ_ADC с канала {}: {:.4}", self.uso.selected_channel, analog_val));
            }
            0x12 => { // WRITE_DAC
                if addr1 < DAC_CHANNELS && addr2 < MEMORY_SIZE {
                    let word = DneprWord(self.memory[addr2]);
                    let analog_val = word.to_float();
                    self.uso.dac_outputs[addr1] = analog_val;
                    self.log_message(format!("[УСО] WRITE_DAC на канал {}: {:.4}", addr1, analog_val));
                }
            }
            _ => {
                self.log_message(format!("[Ошибка] Неизвестный опкод: {:#X}", opcode));
                self.is_running = false;
            }
        }
    }

    pub fn print_dump(&self) {
        println!("\n=================================================================");
        println!("         ФИНАЛЬНЫЙ ДАМП СОСТОЯНИЯ ЭВМ «ДНЕПР»                    ");
        println!("=================================================================");
        let acc_word = DneprWord(self.accumulator);
        println!(" Регистры и статистика процессора:");
        println!("   Счетчик команд (PC):   {}", self.program_counter);
        println!("   Аккумулятор (ACC):     {:#010X}", self.accumulator);
        println!("   Значение ACC (f64):    {:.6}", acc_word.to_float());
        println!("   Всего тактов (Cycles): {}", self.cycles);
        println!("   Примерное время ЭВМ:   {} мкс (~{:.2} мс)", self.cycles * 34, (self.cycles * 34) as f64 / 1000.0);
        println!("   Статус процессора:     {}", if self.is_running { "РАБОТАЕТ" } else { "ОСТАНОВЛЕН" });
        println!("=================================================================\n");
    }
}
