from flask import Flask, render_template, request, send_file, jsonify
from jinja2 import Environment, FileSystemLoader
import io, zipfile

app = Flask(__name__)
JINJA_ENV = Environment(loader=FileSystemLoader("templates"), trim_blocks=True, lstrip_blocks=True)

# Metadatos de cada sensor soportado. struct_name y sensor_struct
# se usan en los templates de Rust para nombrar tipos consistentemente.
SENSOR_META = {
    "dht11":   {"label": "DHT11 — Temperatura y humedad",  "type": "digital", "parser": "dht",
                "struct_name": "Dht11", "sensor_struct": "Dht11Sensor"},
    "dht22":   {"label": "DHT22 — Temperatura y humedad",  "type": "digital", "parser": "dht",
                "struct_name": "Dht22", "sensor_struct": "Dht22Sensor"},
    "ds18b20": {"label": "DS18B20 — Temperatura OneWire",  "type": "onewire", "parser": "ds18b20",
                "struct_name": "Ds18b20", "sensor_struct": "Ds18b20Sensor"},
    "mhrd":    {"label": "MH-RD — Sensor de lluvia",       "type": "digital", "parser": "mhrd",
                "struct_name": "MhRd", "sensor_struct": "MhRdSensor"},
}

@app.route("/")
def index():
    return render_template("index.html")

@app.route("/preview", methods=["POST"])
def preview():
    cfg = _parse(request.json)
    return jsonify({
        "main_rs":          _render("main_rs.j2", cfg),
        "config_rs":        _render("config_rs.j2", cfg),
        "sensores_rs":      _render("sensores_rs.j2", cfg),
        "comunicacion_rs":  _render("comunicacion_rs.j2", cfg),
        "almacenamiento_rs": _render("almacenamiento_rs.j2", cfg) if cfg["use_sqlite"] else "",
        "app_rs":           _render("app_rs.j2", cfg),
        "cargo_toml":       _render("cargo_toml.j2", cfg),
        "config_toml":      _render("config_toml.j2", cfg),
    })

@app.route("/generate", methods=["POST"])
def generate():
    cfg = _parse(request.json)
    buf = io.BytesIO()
    with zipfile.ZipFile(buf, "w", zipfile.ZIP_DEFLATED) as zf:
        zf.writestr("src/main.rs",          _render("main_rs.j2", cfg))
        zf.writestr("src/config.rs",        _render("config_rs.j2", cfg))
        zf.writestr("src/sensores.rs",      _render("sensores_rs.j2", cfg))
        zf.writestr("src/comunicacion.rs",  _render("comunicacion_rs.j2", cfg))
        if cfg["use_sqlite"]:
            zf.writestr("src/almacenamiento.rs", _render("almacenamiento_rs.j2", cfg))
        zf.writestr("src/app.rs",           _render("app_rs.j2", cfg))
        zf.writestr("Cargo.toml",           _render("cargo_toml.j2", cfg))
        zf.writestr("config.toml",          _render("config_toml.j2", cfg))
        zf.writestr("README.md",            _render("readme_md.j2", cfg))
        zf.writestr("deploy.sh",            _render("deploy_sh.j2", cfg))
    buf.seek(0)
    return send_file(buf, download_name="lince_app.zip", as_attachment=True, mimetype="application/zip")

def _parse(data):
    sensors = []
    for key in ["dht11", "dht22", "ds18b20", "mhrd"]:
        if key in data.get("sensors", []):
            entry = dict(SENSOR_META[key], key=key)
            entry["pin"]       = data.get(f"pin_{key}", "17")
            entry["device_id"] = data.get("ds18b20_id", "28-000000000000")
            sensors.append(entry)
    return {
        "sensors":          sensors,
        "broker":           data.get("broker",    "localhost"),
        "port":             int(data.get("port",  1883)),
        "topic":            data.get("topic",     "device/messages"),
        "client_id":        data.get("client_id", "lince-gateway"),
        "device_id":        data.get("device_id", "raspberry-lince"),
        "db_path":          data.get("db_path",   "pendientes.db"),
        "interval":         int(data.get("interval", 10)),
        "use_sqlite":       bool(data.get("use_sqlite",      True)),
        "use_smartcampus":  bool(data.get("use_smartcampus", True)),
    }

def _render(template, ctx):
    return JINJA_ENV.get_template(template).render(**ctx)

if __name__ == "__main__":
    app.run(debug=True, port=5000)
