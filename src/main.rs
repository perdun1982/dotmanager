mod models;

use models::*;

fn main() {
    let package = Package {
        manifest: Manifest {
            name: "example".to_string(),
            version: "0.1.0".to_string(),
            dependencies: Dependencies {
                crates: vec!["serde".to_string(), "tokio".to_string()],
            },
        },
    };

    println!("Package name: {}", package.manifest.name);
    println!("Package version: {}", package.manifest.version);
    println!("Dependencies: {:?}", package.manifest.dependencies.crates);
}
