use eframe::egui;
use crate::cpu::DneprCPU;

pub fn draw_modes(ui: &mut egui::Ui, cpu: &mut DneprCPU) {
    ui.vertical(|ui| {
        ui.label(egui::RichText::new("Режимы ЭВМ").strong());
        egui::Grid::new("left_switches").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
            ui.small("Автомат:"); ui.checkbox(&mut cpu.is_running, ""); ui.end_row();
            ui.small("По шагам:");
            let mut step_mode = !cpu.is_running;
            if ui.checkbox(&mut step_mode, "").changed() {
                cpu.is_running = !step_mode;
            };
            ui.end_row();
            ui.small("Ост.Адр:");  let mut mock1 = false; ui.checkbox(&mut mock1, ""); ui.end_row();
            ui.small("Блок.ЗП:");  let mut mock2 = false; ui.checkbox(&mut mock2, ""); ui.end_row();
            ui.small("Питание:");  let mut mock3 = true;  ui.checkbox(&mut mock3, ""); ui.end_row();
        });
    });
}
