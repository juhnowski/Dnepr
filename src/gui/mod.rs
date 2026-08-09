mod panel;
mod ram;
mod file_loader;
mod panels;

use eframe::egui;
use crate::cpu::DneprCPU;

pub struct DneprGuiApp {
    pub cpu: DneprCPU,
    pub input_switches_a1: [bool; 26],
    pub input_switches_a2: [bool; 26],
}

impl DneprGuiApp {
    pub fn new() -> Self {
        let mut cpu = DneprCPU::new();
        cpu.uso.adc_inputs = [0.65; 8];
        Self {
            cpu,
            input_switches_a1: [false; 26],
            input_switches_a2: [false; 26],
        }
    }
}

impl eframe::App for DneprGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Симуляция частоты работы автомата
        if self.cpu.is_running {
            for _ in 0..5 {
                // Если внутри step() выполнилась команда HALT, она сама сбросит is_running в false
                if !self.cpu.is_running { break; }
                self.cpu.step();
            }
            ctx.request_repaint_after(std::time::Duration::from_millis(10));
        }

        // Отрисовка изолированных панелей, вынесенных в panels.rs
        self.draw_bottom_log_panel(ctx);
        self.draw_right_ram_panel(ctx);
        self.draw_central_control_panel(ctx);
    }
}
