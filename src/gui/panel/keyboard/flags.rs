use eframe::egui;
use crate::cpu::DneprCPU;

pub fn draw_flags(ui: &mut egui::Ui, cpu: &mut DneprCPU) {
    ui.vertical(|ui| {
        ui.label(egui::RichText::new("Признаки").strong());
        egui::Grid::new("right_switches").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
            for i in 0..5 {
                ui.small(format!("Ключ П{}:", i));
                ui.checkbox(&mut cpu.program_switches[i], "");
                ui.end_row();
            }
        });
    });
}
