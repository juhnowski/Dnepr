use crate::cpu::{DneprCPU, MAX_LOGS};
use crate::types::DneprWord;

impl DneprCPU {
    /// Логирование событий с дублированием в терминал Cargo и кольцевой буфер GUI
    pub fn log_message(&mut self, msg: String) {
        println!("{}", msg);

        self.logs.push(msg);
        if self.logs.len() > MAX_LOGS {
            self.logs.remove(0);
        }
    }

    /// Текстовый дамп регистров для консольного запуска
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
