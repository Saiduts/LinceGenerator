// src/config.rs
//
// Define la estructura de configuración de la aplicación y la carga
// desde `config.toml` en tiempo de ejecución. Esto permite cambiar
// pines, broker o intervalos sin recompilar el binario.

use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub mqtt: MqttConfig,
    pub app: RuntimeConfig,
    pub mhrd: MhRdConfig,
}

#[derive(Debug, Deserialize)]
pub struct MqttConfig {
    pub broker:    String,
    pub port:      u16,
    pub topic:     String,
    pub client_id: String,
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
    pub interval_secs: u64,
    pub db_path: String,
}

#[derive(Debug, Deserialize)]
pub struct MhRdConfig {
    pub enabled: bool,
    pub pin: u8,
}

impl AppConfig {
    /// Carga la configuración desde la ruta indicada (normalmente "config.toml").
    pub fn load(path: &str) -> Self {
        let contenido = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("No se pudo leer '{}': {}", path, e));

        toml::from_str(&contenido)
            .unwrap_or_else(|e| panic!("Error parseando '{}': {}", path, e))
    }
}