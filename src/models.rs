use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub dependencies: Dependencies,
    pub links: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct Dependencies {
    pub packages: Vec<String>,
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: u32,
    pub is_installed: bool,
} // <- Здесь структура Package ЗАКОНЧИЛАСЬ

// Открываем блок impl Package
impl Package {

    pub fn install(&mut self) {
        if !self.is_installed {
            self.is_installed = true;
            println!("Пакет {} установлен", self.name);
        } else {
            println!("Пакет {} уже установлен", self.name);
        }
    } // <- Здесь метод install ЗАКОНЧИЛСЯ (но impl Package еще открыт!)

    // Второй метод должен быть здесь, ВНУТРИ того же impl
    pub fn upgrade(&mut self) {
        if self.is_installed {
            self.version += 1;
            println!("Пакет {} обновлен до версии {}", self.name, self.version);
        } else {
            println!("Пакет {} не обновлен, ошибка", self.name);
        }
    }

} // <- Вот здесь блок impl Package ЗАКОНЧИЛСЯ. И только тут!
