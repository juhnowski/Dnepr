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

    /// Внутренний метод для открытия диалогового окна и загрузки .asm файла
    fn open_and_load_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Ассемблер Днепр", &["asm", "txt"])
            .pick_file()
        {
            println!("--- Выбран файл для загрузки: {} ---", path.display());
            match fs::read_to_string(&path) {
                Ok(asm_code) => {
                    match Assembler::compile(&asm_code) {
                        Ok(binary_program) => {
                            // Полный сброс процессора перед загрузкой новой программы
                            self.cpu.accumulator = 0;
                            self.cpu.program_counter = 0;
                            self.cpu.cycles = 0;
                            self.cpu.is_running = false;
                            self.cpu.memory = [0; MEMORY_SIZE];

                            // Запись бинарного кода в память
                            for (i, &instruction) in binary_program.iter().enumerate() {
                                if i < MEMORY_SIZE {
                                    self.cpu.memory[i] = instruction;
                                }
                            }
                            println!("[ОК] Программа успешно скомпилирована и загружена в ОЗУ.");
                        }
                        Err(e) => {
                            eprintln!("[Ошибка компиляции]: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[Ошибка чтения файла]: {}", e);
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

        egui::SidePanel::right("ram_panel")
            .resizable(true)
            .default_width(500.0)
            .min_width(400.0)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ram::draw_ram_table(ui, &self.cpu);
                });
            });

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
