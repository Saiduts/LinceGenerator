// src/sensores.rs
//
// Inicialización de los sensores configurados. Cada función envuelve
// la creación del sensor y traduce errores de inicialización en un
// panic descriptivo, para que un fallo de hardware se note de inmediato
// al arrancar en vez de fallar silenciosamente más adelante.

use crate::config::AppConfig;

use lince::devices::sensors::mhrd::MhRdSensor;

/// Conjunto de sensores activos en esta aplicación.
///
/// Agrupar los sensores en una sola struct evita pasar cada uno por
/// separado entre funciones del módulo `app`.
pub struct Sensores {
    pub mhrd: MhRdSensor,
}

impl Sensores {
    /// Inicializa todos los sensores configurados a partir de `AppConfig`.
    pub fn inicializar(cfg: &AppConfig) -> Self {
        Self {
            mhrd: MhRdSensor::new(cfg.mhrd.pin, true)
                .expect("No se pudo inicializar MH-RD"),
        }
    }
}