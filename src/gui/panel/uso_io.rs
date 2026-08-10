use eframe::egui;
use crate::cpu::DneprCPU;
use crate::uso::{ADC_CHANNELS, DAC_CHANNELS};

pub fn draw_uso_io(ui: &mut egui::Ui, cpu: &mut DneprCPU) {
    // БЛОК: ИНТЕРАКТИВНЫЕ ПОЛЗУНКИ АЦП УСО
    ui.group(|ui| {
        ui.label("📡 Датчики технологического объекта (Входы АЦП УСО):");
        ui.add_space(5.0);
        for ch in 0..ADC_CHANNELS {
            ui.horizontal(|ui| {
                let is_active = cpu.uso.selected_channel == ch;
                let label_text = if is_active {
                    egui::RichText::new(format!("➡ Канал #{}:", ch)).color(egui::Color32::LIGHT_BLUE).strong()
                } else {
                    egui::RichText::new(format!("  Канал #{}:", ch))
                };
                ui.label(label_text);
                ui.add(egui::Slider::new(&mut cpu.uso.adc_inputs[ch], -1.0..=0.999999).text(if ch == 3 { "(Датчик давления)" } else { "" }));
            });
        }
    });

    ui.add_space(10.0);

    // БЛОК: ВЫХОДНЫЕ СИГНАЛЫ ЦАП УСО (Двунаправленные шкалы)
    ui.group(|ui| {
        ui.label("⚙️ Исполнительные механизмы (Выходы ЦАП УСО):");
        ui.add_space(5.0);

        egui::Grid::new("dac_grid")
            .num_columns(3)
            .spacing([10.0, 8.0])
            .show(ui, |ui| {
                for ch in 0..DAC_CHANNELS {
                    ui.label(format!("  Выход ЦАП #{}:", ch));

                    let val = cpu.uso.dac_outputs[ch];

                    // Резервируем жестко заданную прямоугольную область под кастомный индикатор
                    let desired_size = egui::vec2(220.0, 20.0);
                    let (rect, _response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

                    // Отрисовка подложки (фона) шкалы
                    let background_color = ui.visuals().extreme_bg_color;
                    ui.painter().rect_filled(rect, 3.0, background_color);

                    // Рисуем центральную осевую линию (отметку нуля)
                    let center_x = rect.left() + rect.width() / 2.0;
                    ui.painter().line_segment(
                        [egui::pos2(center_x, rect.top()), egui::pos2(center_x, rect.bottom())],
                        ui.visuals().widgets.noninteractive.bg_stroke
                    );

                    // Переводим значение в f32 для вычислений геометрии egui
                    let val_f32 = val as f32;
                    let half_width = rect.width() / 2.0;

                    // Расчет заполнения шкалы в зависимости от знака числа
                    if val_f32 > 0.0 {
                        // Положительное значение: заполняем вправо от центра
                        let bar_width = val_f32.min(1.0) * half_width;
                        let bar_rect = egui::Rect::from_min_max(
                            egui::pos2(center_x, rect.top() + 2.0),
                            egui::pos2(center_x + bar_width, rect.bottom() - 2.0)
                        );
                        ui.painter().rect_filled(bar_rect, 1.0, egui::Color32::from_rgb(100, 180, 255)); // Синий
                    } else if val_f32 < 0.0 {
                        // Отрицательное значение: заполняем строго ВЛЕВО от центра
                        let bar_width = val_f32.abs().min(1.0) * half_width;
                        let bar_rect = egui::Rect::from_min_max(
                            egui::pos2(center_x - bar_width, rect.top() + 2.0), // Левая точка смещается влево
                            egui::pos2(center_x, rect.bottom() - 2.0)          // Правая точка — это центр (ось нуля)
                        );
                        ui.painter().rect_filled(bar_rect, 1.0, egui::Color32::from_rgb(255, 140, 60)); // Оранжевый
                    }

                    // Поверх шкалы выводим числовое значение по центру
                    let text_color = ui.visuals().widgets.noninteractive.text_color();
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        format!("{:.4}", val),
                        egui::FontId::monospace(12.0),
                        text_color
                    );

                    // Описание канала
                    if ch == 1 {
                        ui.small("(Клапан сброса давления)");
                    } else {
                        ui.label("");
                    }
                    ui.end_row();
                }
            });
    });
}
