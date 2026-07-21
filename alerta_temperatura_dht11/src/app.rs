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

/// Estado de alerta respecto al umbral configurado en `[dht11]`.
///
/// Solo se publica por MQTT cuando este estado CAMBIA de una lectura
/// a la siguiente; mientras la temperatura se mantenga del mismo lado
/// del umbral no se genera tráfico de red nuevo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EstadoAlerta {
    Normal,
    Alerta,
}

/// Ejecuta el loop de lectura indefinidamente.
///
/// Por cada sensor configurado: lee, (opcionalmente) parsea y formatea
/// a Smart Campus, (opcionalmente) guarda en SQLite, e intenta enviar
/// por MQTT. Si el envío falla por desconexión y SQLite está activado,
/// se reintenta la cola de pendientes antes de continuar.
///
/// A diferencia de una aplicación Lince genérica (que publica cada
/// lectura), esta aplicación mantiene el último `EstadoAlerta` conocido
/// y solo llama a `enviar(...)` cuando dicho estado cambia.
pub fn ejecutar(
    cfg: &AppConfig,
    mut sensores: Sensores,
    mut com: Comunicacion,
    mut storage: SqliteStorage,
) {
    println!("Lince iniciado — broker: {}:{}", cfg.mqtt.broker, cfg.mqtt.port);
    println!("Topic: {}  |  Intervalo: {}s", cfg.mqtt.topic, cfg.app.interval_secs);
    println!("Umbral de alerta: {:.1}°C\n", cfg.dht11.umbral_temperatura_c);

    thread::sleep(Duration::from_secs(2));

    // Estado inicial desconocido: la primera lectura válida fija el punto de partida
    let mut estado_dht11: Option<EstadoAlerta> = None;

    loop {
        procesar_dht11(
            &mut sensores.dht11,
            &mut com,
            &mut storage,
            cfg.dht11.umbral_temperatura_c,
            &mut estado_dht11,
        );
        thread::sleep(Duration::from_secs(2));

        println!("Pendientes: {}  —  esperando {}s...\n", storage.pending_count(), cfg.app.interval_secs);
        thread::sleep(Duration::from_secs(cfg.app.interval_secs));
    }
}

/// Procesa una lectura del sensor DHT11.
///
/// Calcula el `EstadoAlerta` actual según `umbral` y lo compara con
/// `estado_anterior`. Solo guarda en SQLite y publica por MQTT cuando
/// el estado CAMBIÓ respecto a la lectura anterior (cruce del umbral).
fn procesar_dht11(
    sensor: &mut Dht11Sensor,
    com: &mut Comunicacion,
    storage: &mut SqliteStorage,
    umbral: f32,
    estado_anterior: &mut Option<EstadoAlerta>,
) {
    match sensor.read() {
        Ok(output) => {
            match SensorParser::dht(&output) {
                Ok(valores) => {
                    let temp = valores["temperatura"];
                    println!("[DHT11] Temp: {}°C  Hum: {}%", temp, valores["humedad"]);

                    let estado_actual = if temp >= umbral {
                        EstadoAlerta::Alerta
                    } else {
                        EstadoAlerta::Normal
                    };

                    if *estado_anterior == Some(estado_actual) {
                        // Mismo lado del umbral que la lectura anterior: no se publica nada
                        return;
                    }

                    println!("  ⚡ cruce de umbral -> {:?}", estado_actual);
                    let json = com.formatter.desde_mapa(&valores);
                    storage.save(output).unwrap();
                    enviar(&mut com.mqtt, json.as_bytes(), storage);

                    *estado_anterior = Some(estado_actual);
                }
                Err(e) => eprintln!("[DHT11] Error al parsear: {:?}", e),
            }
        }
        Err(e) => eprintln!("[DHT11] Error al leer: {:?}", e),
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
