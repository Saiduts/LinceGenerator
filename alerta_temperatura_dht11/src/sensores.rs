// src/sensores.rs
//
// Inicialización de los sensores configurados. Cada función envuelve
// la creación del sensor y traduce errores de inicialización en un
// panic descriptivo, para que un fallo de hardware se note de inmediato
// al arrancar en vez de fallar silenciosamente más adelante.

use crate::config::AppConfig;

use lince::devices::sensors::dht11::Dht11Sensor;

/// Conjunto de sensores activos en esta aplicación.
///
/// Agrupar los sensores en una sola struct evita pasar cada uno por
/// separado entre funciones del módulo `app`.
pub struct Sensores {
    pub dht11: Dht11Sensor,
}

impl Sensores {
    /// Inicializa todos los sensores configurados a partir de `AppConfig`.
    pub fn inicializar(cfg: &AppConfig) -> Self {
        Self {
            dht11: Dht11Sensor::new(cfg.dht11.pin)
                .expect("No se pudo inicializar DHT11"),
        }
    }
}