use eframe::egui;
use crate::cpu::DneprCPU;

pub fn draw_keyboard(ui: &mut egui::Ui, cpu: &mut DneprCPU, input_switches: &mut [bool; 26]) {
    ui.group(|ui| {
        ui.label("🎛️ Главный пульт контроля и управления (ПКУ):");
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            // ПОЛЕ 1 (ЛЕВОЕ): Тумблеры режимов работы
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Режимы ЭВМ").strong());
                egui::Grid::new("left_switches").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                    ui.small("Автомат:"); ui.checkbox(&mut cpu.is_running, ""); ui.end_row();
                    ui.small("По шагам:"); let mut step_mode = !cpu.is_running; if ui.checkbox(&mut step_mode, "").changed() { cpu.is_running = !step_mode; }; ui.end_row();
                    ui.small("Ост.Адр:");  let mut mock1 = false; ui.checkbox(&mut mock1, ""); ui.end_row();
                    ui.small("Блок.ЗП:");  let mut mock2 = false; ui.checkbox(&mut mock2, ""); ui.end_row();
                    ui.small("Питание:");  let mut mock3 = true;  ui.checkbox(&mut mock3, ""); ui.end_row();
                });
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // ПОЛЕ 2 (ЦЕНТРАЛЬНОЕ): Клавишное поле регистра ввода (26 цветных кнопок)
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Регистр ввода данных (26 бит)").strong());
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    for bit in (0..26).rev() {
                        let btn_color = if bit >= 18 {
                            if input_switches[bit] { egui::Color32::from_rgb(200, 50, 50) } else { egui::Color32::from_rgb(140, 30, 30) }
                        } else if bit >= 9 {
                            if input_switches[bit] { egui::Color32::from_rgb(230, 230, 210) } else { egui::Color32::from_rgb(180, 180, 160) }
                        } else {
                            if input_switches[bit] { egui::Color32::from_rgb(70, 70, 70) } else { egui::Color32::from_rgb(25, 25, 25) }
                        };

                        let text_color = if bit >= 9 && bit < 18 { egui::Color32::BLACK } else { egui::Color32::WHITE };

                        let btn_text = format!("{}", bit);
                        let btn = egui::Button::new(egui::RichText::new(btn_text).small().color(text_color))
                            .fill(btn_color)
                            .min_size(egui::vec2(22.0, 30.0));

                        if ui.add(btn).clicked() {
                            input_switches[bit] = !input_switches[bit];
                        }

                        if bit % 3 == 0 && bit != 0 {
                            ui.add_space(5.0);
                        }
                    }
                });

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    let mut entered_value: u32 = 0;
                    for bit in 0..26 {
                        if input_switches[bit] { entered_value |= 1 << bit; }
                    }

                    if ui.button("📥 Записать клавиши в ACC").clicked() {
                        cpu.accumulator = entered_value;
                        cpu.log_message(format!("[Пульт] Данные с клавишного регистра ({:#010X}) записаны в ACC", entered_value));
                    }
                    if ui.button("🗑️ Сбросить клавиши").clicked() {
                        *input_switches = [false; 26];
                    }
                });
            });

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(20.0);

            // ПОЛЕ 3 (ПРАВОЕ): Тумблеры условий программных признаков
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Признаки / Флаги").strong());
                egui::Grid::new("right_switches").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                    for i in 0..5 {
                        ui.small(format!("Тумблер П{}:", i));
                        ui.checkbox(&mut cpu.program_switches[i], "");
                        ui.end_row();
                    }
                });
            });
        });
    });
}
