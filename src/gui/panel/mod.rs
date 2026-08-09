mod keyboard;
mod uso_io;
mod indicators;

use eframe::egui;
use crate::cpu::DneprCPU;

pub fn draw_control_panel(
    ui: &mut egui::Ui,
    cpu: &mut DneprCPU,
    input_switches_a1: &mut [bool; 26],
    input_switches_a2: &mut [bool; 26]
) {
    // 1. Рисуем лампы ACC и системные кнопки
    indicators::draw_indicators_and_buttons(ui, cpu);
    ui.add_space(10.0);

    // 2. Рисуем историческое трехсекционное клавишное поле с раздельным вводом адресов
    keyboard::draw_keyboard(ui, cpu, input_switches_a1, input_switches_a2);
    ui.add_space(10.0);

    // 3. Рисуем ползунки АЦП и индикаторы ЦАП УСО
    uso_io::draw_uso_io(ui, cpu);
    ui.add_space(10.0);

    // 4. Общая телеметрия процессора внизу панели
    ui.label(format!("Счетчик команд (PC): {}", cpu.program_counter));
    ui.label(format!("Всего выполнено тактов: {}", cpu.cycles));
    ui.label(format!("Виртуальное время работы: {} мкс", cpu.cycles * 34));
}
