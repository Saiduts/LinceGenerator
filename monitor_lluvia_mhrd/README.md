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

- **MH-RD — Sensor de lluvia** — GPIO 27

## Funcionalidades activas

- Formato Smart Campus: **activado**
- Reintento por desconexión (SQLite): **activado**
- Publicación por cambio de estado: **activado** (ver más abajo)

Broker MQTT: `localhost:1883`
Topic: `sensores/lluvia/estado`
Intervalo de lectura: `2s`

## Publicación por cambio de estado

A diferencia de una aplicación Lince genérica (que publica cada
lectura), esta aplicación **solo publica por MQTT cuando el estado del
sensor cambia** (transición SECO ↔ HÚMEDO):

- Si pasa de SECO a HÚMEDO → se publica una vez ("empezó a llover").
- Si pasa de HÚMEDO a SECO → se publica una vez más ("dejó de
  llover").
- Mientras el estado se mantenga igual, no se genera tráfico de red ni
  escritura en SQLite nuevos, aunque el programa sigue leyendo el
  sensor e imprimiendo cada lectura en consola.

## Editar configuración

Todos los parámetros de red, pines y rutas viven en `config.toml`.
Modificarlo no requiere recompilar — solo reiniciar el binario.

## Compilar en Raspberry Pi

```bash
cargo build --release
./target/release/monitor_lluvia_mhrd
```

## Cross-compilar desde Linux x86_64

```bash
rustup target add aarch64-unknown-linux-gnu
sudo apt install gcc-aarch64-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

El binario queda en `target/aarch64-unknown-linux-gnu/release/monitor_lluvia_mhrd`.

Cópialo junto con `config.toml` a la Raspberry Pi (ambos deben estar
en el mismo directorio):

```bash
scp target/aarch64-unknown-linux-gnu/release/monitor_lluvia_mhrd config.toml pi@<IP_RASPBERRY>:~/
```

## Ejecutar en la Raspberry Pi

```bash
chmod +x monitor_lluvia_mhrd
./monitor_lluvia_mhrd
```