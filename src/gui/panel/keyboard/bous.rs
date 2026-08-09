use eframe::egui;
use crate::cpu::{DneprCPU, MEMORY_SIZE};

pub fn draw_bous_modules(
    ui: &mut egui::Ui,
    cpu: &mut DneprCPU,
    input_switches_a1: &mut [bool; 26],
    input_switches_a2: &mut [bool; 26]
) {
    ui.vertical(|ui| {
        // --- МОДУЛЬ БОУС-1 (Адрес 1) ---
        ui.label(egui::RichText::new("ЛЕВЫЙ БОУС-1 (Регистр Адреса 1)").strong());
        ui.horizontal(|ui| {
            for bit in (0..26).rev() {
                let btn_color = get_historical_color(bit, input_switches_a1[bit]);
                let text_color = if bit >= 9 && bit < 18 { egui::Color32::BLACK } else { egui::Color32::WHITE };

                let btn = egui::Button::new(egui::RichText::new(format!("{}", bit)).small().color(text_color))
                    .fill(btn_color)
                    .min_size(egui::vec2(20.0, 26.0));

                if ui.add(btn).clicked() {
                    input_switches_a1[bit] = !input_switches_a1[bit];
                }
                if bit % 3 == 0 && bit != 0 { ui.add_space(4.0); }
            }
        });

        ui.add_space(10.0);

        // --- МОДУЛЬ БОУС-2 (Адрес 2 / Данные) ---
        ui.label(egui::RichText::new("ПРАВЫЙ БОУС-2 (Регистр Адреса 2 / Данных)").strong());
        ui.horizontal(|ui| {
            for bit in (0..26).rev() {
                let btn_color = get_historical_color(bit, input_switches_a2[bit]);
                let text_color = if bit >= 9 && bit < 18 { egui::Color32::BLACK } else { egui::Color32::WHITE };

                let btn = egui::Button::new(egui::RichText::new(format!("{}", bit)).small().color(text_color))
                    .fill(btn_color)
                    .min_size(egui::vec2(20.0, 26.0));

                if ui.add(btn).clicked() {
                    input_switches_a2[bit] = !input_switches_a2[bit];
                }
                if bit % 3 == 0 && bit != 0 { ui.add_space(4.0); }
            }
        });

        ui.add_space(10.0);

        // Кнопки переноса кодов
        ui.horizontal(|ui| {
            let val_a1 = bits_to_u32(input_switches_a1);
            let val_a2 = bits_to_u32(input_switches_a2);

            if ui.button("📥 Записать БОУС-1 в ACC").clicked() {
                cpu.accumulator = val_a1;
                cpu.log_message(format!("[Пульт] Код с левого БОУС-1 ({:#010X}) записан в ACC", val_a1));
            }

            if ui.button("📍 Установить PC из БОУС-1").clicked() {
                if val_a1 < MEMORY_SIZE as u32 {
                    cpu.program_counter = val_a1 as usize;
                    cpu.log_message(format!("[Пульт] Счетчик команд (PC) вручную переведен на адрес [{:03}]", val_a1));
                } else {
                    cpu.log_message(format!("[Ошибка] Адрес [{}] выходит за пределы ОЗУ", val_a1));
                }
            }

            if ui.button("📥 Записать БОУС-2 в ACC").clicked() {
                cpu.accumulator = val_a2;
                cpu.log_message(format!("[Пульт] Код с правого БОУС-2 ({:#010X}) записан в ACC", val_a2));
            }
            if ui.button("🗑️ Сбросить всё").clicked() {
                *input_switches_a1 = [false; 26];
                *input_switches_a2 = [false; 26];
            }
        });
    });
}

fn get_historical_color(bit: usize, is_active: bool) -> egui::Color32 {
    if bit >= 18 {
        if is_active { egui::Color32::from_rgb(200, 50, 50) } else { egui::Color32::from_rgb(140, 30, 30) }
    } else if bit >= 9 {
        if is_active { egui::Color32::from_rgb(240, 240, 220) } else { egui::Color32::from_rgb(190, 190, 170) }
    } else {
        if is_active { egui::Color32::from_rgb(80, 80, 80) } else { egui::Color32::from_rgb(30, 30, 30) }
    }
}

fn bits_to_u32(switches: &[bool; 26]) -> u32 {
    let mut val: u32 = 0;
    for bit in 0..26 {
        if switches[bit] { val |= 1 << bit; }
    }
    val
}
