mod models;
use models::Package;
fn main() {
    let mut my_pkg = Package {
        name: String::from("waybar"),
        version: 12,
        is_installed: false,
    };

    println!("--- Тест 1: Обновление неустановленного пакета ---");
    my_pkg.upgrade(); // Ожидаем ошибку

    println!("\n--- Тест 2: Установка ---");
    my_pkg.install(); // Ожидаем успех

    println!("\n--- Тест 3: Обновление установленного пакета ---");
    my_pkg.upgrade(); // Ожидаем успех и версию 13

    println!("\n--- Тест 4: Повторная установка ---");
    my_pkg.install(); // Ожидаем сообщение, что уже стоит
}
