use eframe::egui;
use crate::cpu::{DneprCPU, MEMORY_SIZE};
use crate::types::DneprWord;
use crate::uso::{ADC_CHANNELS, DAC_CHANNELS};

pub fn draw_control_panel(ui: &mut egui::Ui, cpu: &mut DneprCPU, input_switches: &mut [bool; 26]) {
    // БЛОК ИНДИКАЦИИ (Неоновые лампы аккумулятора ACC)
    ui.group(|ui| {
        ui.label("🔴 Световые индикаторы аккумулятора (ACC):");
        ui.horizontal(|ui| {
            for bit in (0..26).rev() {
                let is_on = (cpu.accumulator & (1 << bit)) != 0;
                let color = if is_on { egui::Color32::RED } else { egui::Color32::DARK_GRAY };
                let (rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
                ui.painter().circle_filled(rect.center(), 5.0, color);
            }
        });
        let acc_word = DneprWord(cpu.accumulator);
        ui.label(format!("Значение ACC (f64): {:.6}", acc_word.to_float()));
    });

    ui.add_space(10.0);

    // БЛОК ВВОД ДАННЫХ (Тумблеры регистра)
    ui.group(|ui| {
        ui.label("🎛️ Механический регистр ввода данных (26 тумблеров):");
        ui.horizontal(|ui| {
            for bit in (0..26).rev() {
                ui.vertical(|ui| {
                    ui.checkbox(&mut input_switches[bit], "");
                    ui.small(format!("{}", bit));
                });
            }
        });

        let mut entered_value: u32 = 0;
        for bit in 0..26 {
            if input_switches[bit] { entered_value |= 1 << bit; }
        }

        if ui.button("📥 Записать ввод в аккумулятор").clicked() {
            cpu.accumulator = entered_value;
        }
    });

    ui.add_space(10.0);

    // КНОПКИ УПРАВЛЕНИЯ РЕЖИМАМИ ЭВМ
    ui.group(|ui| {
        ui.label("🕹️ Системные команды:");
        ui.horizontal(|ui| {
            if ui.button("▶️ АВТОМАТ").clicked() {
                cpu.is_running = true;
            }
            if ui.button("⏸️ ОСТАНОВ").clicked() {
                cpu.is_running = false;
            }
            if ui.button("⏭️ ШАГ").clicked() {
                cpu.step();
            }
            if ui.button("🔄 СБРОС").clicked() {
                cpu.accumulator = 0;
                cpu.program_counter = 0;
                cpu.cycles = 0;
                cpu.is_running = false;
                cpu.memory = [0; MEMORY_SIZE];
                cpu.logs.clear();
                cpu.logs.push("[Система] Состояние сброшено.".to_string());
            }
        });
    });

    ui.add_space(10.0);

    // БЛОК: ИНТЕРАКТИВНЫЕ ПОЛЗУНКИ АЦП УСО
    ui.group(|ui| {
        ui.label("📡 Датчики технологического объекта (Входы АЦП УСО):");
        ui.add_space(5.0);

        for ch in 0..ADC_CHANNELS {
            ui.horizontal(|ui| {
                let is_active = cpu.uso.selected_channel == ch;
                let label_text = if is_active {
                    egui::RichText::new(format!("➡ Канал #{}:", ch)).color(egui::Color32::LIGHT_BLUE).strong()
                } else {
                    egui::RichText::new(format!("  Канал #{}:", ch))
                };

                ui.label(label_text);
                ui.add(
                    egui::Slider::new(&mut cpu.uso.adc_inputs[ch], -1.0..=0.999999)
                        .text(if ch == 3 { "(Датчик давления)" } else { "" })
                );
            });
        }
    });

    ui.add_space(10.0);

    // БЛОК: ВЫХОДНЫЕ СИГНАЛЫ ЦАП УСО (Исполнительные механизмы)
    ui.group(|ui| {
        ui.label("⚙️ Исполнительные механизмы (Выходы ЦАП УСО):");
        ui.add_space(5.0);

        // Используем сетку Grid для идеального выравнивания колонок
        egui::Grid::new("dac_grid")
            .num_columns(3) // 1: Имя, 2: Шкала, 3: Примечание
            .spacing([10.0, 6.0]) // Задаем отступы между колонками и строками
            .show(ui, |ui| {
                for ch in 0..DAC_CHANNELS {
                    // Колонка 1: Метка канала
                    ui.label(format!("  Выход ЦАП #{}:", ch));

                    // Расчет значения для шкалы
                    let val = cpu.uso.dac_outputs[ch];
                    let normalized_val = (val + 1.0) / 2.0;

                    // Колонка 2: Шкала прогресса с ЖЕСТКО ЗАДАННОЙ шириной
                    ui.add(
                        egui::ProgressBar::new(normalized_val as f32)
                            .text(format!("{:.4}", val))
                            .desired_width(220.0) // Фиксируем ширину всех шкал, чтобы они были строго одинаковыми
                    );

                    // Колонка 3: Пояснительный текст (не влияет на ширину шкалы)
                    if ch == 1 {
                        ui.small("(Клапан сброса давления)");
                    } else {
                        ui.label(""); // Пустая заглушка для остальных строк сетки
                    }

                    ui.end_row();
                }
            });
    });


    ui.add_space(10.0);
    ui.label(format!("Счетчик команд (PC): {}", cpu.program_counter));
    ui.label(format!("Всего выполнено тактов: {}", cpu.cycles));
    ui.label(format!("Виртуальное время работы: {} мкс", cpu.cycles * 34));
}
