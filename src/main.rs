mod types;
mod uso;
mod cpu;
mod asm;
mod scenario;

use cpu::DneprCPU;
use asm::Assembler;
use scenario::Scenario;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Собираем все аргументы командной строки в вектор строк
    let args: Vec<String> = env::args().collect();

    // Первый аргумент (args[0]) — это всегда путь к самому бинарнику.
    // Нам нужен второй аргумент (args[1]) с именем файла ассемблера.
    if args.len() < 2 {
        eprintln!("Ошибка: Не указан файл программы.");
        eprintln!("Использование: cargo run -- <имя_файла.asm>");
        return;
    }

    // Извлекаем имя файла из аргументов
    let asm_path_str = &args[1];
    let asm_path = Path::new(asm_path_str);

    // Автоматически формируем путь к файлу сценария с расширением .sco
    let scenario_path = asm_path.with_extension("sco");

    // 1. Читаем и компилируем ассемблер
    println!("--- Чтение файла исходного кода: {} ---", asm_path.display());
    let asm_code = match fs::read_to_string(asm_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Ошибка чтения ассемблера: {}", e);
            return;
        }
    };

    let binary_program = match Assembler::compile(&asm_code) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Ошибка компиляции: {:?}", e);
            return;
        }
    };

    // 2. Инициализация сценария и флага его наличия
    let mut has_scenario = false;

    println!("--- Автоматический поиск файла сценария: {} ---", scenario_path.display());
    let scenario = match Scenario::load(&scenario_path) {
        Ok(Some(scen)) => {
            has_scenario = true;
            println!("[Режим внешней среды]: Сценарий найден и активирован.");
            scen
        }
        Ok(None) => {
            has_scenario = false;
            println!("[Режим внешней среды]: Файл .sco отсутствует. Симуляция без динамического сценария.");
            Scenario::default()
        }
        Err(e) => {
            eprintln!("Критическая ошибка синтаксиса в файле сценария: {}", e);
            return;
        }
    };

    let mut cpu = DneprCPU::new();

    // Фоновые значения АЦП по умолчанию
    cpu.uso.adc_inputs[3] = 0.65;

    // Загружаем бинарник в ОЗУ
    for (i, &instruction) in binary_program.iter().enumerate() {
        if i < cpu::MEMORY_SIZE {
            cpu.memory[i] = instruction;
        }
    }

    cpu.is_running = true;
    let mut iteration = 1;
    println!("--- Запуск технологического цикла ЭВМ 'Днепр' ---");

    while cpu.is_running {
        print!("#{}: ", iteration);

        // Обрабатываем события сценария только если флаг наличия равен true
        if has_scenario {
            scenario.update_environment(iteration, &mut cpu.uso);
        }

        cpu.step();
        iteration += 1;

        if iteration > 30 {
            println!("[Симулятор] Превышен лимит итераций защиты.");
            break;
        }
    }

    println!("\n--- Программа успешно завершила работу ---");
    cpu.print_dump();
}
