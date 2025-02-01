use std::env;
use std::env::VarError;

pub struct Environment {
    server_host: String,
    server_port: String,
    database_url: String,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            server_host: "127.0.0.1".to_string(),
            server_port: "8080".to_string(),
            database_url: "sqlite::memory:".to_string(),
        }
    }
}

impl Environment {
    pub fn server_url(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    pub fn database_url(&self) -> String {
        (&self.database_url).to_string()
    }
}

pub fn load_env() -> Environment {
    try_load_env().unwrap_or_default()
}

fn try_load_env() -> Result<Environment, VarError> {
    let server_host = env::var("SERVER_HOST")?;
    let server_port = env::var("SERVER_PORT")?;
    let database_url = env::var("DATABASE_URL")?;
    Ok(Environment {
        server_host,
        server_port,
        database_url,
    })
}
