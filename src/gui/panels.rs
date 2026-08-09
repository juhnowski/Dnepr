use eframe::egui;
use crate::gui::DneprGuiApp;
use crate::gui::ram;
use crate::gui::panel;

impl DneprGuiApp {
    /// Отрисовка нижнего терминала логирования
    pub fn draw_bottom_log_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("log_panel")
            .resizable(true)
            .default_height(140.0)
            .min_height(80.0)
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("📟 Системный журнал вывода ЭВМ (Лог):").strong());
                ui.add_space(2.0);

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for (idx, log_line) in self.cpu.logs.iter().enumerate() {
                            let text_color = if log_line.contains("[Ошибка") {
                                egui::Color32::LIGHT_RED
                            } else if log_line.contains("[УСО]") {
                                egui::Color32::LIGHT_GREEN
                            } else if log_line.contains("[Система]") {
                                egui::Color32::LIGHT_YELLOW
                            } else {
                                egui::Color32::WHITE
                            };

                            let label = ui.label(egui::RichText::new(log_line).monospace().color(text_color));
                            if idx == self.cpu.logs.len() - 1 {
                                label.scroll_to_me(Some(egui::Align::BOTTOM));
                            }
                        }
                    });
            });
    }

    /// Отрисовка правой панели ОЗУ
    pub fn draw_right_ram_panel(&self, ctx: &egui::Context) {
        egui::SidePanel::right("ram_panel")
            .resizable(true)
            .default_width(500.0)
            .min_width(400.0)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ram::draw_ram_table(ui, &self.cpu);
                });
            });
    }

    /// Отрисовка центрального пульта управления
    pub fn draw_central_control_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("💻 Пульт контроля и управления ЭВМ «Днепр» (1961 г.)");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("📂 Открыть .asm программу").clicked() {
                        self.open_and_load_file();
                    }
                });
            });
            ui.add_space(10.0);

            ui.vertical_centered_justified(|ui| {
                panel::draw_control_panel(
                    ui,
                    &mut self.cpu,
                    &mut self.input_switches_a1,
                    &mut self.input_switches_a2
                );
            });
        });
    }
}
