mod executor;
mod debug;

use crate::uso::PeripheralUso;

pub const MEMORY_SIZE: usize = 512;
pub const MAX_LOGS: usize = 100;

pub struct DneprCPU {
    pub accumulator: u32,
    pub program_counter: usize,
    pub memory: [u32; MEMORY_SIZE],
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
            is_running: false,
            uso: PeripheralUso::new(),
            cycles: 0,
            logs: vec!["[Система] Симулятор ЭВМ «Днепр» готов к работе.".to_string()],
            program_switches: [false; 5],
        }
    }

    /// Цикл выборки и декодирования инструкции
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

        // Вызов исполнительного ядра из executor.rs
        self.execute(opcode, addr1, addr2);
    }
}
