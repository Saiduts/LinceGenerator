// src/main.rs
//
// Punto de entrada. Carga la configuración, inicializa cada módulo
// y delega el loop principal a `app::ejecutar`.
//
// Generado por Lince Generator.
// Sensores: mhrd//
// Smart Campus: True  |  Reintento SQLite: True

mod config;
mod sensores;
mod comunicacion;
mod almacenamiento;
mod app;

use config::AppConfig;
use sensores::Sensores;
use comunicacion::Comunicacion;

fn main() {
    let cfg = AppConfig::load("config.toml");

    let sensores = Sensores::inicializar(&cfg);
    let com      = Comunicacion::inicializar(&cfg);
    let storage  = almacenamiento::inicializar(&cfg);

    app::ejecutar(
        &cfg,
        sensores,
        com,
        storage,
    );
}