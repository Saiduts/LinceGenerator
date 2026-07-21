# Aplicación Lince generada automáticamente

## Estructura del proyecto

```
src/
├── main.rs            ← punto de entrada, orquesta los módulos
├── config.rs          ← carga config.toml en runtime
├── sensores.rs        ← inicialización de sensores
├── comunicacion.rs     ← MQTT + formato Smart Campus
├── almacenamiento.rs   ← SQLite con reintento por desconexión
└── app.rs             ← loop principal de lectura y envío
config.toml             ← configuración editable sin recompilar
```

## Sensores configurados

- **DHT11 — Temperatura y humedad** — GPIO 17

## Funcionalidades activas

- Formato Smart Campus: **activado**
- Reintento por desconexión (SQLite): **activado**
- Publicación por cruce de umbral: **activado** (ver más abajo)

Broker MQTT: `localhost:1883`
Topic: `sensores/temperatura/alertas`
Intervalo de lectura: `5s`

## Publicación por cruce de umbral

A diferencia de una aplicación Lince genérica (que publica cada
lectura), esta aplicación **solo publica por MQTT cuando la temperatura
cruza el umbral configurado** en `[dht11].umbral_temperatura_c`:

- Si la temperatura pasa de estar por debajo del umbral a estar en o
  por encima de él → se publica una vez ("entrada en alerta").
- Si vuelve a bajar del umbral → se publica una vez más ("regreso a
  normal").
- Mientras se mantenga del mismo lado del umbral, no se genera tráfico
  de red ni escritura en SQLite nuevos, aunque el programa sigue
  leyendo el sensor e imprimiendo cada lectura en consola.

## Editar configuración

Todos los parámetros de red, pines y rutas viven en `config.toml`.
Modificarlo no requiere recompilar — solo reiniciar el binario.

## Compilar en Raspberry Pi

```bash
cargo build --release
./target/release/alerta_temperatura_dht11
```

## Cross-compilar desde Linux x86_64

```bash
rustup target add aarch64-unknown-linux-gnu
sudo apt install gcc-aarch64-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

El binario queda en `target/aarch64-unknown-linux-gnu/release/alerta_temperatura_dht11`.

Cópialo junto con `config.toml` a la Raspberry Pi (ambos deben estar
en el mismo directorio):

```bash
scp target/aarch64-unknown-linux-gnu/release/alerta_temperatura_dht11 config.toml pi@<IP_RASPBERRY>:~/
```

## Ejecutar en la Raspberry Pi

```bash
chmod +x alerta_temperatura_dht11
./alerta_temperatura_dht11
```