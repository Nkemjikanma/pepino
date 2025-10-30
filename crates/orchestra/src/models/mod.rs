#[derive(Debug)]
pub enum SQLXFlavour {
    PostgreSQL { docker_compose: bool },
    SQLite,
}

#[derive(Debug)]
pub enum DatabaseLayer {
    Sqlx(SQLXFlavour),
    // Diesel,
}

#[derive(Debug)]
pub struct Choices {
    pub project_name: String,
    pub backend: BackendFramework,
    pub database: DatabaseLayer,
}

#[derive(Debug)]
pub enum BackendFramework {
    Axum,
    // ActixWeb,
}

#[derive(Debug)]
pub enum PepinoProcess {
    Create { choices: Choices },
    Dev { path: Option<String> },
    Build(BuildProcess),
}

#[derive(Debug)]
pub enum BuildProcess {
    Frontend { release: bool },
    Backend { target: Option<String> },
}
