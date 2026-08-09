mod types;
mod uso;
mod cpu;
mod asm;
mod scenario;
mod gui;

use cpu::DneprCPU;
use asm::Assembler;
use scenario::Scenario;
use gui::DneprGuiApp;
use eframe::egui;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    // Проверяем, затребован ли графический режим
    if args[1].to_lowercase() == "gui" {
        println!("--- Запуск ЭВМ «Днепр» в графическом режиме (GUI) ---");

        // Настраиваем параметры отображения окна и размеры вьюпорта
        let mut native_options = eframe::NativeOptions::default();
        native_options.viewport = egui::ViewportBuilder::default()
            .with_title("Пульт контроля и управления ЭВМ «Днепр»")
            .with_inner_size(egui::vec2(1800.0, 1000.0)) // Задаем ширину 1024 и высоту 600
            .with_min_inner_size(egui::vec2(800.0, 500.0)); // Ограничиваем минимальное сжатие окна

        if let Err(err) = eframe::run_native(
            "Пульт контроля и управления ЭВМ «Днепр»",
            native_options,
            Box::new(|_cc| Ok(Box::new(DneprGuiApp::new()))),
        ) {
            eprintln!("\n[КРИТИЧЕСКАЯ ОШИБКА GUI]: Не удалось открыть графическое окно!");
            eprintln!("Причина: {:?}", err);
        }
    } else {
        // Консольный режим выполнения конкретного файла ассемблера
        run_console_mode(&args[1]);
    }
}

fn print_usage() {
    eprintln!("Ошибка: Не указаны параметры запуска.");
    eprintln!("Использование:");
    eprintln!("  cargo run -- gui              <- Запуск интерактивного пульта управления");
    eprintln!("  cargo run -- <имя_файла.asm>  <- Компиляция и запуск программы в консоли");
}

fn run_console_mode(asm_path_str: &str) {
    let asm_path = Path::new(asm_path_str);
    let scenario_path = asm_path.with_extension("sco");

    println!("--- Чтение файла исходного кода: {} ---", asm_path.display());
    let asm_code = match fs::read_to_string(asm_path) {
        Ok(code) => code,
        Err(e) => { eprintln!("Ошибка чтения ассемблера: {}", e); return; }
    };

    let binary_program = match Assembler::compile(&asm_code) {
        Ok(code) => code,
        Err(e) => { eprintln!("Ошибка компиляции: {:?}", e); return; }
    };

    let has_scenario: bool;
    println!("--- Автоматический поиск файла сценария: {} ---", scenario_path.display());
    let scenario = match Scenario::load(&scenario_path) {
        Ok(Some(scen)) => {
            has_scenario = true;
            println!("[Режим внешней среды]: Сценарий найден и активирован.");
            scen
        }
        Ok(None) => {
            has_scenario = false;
            println!("[Режим внешней среды]: Файл .sco отсутствует. Симуляция без сценария.");
            Scenario::default()
        }
        Err(e) => { eprintln!("Критическая ошибка сценария: {}", e); return; }
    };

    let compiled_result = match Assembler::compile(&asm_code) {
        Ok(res) => res,
        Err(e) => { eprintln!("Ошибка компиляции: {:?}", e); return; }
    };

    let mut cpu = DneprCPU::new();
    cpu.uso.adc_inputs = [0.65; 8];

    for (i, &instruction) in compiled_result.binary.iter().enumerate() {
        if i < cpu::MEMORY_SIZE {
            cpu.memory[i] = instruction;
            cpu.memory_is_code[i] = compiled_result.is_code[i];
        }
    }

    cpu.is_running = true;
    let mut iteration = 1;
    println!("--- Запуск технологического цикла ЭВМ 'Днепр' ---");

    while cpu.is_running {
        println!("\n--- Итерация симулятора #{} ---", iteration);
        if has_scenario {
            scenario.update_environment(iteration, &mut cpu.uso);
        }
        cpu.step();
        iteration += 1;
        if iteration > 100 {
            println!("[Симулятор] Превышен лимит итераций защиты.");
            break;
        }
    }

    println!("\n--- Программа успешно завершила работу ---");
    cpu.print_dump();
}
