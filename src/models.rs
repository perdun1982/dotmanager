use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::env;
use std::process::Command;
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
impl Manifest {
    pub fn install_all(&self) {
        println!("Начинаю установку пакетов...");

        for pkg_name in &self.dependencies.packages {

            let mut pkg = Package {
                name: pkg_name.clone(),
                version: 1,
                is_installed: false,
            };

            pkg.install();
        }
    }
    pub fn apply_links(&self) {
        println!("Начинаю создание симлинков...");

        let home_dir = env::var("HOME").unwrap_or_else(|_| String::from(""));

        for (source, target) in &self.links {
            let expanded_target = target.replace("~", &home_dir);

            let source_path = fs::canonicalize(source).expect("Не удалось найти исходный файл");
            let target_path = Path::new(&expanded_target);

            if target_path.exists() || target_path.is_symlink() {
                fs::remove_file(target_path).expect("Не удалось удалить старый файл");
            }

            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent).expect("Не удалось создать папки");
            }

            symlink(source_path, target_path).expect("Не удалось создать симлинк");

            println!("✅ Ссылка создана: {} -> {}", source, target);
        }
    }
}
#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: u32,
    pub is_installed: bool,
}

impl Package {

    pub fn install(&mut self) {
        if !self.is_installed {
            println!("Устанавливаю {}...", self.name);

            let status = Command::new("nix")
                .arg("--extra-experimental-features")
                .arg("nix-command flakes")
                .arg("profile")
                .arg("add")
                .arg(format!("nixpkgs#{}", self.name))
                .status()
                .expect("Не удалось запустить команду nix");

            if status.success() {
                self.is_installed = true;
                println!("✅ Пакет {} успешно установлен!", self.name);
            } else {
                println!("❌ Ошибка при установке пакета {}", self.name);
            }

        } else {
            println!("Пакет {} уже установлен", self.name);
        }
    }

    pub fn upgrade(&mut self) {
        if self.is_installed {
            self.version += 1;
            println!("Пакет {} обновлен до версии {}", self.name, self.version);
        } else {
            println!("Пакет {} не обновлен, ошибка", self.name);
        }
    }

}
