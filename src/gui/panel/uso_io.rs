use eframe::egui;
use crate::cpu::DneprCPU;
use crate::uso::{ADC_CHANNELS, DAC_CHANNELS};

pub fn draw_uso_io(ui: &mut egui::Ui, cpu: &mut DneprCPU) {
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
                ui.add(egui::Slider::new(&mut cpu.uso.adc_inputs[ch], -1.0..=0.999999).text(if ch == 3 { "(Датчик давления)" } else { "" }));
            });
        }
    });

    ui.add_space(10.0);

    // БЛОК: ВЫХОДНЫЕ СИГНАЛЫ ЦАП УСО (Исполнительные механизмы)
    ui.group(|ui| {
        ui.label("⚙️ Исполнительные механизмы (Выходы ЦАП УСО):");
        ui.add_space(5.0);
        egui::Grid::new("dac_grid")
            .num_columns(3)
            .spacing([10.0, 6.0])
            .show(ui, |ui| {
                for ch in 0..DAC_CHANNELS {
                    ui.label(format!("  Выход ЦАП #{}:", ch));
                    let val = cpu.uso.dac_outputs[ch];
                    let normalized_val = (val + 1.0) / 2.0;
                    ui.add(egui::ProgressBar::new(normalized_val as f32).text(format!("{:.4}", val)).desired_width(220.0));
                    if ch == 1 { ui.small("(Клапан сброса давления)"); } else { ui.label(""); }
                    ui.end_row();
                }
            });
    });
}
