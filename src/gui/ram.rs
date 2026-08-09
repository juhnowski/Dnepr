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
                            ui.colored_label(egui::Color32::LIGHT_BLUE, "⬅ СЛЕД_КОМ");
                        } else if val != 0 {
                            if cpu.memory_is_code[addr] {
                                let op = (val >> 20) & 0x3F;
                                let mnemonic = match op {
                                    0x00 => "КОМ: ОСТ",
                                    0x01 => "КОМ: ЗП",
                                    0x02 => "КОМ: СЛ",
                                    0x03 => "КОМ: ВЫЧ",
                                    0x04 => "КОМ: БП",
                                    0x05 => "КОМ: ПЗ",
                                    0x06 => "КОМ: УМН",
                                    0x07 => "КОМ: ПК",
                                    0x08 => "КОМ: СДЛ",
                                    0x09 => "КОМ: СДП",
                                    0x10 => "КОМ: ВК",
                                    0x11 => "КОМ: ЧТ_АЦП",
                                    0x12 => "КОМ: ЗП_ЦАП",
                                    _ => "КОМ: ???",
                                };
                                ui.colored_label(egui::Color32::from_rgb(255, 180, 100), mnemonic);
                            } else {
                                ui.colored_label(egui::Color32::from_rgb(100, 255, 100), "ДАННЫЕ");
                            }
                        } else {
                            ui.label("");
                        }
                        ui.end_row();

                    }
                });
        });

        // НОВЫЙ БЛОК: Вывод текущего значения PC прямо под таблицей памяти
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label("📍 Текущий указатель адреса ЭВМ (Счетчик команд / PC):");
            ui.colored_label(
                egui::Color32::LIGHT_BLUE,
                egui::RichText::new(format!(" [{:03}] ", cpu.program_counter)).strong().monospace()
            );
        });
    });
}
