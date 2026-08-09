use eframe::egui;
use crate::cpu::{DneprCPU, MEMORY_SIZE};
use crate::types::DneprWord;

pub fn draw_ram_table(ui: &mut egui::Ui, cpu: &DneprCPU) {
    ui.group(|ui| {
        ui.label("💾 Оперативная память (Ферритовые сердечники ОЗУ):");
        ui.add_space(5.0);

        egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
            egui::Grid::new("ram_grid")
                .striped(true)
                .num_columns(4)
                .spacing([15.0, 4.0])
                .show(ui, |ui| {
                    // Используем .strong() вместо .bold()
                    ui.label(egui::RichText::new("Ячейка").strong());
                    ui.label(egui::RichText::new("HEX Код").strong());
                    ui.label(egui::RichText::new("Вещественное").strong());
                    ui.label(egui::RichText::new("Статус").strong());
                    ui.end_row();

                    for addr in 0..MEMORY_SIZE {
                        let val = cpu.memory[addr];
                        let word = DneprWord(val);
                        let is_current_pc = cpu.program_counter == addr;

                        let text_color = if is_current_pc {
                            egui::Color32::LIGHT_BLUE
                        } else if val == 0 {
                            egui::Color32::GRAY
                        } else {
                            egui::Color32::WHITE
                        };

                        ui.colored_label(text_color, format!("[{:03}]", addr));
                        ui.colored_label(text_color, format!("{:#010X}", val));
                        ui.colored_label(text_color, format!("{:>9.6}", word.to_float()));

                        if is_current_pc {
                            ui.colored_label(egui::Color32::LIGHT_BLUE, "⬅ NEXT OP");
                        } else if val != 0 {
                            ui.colored_label(egui::Color32::GREEN, "DATA");
                        } else {
                            ui.label("");
                        }

                        ui.end_row();
                    }
                });
        });
    });
}
