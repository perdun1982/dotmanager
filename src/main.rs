use std::process::Command;
use std::path::Path;

// Функция установки пакета
fn install_package(package_name: &str) {
    // Проверяем, есть ли папка /etc/nixos (значит, это NixOS)
    if Path::new("/etc/nixos").exists() {
        println!("[NixOS] Устанавливаю {}...", package_name);

        // Запускаем: nix profile install nixpkgs#имя_пакета
        let status = Command::new("nix")
            .arg("profile")
            .arg("search")
            .arg(format!("nixpkgs#{}", package_name))
            .status() // Выполняем и ждем завершения
            .expect("Не удалось запустить nix");

        if status.success() {
            println!("✅ Пакет {} успешно установлен!", package_name);
        } else {
            println!("❌ Ошибка при установке {}", package_name);
        }
    } else {
        println!("[Неизвестная ОС] Здесь я пока не умею ставить пакеты. Пакет: {}", package_name);
    }
}
// 1. Подключаем библиотеки (как #include в C)
use serde::Deserialize;
use std::fs;

// 2. Описываем структуру файла (как struct в C)
// #[derive(Deserialize)] — это магия, которая говорит компилятору:
// "Сгенерируй код, который сам разберет TOML в эту структуру".
#[derive(Deserialize, Debug)]
struct Manifest {
    dependencies: Dependencies,
}
#[derive(Deserialize, Debug)]
struct Package{
    name: String,
    version: u32,
    is_installed: bool,
}
#[derive(Deserialize, Debug)]
struct Dependencies {
    // Vec<String> — это как динамический массив в C,
    // в котором лежат строки.
    packages: Vec<String>,
}

// 3. Главная функция
fn main() {
    // Читаем файл в строку.
    // В C ты бы открывал FILE* и читал через fgets.
    // В Rust fs::read_to_string делает всё сразу.
    let content = fs::read_to_string("rice.toml")
        .expect("Не удалось прочитать файл rice.toml");
        // expect — это как if (file == NULL) { printf("Ошибка"); exit(1); }

    // Парсим текст TOML в нашу структуру
    let manifest: Manifest = toml::from_str(&content)
        .expect("Ошибка в формате TOML");

    // Выводим результат на экран.
    // {:?} — это спецсимвол для вывода структур (отладка)
    // Проходим по каждому пакету из манифеста
    for package in &manifest.dependencies.packages {
        install_package(package); // Вызываем функцию установки
    }
let mut my_pkg = Package{
    name: String::from("waybar"),
    version: 12,
    is_installed: false,
};
println!("До изменений: {:?}", my_pkg);
my_pkg.is_installed = true;
println!("После: {:?}", my_pkg)
}
