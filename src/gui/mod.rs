mod panel;
mod ram;

use eframe::egui;
use crate::cpu::{DneprCPU, MEMORY_SIZE};
use crate::asm::Assembler;
use std::fs;

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

    fn open_and_load_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Ассемблер Днепр", &["asm", "txt"])
            .pick_file()
        {
            self.cpu.log_message(format!("[Система] Чтение файла: {}", path.display()));
            match fs::read_to_string(&path) {
                Ok(asm_code) => {
                    match Assembler::compile(&asm_code) {
                        Ok(binary_program) => {
                            self.cpu.accumulator = 0;
                            self.cpu.program_counter = 0;
                            self.cpu.cycles = 0;
                            self.cpu.is_running = false;
                            self.cpu.memory = [0; MEMORY_SIZE];

                            for (i, &instruction) in binary_program.iter().enumerate() {
                                if i < MEMORY_SIZE {
                                    self.cpu.memory[i] = instruction;
                                }
                            }
                            self.cpu.log_message("[Система] Программа успешно скомпилирована и загружена в ОЗУ.".to_string());
                        }
                        Err(e) => {
                            self.cpu.log_message(format!("[Ошибка Ассемблера] {:?}", e));
                        }
                    }
                }
                Err(e) => {
                    self.cpu.log_message(format!("[Ошибка Файла] Не удалось прочесть: {}", e));
                }
            }
        }
    }
}

impl eframe::App for DneprGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.cpu.is_running {
            self.cpu.step();
            ctx.request_repaint();
        }

        // 1. НИЖНЯЯ ПАНЕЛЬ: Терминал логирования (Консоль ЭВМ)
        egui::TopBottomPanel::bottom("log_panel")
            .resizable(true)
            .default_height(140.0)
            .min_height(80.0)
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("📟 Системный журнал вывода ЭВМ (Лог):").strong());
                ui.add_space(2.0);

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .stick_to_bottom(true) // Принудительный скролл вниз при появлении новых данных
                    .show(ui, |ui| {
                        // Отрисовываем логи в виде терминальной сетки моноширинным шрифтом
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

                            // Фокусируем скролл на самой последней добавленной строчке
                            if idx == self.cpu.logs.len() - 1 {
                                label.scroll_to_me(Some(egui::Align::BOTTOM));
                            }
                        }
                    });
            });

        // 2. ПРАВАЯ ПАНЕЛЬ: ОЗУ
        egui::SidePanel::right("ram_panel")
            .resizable(true)
            .default_width(500.0)
            .min_width(400.0)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ram::draw_ram_table(ui, &self.cpu);
                });
            });

        // 3. ЦЕНТРАЛЬНАЯ ПАНЕЛЬ: Органы управления
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
                panel::draw_control_panel(ui, &mut self.cpu, &mut self.input_switches);
            });
        });
    }
}
