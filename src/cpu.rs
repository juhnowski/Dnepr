use crate::uso::{PeripheralUso, ADC_CHANNELS, DAC_CHANNELS};
use crate::types::DneprWord;

pub const MEMORY_SIZE: usize = 512;

pub struct DneprCPU {
    pub accumulator: u32,
    pub program_counter: usize,
    pub memory: [u32; MEMORY_SIZE],
    pub is_running: bool,
    pub uso: PeripheralUso,
}

impl DneprCPU {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            program_counter: 0,
            memory: [0; MEMORY_SIZE],
            is_running: false,
            uso: PeripheralUso::new(),
        }
    }

    pub fn step(&mut self) {
        if !self.is_running || self.program_counter >= MEMORY_SIZE {
            self.is_running = false;
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
        match opcode {
            0x00 => { // HALT
                self.is_running = false;
                println!("[ЦП] HALT: Выполнение программы остановлено на адресе ячейки {}", self.program_counter - 1);
            }
            0x01 => { // STORE
                if addr1 < MEMORY_SIZE {
                    self.memory[addr1] = self.accumulator;
                    println!(
                        "[ЦП] STORE: Значение {:#010X} ({:.4}) сохранено в ячейку памяти {}",
                        self.accumulator,
                        DneprWord(self.accumulator).to_float(),
                        addr1
                    );
                }
            }
            0x02 => { // ADD
                if addr1 < MEMORY_SIZE && addr2 < MEMORY_SIZE {
                    let w1 = DneprWord(self.memory[addr1]);
                    let w2 = DneprWord(self.memory[addr2]);
                    let res = w1.add(w2);
                    self.accumulator = res.0;

                    println!("[ЦП] ADD: {:.4} + {:.4} = {:.4}", w1.to_float(), w2.to_float(), res.to_float());
                }
            }
            0x03 => { // SUB
                if addr1 < MEMORY_SIZE && addr2 < MEMORY_SIZE {
                    let w1 = DneprWord(self.memory[addr1]);
                    let w2 = DneprWord(self.memory[addr2]);
                    let res = w1.sub(w2);
                    self.accumulator = res.0;

                    println!("[ЦП] SUB: {:.4} - {:.4} = {:.4}", w1.to_float(), w2.to_float(), res.to_float());
                }
            }
            0x04 => { // JUMP
                if addr1 < MEMORY_SIZE {
                    self.program_counter = addr1;
                    println!("[ЦП] JUMP на адрес {}", addr1);
                }
            }
            0x05 => { // JZ
                if self.accumulator == 0 {
                    if addr1 < MEMORY_SIZE {
                        self.program_counter = addr1;
                        println!("[ЦП] JZ на адрес {} выполнен (аккумулятор = 0)", addr1);
                    }
                } else {
                    println!("[ЦП] JZ на адрес {} пропущен (аккумулятор != 0)", addr1);
                }
            }
            0x06 => { // MULT
                if addr1 < MEMORY_SIZE && addr2 < MEMORY_SIZE {
                    let w1 = DneprWord(self.memory[addr1]);
                    let w2 = DneprWord(self.memory[addr2]);
                    let res = w1.multiply(w2);
                    self.accumulator = res.0;

                    println!("[ЦП] MULT: {:.4} * {:.4} = {:.4}", w1.to_float(), w2.to_float(), res.to_float());
                }
            }
            0x10 => { // SEL_CH
                if addr1 < ADC_CHANNELS {
                    self.uso.selected_channel = addr1;
                    println!("[УСО] Выбран канал АЦП: {}", addr1);
                }
            }
            0x11 => { // READ_ADC
                let analog_val = self.uso.adc_inputs[self.uso.selected_channel];
                let word = DneprWord::from_float(analog_val);
                self.accumulator = word.0;
                println!(
                    "[УСО] АЦП считал с канала {}: {:.4} (В коде ЭВМ: {:#010X})",
                    self.uso.selected_channel, analog_val, word.0
                );
            }
            0x12 => { // WRITE_DAC
                if addr1 < DAC_CHANNELS && addr2 < MEMORY_SIZE {
                    let word = DneprWord(self.memory[addr2]);
                    let analog_val = word.to_float();
                    self.uso.dac_outputs[addr1] = analog_val;
                    println!("[УСО] ЦАП вывел на канал {}: {:.4}", addr1, analog_val);
                }
            }
            _ => {
                println!("Неизвестный опкод: {:#X}", opcode);
                self.is_running = false;
            }
        }
    }

    /// Вывод красивого дампа состояния процессора и задействованной памяти
    pub fn print_dump(&self) {
        println!("\n=================================================================");
        println!("         ФИНАЛЬНЫЙ ДАМП СОСТОЯНИЯ ЭВМ «ДНЕПР»                    ");
        println!("=================================================================");

        let acc_word = DneprWord(self.accumulator);
        println!(" Регистры процессора:");
        println!("   Счетчик команд (PC):   {}", self.program_counter);
        println!("   Аккумулятор (ACC):     {:#010X} (двоичный код: {:026b})", self.accumulator, self.accumulator);
        println!("   Значение ACC (f64):    {:.6}", acc_word.to_float());
        println!("   Статус процессора:     {}", if self.is_running { "РАБОТАЕТ" } else { "ОСТАНОВЛЕН" });

        println!("\n Состояние выходов ЦАП (УСО):");
        for (ch, &val) in self.uso.dac_outputs.iter().enumerate() {
            if val != 0.0 {
                println!("   Канал ЦАП #{}:            {:.4}", ch, val);
            }
        }

        println!("\n Задействованные ячейки памяти (ОЗУ):");
        let mut has_data = false;
        for (addr, &val) in self.memory.iter().enumerate() {
            if val != 0 {
                has_data = true;
                let word = DneprWord(val);
                println!(
                    "   Ячейка [{:03}]:  Код: {:#010X}  |  Вещественное: {:>9.6}  |  Биты: {:026b}",
                    addr, val, word.to_float(), val
                );
            }
        }

        if !has_data {
            println!("   [Память пуста]");
        }
        println!("=================================================================\n");
    }

}
