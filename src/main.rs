mod models;
use models::*;
use std::fs;

fn main() {

    let content = fs::read_to_string("rice.toml").expect("Не удалось прочитать rice.toml");
    let manifest: Manifest = toml::from_str(&content).expect("Ошибка в формате TOML");

    println!("Найдено пакетов: {}", manifest.dependencies.packages.len());

    manifest.install_all();
    manifest.apply_links();
}
