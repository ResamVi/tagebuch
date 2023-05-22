use std::env;

pub struct Config {
    pub secret: String,
    host: String,
    pub port: String,
    database_host: String,
    database_user: String,
    database_password: String,
    database_port: String,
    pub file_path: String,
}

impl Config {
    pub fn from_env() -> Self {
        let secret = match env::var_os("SECRET") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "sesam öffne dich".to_string(),
        };
            
        let host = match env::var_os("HOST") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "0.0.0.0".to_string(),
        };
        let port = match env::var_os("PORT") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "4123".to_string(),
        };

        let diary_path = match env::var_os("DIARY_PATH") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "./diary".to_string(),
        };

        // We switched from a database to the filesystem.
        // All these fields are redundant but kept anyways.

        let database_host = match env::var_os("DB_HOST") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "localhost".to_string(),
        };

        let database_user = match env::var_os("DB_USER") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "postgres".to_string(),
        };

        let database_password = match env::var_os("DB_PASS") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "example".to_string(),
        };

        let database_port = match env::var_os("DB_PORT") {
            Some(val) => val.into_string().unwrap_or("".to_string()),
            None => "5433".to_string(),
        };

        Config {
            secret,
            host,
            port,
            database_host,
            database_user,
            database_password,
            database_port,
            file_path: diary_path,
        }
    }

    /// Print metadata for database connections.
    pub fn connection_string(&self) -> String {
        format!("host={} user={} password={} port={}", 
                self.database_host, 
                self.database_user, 
                self.database_password, 
                self.database_port
        )
    }

    /// URL where a websocket connection can be established to.
    pub fn bind_address(&self) -> String {
        format!("{0}:{1}", self.host, self.port)
    }

    /// Print metadata for filesystem storage.
    pub fn file_path(&self) -> String {
        format!("file_path={}", self.file_path)
    }
}

