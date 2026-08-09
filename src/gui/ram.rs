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
                            ui.colored_label(egui::Color32::LIGHT_BLUE, "⬅ NEXT OP");
                        } else if val != 0 {
                            // Проверяем по карте типов: код это или данные
                            if cpu.memory_is_code[addr] {
                                let op = (val >> 20) & 0x3F;
                                let mnemonic = match op {
                                    0x01 => "OP: STORE",
                                    0x02 => "OP: ADD",
                                    0x03 => "OP: SUB",
                                    0x04 => "OP: JUMP",
                                    0x05 => "OP: JZ",
                                    0x06 => "OP: MULT",
                                    0x07 => "OP: JPS",
                                    0x08 => "OP: SHL",
                                    0x09 => "OP: SHR",
                                    0x10 => "OP: SEL_CH",
                                    0x11 => "OP: READ_ADC",
                                    0x12 => "OP: WRITE_DAC",
                                    _ => "OP: UNK",
                                };
                                ui.colored_label(egui::Color32::from_rgb(255, 180, 100), mnemonic);
                            } else {
                                // Любой ручной ввод или директива DATA отображаются исключительно как данные!
                                ui.colored_label(egui::Color32::GREEN, "DATA");
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
