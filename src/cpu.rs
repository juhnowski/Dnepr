use crate::uso::{PeripheralUso, ADC_CHANNELS, DAC_CHANNELS};
use crate::types::DneprWord;

pub const MEMORY_SIZE: usize = 512;

pub struct DneprCPU {
    pub accumulator: u32,
    pub program_counter: usize,
    pub memory: [u32; MEMORY_SIZE],
    pub is_running: bool,
    pub uso: PeripheralUso,
    // Счетчик тактов (циклов) процессора
    pub cycles: u64,
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
        // Определяем стоимость команды в тактах
        let op_cycles = match opcode {
            0x00 => 1,  // HALT
            0x01 => 2,  // STORE (обращение к памяти)
            0x02 => 2,  // ADD (чтение операндов из памяти)
            0x03 => 2,  // SUB
            0x04 => 1,  // JUMP
            0x05 => 1,  // JZ
            0x06 => 6,  // MULT (длинная операция умножения на Днепре)
            0x10 => 1,  // SEL_CH
            0x11 => 4,  // READ_ADC (требуется время на преобразование АЦП)
            0x12 => 3,  // WRITE_DAC (вывод на ЦАП)
            _ => 1,
        };

        self.cycles += op_cycles;

        match opcode {
            0x00 => { // HALT
                self.is_running = false;
                println!("[ЦП] HALT: Выполнение остановлено (команда заняла {} такт(ов))", op_cycles);
            }
            0x01 => { // STORE
                if addr1 < MEMORY_SIZE {
                    self.memory[addr1] = self.accumulator;
                    println!(
                        "[ЦП] STORE: Значение {:#010X} ({:.4}) сохранено в ячейку {} [{} такт(а)]",
                        self.accumulator, DneprWord(self.accumulator).to_float(), addr1, op_cycles
                    );
                }
            }
            0x02 => { // ADD
                if addr1 < MEMORY_SIZE && addr2 < MEMORY_SIZE {
                    let w1 = DneprWord(self.memory[addr1]);
                    let w2 = DneprWord(self.memory[addr2]);
                    let res = w1.add(w2);
                    self.accumulator = res.0;
                    println!("[ЦП] ADD: {:.4} + {:.4} = {:.4} [{} такт(а)]", w1.to_float(), w2.to_float(), res.to_float(), op_cycles);
                }
            }
            0x03 => { // SUB
                if addr1 < MEMORY_SIZE && addr2 < MEMORY_SIZE {
                    let w1 = DneprWord(self.memory[addr1]);
                    let w2 = DneprWord(self.memory[addr2]);
                    let res = w1.sub(w2);
                    self.accumulator = res.0;
                    println!("[ЦП] SUB: {:.4} - {:.4} = {:.4} [{} такт(а)]", w1.to_float(), w2.to_float(), res.to_float(), op_cycles);
                }
            }
            0x04 => { // JUMP
                if addr1 < MEMORY_SIZE {
                    self.program_counter = addr1;
                    println!("[ЦП] JUMP на адрес {} [{} такт]", addr1, op_cycles);
                }
            }
            0x05 => { // JZ
                if self.accumulator == 0 {
                    if addr1 < MEMORY_SIZE {
                        self.program_counter = addr1;
                        println!("[ЦП] JZ на адрес {} выполнен [{} такт]", addr1, op_cycles);
                    }
                } else {
                    println!("[ЦП] JZ на адрес {} пропущен (ACC != 0) [{} такт]", addr1, op_cycles);
                }
            }
            0x06 => { // MULT
                if addr1 < MEMORY_SIZE && addr2 < MEMORY_SIZE {
                    let w1 = DneprWord(self.memory[addr1]);
                    let w2 = DneprWord(self.memory[addr2]);
                    let res = w1.multiply(w2);
                    self.accumulator = res.0;
                    println!("[ЦП] MULT: {:.4} * {:.4} = {:.4} [{} тактов!]", w1.to_float(), w2.to_float(), res.to_float(), op_cycles);
                }
            }
            0x10 => { // SEL_CH
                if addr1 < ADC_CHANNELS {
                    self.uso.selected_channel = addr1;
                    println!("[УСО] Выбран канал АЦП: {} [{} такт]", addr1, op_cycles);
                }
            }
            0x11 => { // READ_ADC
                let analog_val = self.uso.adc_inputs[self.uso.selected_channel];
                let word = DneprWord::from_float(analog_val);
                self.accumulator = word.0;
                println!(
                    "[УСО] READ_ADC с канала {}: {:.4} [{} тактов на АЦП]",
                    self.uso.selected_channel, analog_val, op_cycles
                );
            }
            0x12 => { // WRITE_DAC
                if addr1 < DAC_CHANNELS && addr2 < MEMORY_SIZE {
                    let word = DneprWord(self.memory[addr2]);
                    let analog_val = word.to_float();
                    self.uso.dac_outputs[addr1] = analog_val;
                    println!("[УСО] WRITE_DAC на канал {}: {:.4} [{} тактов]", addr1, analog_val, op_cycles);
                }
            }
            _ => {
                println!("Неизвестный опкод: {:#X}", opcode);
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
        // Историческая справка: базовый такт/цикл УМШН "Днепр" составлял около 34 микросекунд
        println!("   Примерное время ЭВМ:   {} мкс (~{:.2} мс)", self.cycles * 34, (self.cycles * 34) as f64 / 1000.0);
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
                    "   Ячейка [{:03}]:  Код: {:#010X}  |  Вещественное: {:>9.6}",
                    addr, val, word.to_float()
                );
            }
        }

        if !has_data { println!("   [Память пуста]"); }
        println!("=================================================================\n");
    }
}
