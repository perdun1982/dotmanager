pub struct Manifest {
    pub name: String,
    pub version: String,
    pub dependencies: Dependencies,
}

pub struct Dependencies {
    pub crates: Vec<String>,
}

pub struct Package {
    pub manifest: Manifest,
}
