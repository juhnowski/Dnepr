mod executor;
mod debug;

use crate::uso::PeripheralUso;

pub const MEMORY_SIZE: usize = 512;
pub const MAX_LOGS: usize = 100;

pub struct DneprCPU {
    pub accumulator: u32,
    pub program_counter: usize,
    pub memory: [u32; MEMORY_SIZE],
    pub memory_is_code: [bool; MEMORY_SIZE],
    pub is_running: bool,
    pub uso: PeripheralUso,
    pub cycles: u64,
    pub logs: Vec<String>,
    pub program_switches: [bool; 5],
}

impl DneprCPU {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            program_counter: 0,
            memory: [0; MEMORY_SIZE],
            memory_is_code: [false; MEMORY_SIZE], // По умолчанию вся память — это данные
            is_running: false,
            uso: PeripheralUso::new(),
            cycles: 0,
            logs: vec!["[Система] Симулятор ЭВМ «Днепр» готов к работе.".to_string()],
            program_switches: [false; 5],
        }
    }

    pub fn step(&mut self) {
        if self.program_counter >= MEMORY_SIZE {
            self.log_message("[Система] Останов: Достигнут конец памяти ОЗУ.".to_string());
            self.is_running = false;
            return;
        }

        let instruction = self.memory[self.program_counter];

        if instruction == 0 {
            self.log_message(format!("[ЦП] Ячейка [{:03}] пуста. Аппаратный останов (HALT).", self.program_counter));
            self.is_running = false;
            return;
        }

        // Если мы принудительно выполняем эту ячейку, значит сейчас это точно код
        self.memory_is_code[self.program_counter] = true;

        self.program_counter += 1;

        let opcode = (instruction >> 20) & 0x3F;
        let addr1 = ((instruction >> 10) & 0x3FF) as usize;
        let addr2 = (instruction & 0x3FF) as usize;

        self.execute(opcode, addr1, addr2);
    }
}
