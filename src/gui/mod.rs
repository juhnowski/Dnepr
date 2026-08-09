mod panel;
mod ram;

use eframe::egui;
use crate::cpu::DneprCPU;

pub struct DneprGuiApp {
    cpu: DneprCPU,
    input_switches: [bool; 26],
}

impl DneprGuiApp {
    pub fn new() -> Self {
        let mut cpu = DneprCPU::new();
        cpu.uso.adc_inputs = [0.65; 8];
        Self {
            cpu,
            input_switches: [false; 26],
        }
    }
}

impl eframe::App for DneprGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.cpu.is_running {
            self.cpu.step();
            ctx.request_repaint();
        }

        // 1. Выносим ОЗУ в фиксированную правую панель, которая будет занимать 500 пикселей
        egui::SidePanel::right("ram_panel")
            .resizable(true)
            .default_width(500.0)
            .min_width(400.0)
            .show(ctx, |ui| {
                // Растягиваем внутреннее содержимое на всю ширину панели
                ui.vertical_centered_justified(|ui| {
                    ram::draw_ram_table(ui, &self.cpu);
                });
            });

        // 2. Всё оставшееся место слева автоматически займет пульт управления
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("💻 Пульт контроля и управления ЭВМ «Днепр» (1961 г.)");
            ui.add_space(10.0);

            // Отрисовываем пульт управления, растягивая его блоки
            ui.vertical_centered_justified(|ui| {
                panel::draw_control_panel(ui, &mut self.cpu, &mut self.input_switches);
            });
        });
    }
}
