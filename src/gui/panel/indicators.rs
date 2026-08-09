use eframe::egui;
use crate::cpu::{DneprCPU, MEMORY_SIZE};
use crate::types::DneprWord;

pub fn draw_indicators_and_buttons(ui: &mut egui::Ui, cpu: &mut DneprCPU) {
    // БЛОК ИНДИКАЦИИ (Неоновые лампы аккумулятора ACC)
    ui.group(|ui| {
        ui.label("🔴 Световые индикаторы аккумулятора (ACC):");
        ui.horizontal(|ui| {
            for bit in (0..26).rev() {
                let is_on = (cpu.accumulator & (1 << bit)) != 0;
                let color = if is_on { egui::Color32::from_rgb(255, 85, 85) } else { egui::Color32::from_rgb(40, 40, 40) };
                let (rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
                ui.painter().circle_filled(rect.center(), 5.5, color);

                if bit % 3 == 0 && bit != 0 {
                    ui.add_space(6.0);
                }
            }
        });
        let acc_word = DneprWord(cpu.accumulator);
        ui.label(format!("Значение ACC (f64): {:.6}", acc_word.to_float()));
    });

    ui.add_space(10.0);

    // КНОПКИ УПРАВЛЕНИЯ РЕЖИМАМИ ЭВМ (Одиночные командные кнопки)
    ui.group(|ui| {
        ui.label("🕹️ Системные аппаратные кнопки:");
        ui.horizontal(|ui| {
            if ui.button("▶️ ПУСК").clicked() {
                cpu.is_running = true;
                cpu.log_message("[Пульт] Нажата кнопка ПУСК. ЭВМ переведена в автоматический режим.".to_string());
            }
            if ui.button("⏸️ ОСТАНОВ").clicked() {
                cpu.is_running = false;
                cpu.log_message("[Пульт] Нажата кнопка ОСТАНОВ.".to_string());
            }
            if ui.button("⏭️ ТАКТ-ШАГ").clicked() {
                cpu.step();
            }

            // --- НОВЫЕ АУТЕНТИЧНЫЕ КНОПКИ ПУЛЬТА «ДНЕПР» ---
            if ui.button("💾 ЗАПИСЬ В ОЗУ").clicked() {
                if cpu.program_counter < MEMORY_SIZE {
                    cpu.memory[cpu.program_counter] = cpu.accumulator;
                    cpu.log_message(format!(
                        "[Пульт] Ручная запись: Значение {:#010X} записано в ячейку [{:03}]",
                        cpu.accumulator, cpu.program_counter
                    ));
                }
            }
            if ui.button("📖 ЧТЕНИЕ ИЗ ОЗУ").clicked() {
                if cpu.program_counter < MEMORY_SIZE {
                    cpu.accumulator = cpu.memory[cpu.program_counter];
                    cpu.log_message(format!(
                        "[Пульт] Ручное чтение: Из ячейки [{:03}] считано значение {:#010X}",
                        cpu.program_counter, cpu.accumulator
                    ));
                }
            }

            if ui.button("🔄 СБРОС (ОЗУ+ЦП)").clicked() {
                cpu.accumulator = 0;
                cpu.program_counter = 0;
                cpu.cycles = 0;
                cpu.is_running = false;
                cpu.memory = [0; MEMORY_SIZE];
                cpu.program_switches = [false; 5];
                cpu.logs.clear();
                cpu.logs.push("[Система] Выполнен полный аппаратный сброс комплекса.".to_string());
            }
        });
    });
}
