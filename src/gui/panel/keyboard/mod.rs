mod modes;
mod bous;
mod flags;

use eframe::egui;
use crate::cpu::DneprCPU;

pub fn draw_keyboard(
    ui: &mut egui::Ui,
    cpu: &mut DneprCPU,
    input_switches_a1: &mut [bool; 26],
    input_switches_a2: &mut [bool; 26]
) {
    ui.group(|ui| {
        ui.label("🎛️ Панель Блоков органов ручного управления и сигнализации (БОУС):");
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            // 1. Отрисовка левых тумблеров режимов
            modes::draw_modes(ui, cpu);

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);

            // 2. Отрисовка центральных регистров БОУС-1 и БОУС-2
            bous::draw_bous_modules(ui, cpu, input_switches_a1, input_switches_a2);

            ui.add_space(15.0);
            ui.separator();
            ui.add_space(15.0);

            // 3. Отрисовка правых тумблеров условий
            flags::draw_flags(ui, cpu);
        });
    });
}
