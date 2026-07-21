// src/app.rs
//
// Contiene el loop principal de lectura, formateo y envío.
// Separar esto de `main.rs` permite testear o reutilizar la lógica
// sin depender directamente del punto de entrada del binario.

use std::thread;
use std::time::Duration;

use lince::core::traits::sensor::Sensor;
use lince::core::traits::communicator::{Communicator, CommunicatorError};
use lince::core::traits::storage::Storage;
use lince::storage::sqlite::SqliteStorage;
use lince::parser::SensorParser;
use lince::network::mqtt::MqttCommunicator;

use crate::config::AppConfig;
use crate::sensores::Sensores;
use crate::comunicacion::Comunicacion;

/// Ejecuta el loop de lectura indefinidamente.
///
/// Por cada sensor configurado: lee, (opcionalmente) parsea y formatea
/// a Smart Campus, (opcionalmente) guarda en SQLite, e intenta enviar
/// por MQTT. Si el envío falla por desconexión y SQLite está activado,
/// se reintenta la cola de pendientes antes de continuar.
///
/// A diferencia de una aplicación Lince genérica (que publica cada
/// lectura), esta aplicación mantiene el último estado conocido del
/// sensor y solo llama a `enviar(...)` cuando dicho estado cambia.
pub fn ejecutar(
    cfg: &AppConfig,
    mut sensores: Sensores,
    mut com: Comunicacion,
    mut storage: SqliteStorage,
) {
    println!("Lince iniciado — broker: {}:{}", cfg.mqtt.broker, cfg.mqtt.port);
    println!("Topic: {}  |  Intervalo: {}s\n", cfg.mqtt.topic, cfg.app.interval_secs);

    thread::sleep(Duration::from_secs(2));

    // Estado inicial desconocido: la primera lectura válida fija el punto de partida
    let mut estado_mhrd: Option<bool> = None;

    loop {
        procesar_mhrd(
            &mut sensores.mhrd,
            &mut com,
            &mut storage,
            &mut estado_mhrd,
        );
        thread::sleep(Duration::from_secs(2));

        println!("Pendientes: {}  —  esperando {}s...\n", storage.pending_count(), cfg.app.interval_secs);
        thread::sleep(Duration::from_secs(cfg.app.interval_secs));
    }
}

/// Procesa una lectura del sensor MH-RD.
///
/// Compara el estado actual (`mojado: bool`) con `estado_anterior`.
/// Solo guarda en SQLite y publica por MQTT cuando el estado CAMBIÓ
/// respecto a la lectura anterior (transición SECO <-> HÚMEDO).
fn procesar_mhrd(
    sensor: &mut MhRdSensor,
    com: &mut Comunicacion,
    storage: &mut SqliteStorage,
    estado_anterior: &mut Option<bool>,
) {
    match sensor.read() {
        Ok(output) => {
            match SensorParser::mhrd(&output) {
                Ok(mojado) => {
                    println!("[MH-RD] Estado: {}", if mojado { "HÚMEDO" } else { "SECO" });

                    if *estado_anterior == Some(mojado) {
                        // Mismo estado que la lectura anterior: no se publica nada
                        return;
                    }

                    println!("  ⚡ cambio de estado -> {}", if mojado { "HÚMEDO" } else { "SECO" });
                    let json = com.formatter.desde_bool("lluvia", mojado);
                    storage.save(output).unwrap();
                    enviar(&mut com.mqtt, json.as_bytes(), storage);

                    *estado_anterior = Some(mojado);
                }
                Err(e) => eprintln!("[MH-RD] Error al parsear: {:?}", e),
            }
        }
        Err(e) => eprintln!("[MHRD] Error al leer: {:?}", e),
    }
}

/// Envía datos por MQTT. Si falla por desconexión, reintenta la cola
/// de pendientes guardada en SQLite; si falla por error de protocolo,
/// se descarta porque reintentarlo no cambiaría el resultado.
fn enviar(mqtt: &mut MqttCommunicator, data: &[u8], storage: &mut SqliteStorage) {
    match mqtt.send(data) {
        Ok(())                               => println!("  ✓ enviado"),
        Err(CommunicatorError::Disconnected) => {
            eprintln!("  ✗ sin conexión — reintentando pendientes...");
            let n = storage.flush_pending(mqtt);
            println!("  ✓ reenviados: {}", n);
        }
        Err(CommunicatorError::SendError)    => eprintln!("  ✗ error de protocolo"),
    }
}
