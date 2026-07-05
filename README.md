# LinceGenerator

Asistente de generación simple: herramienta web que produce el código fuente de una aplicación Lince a partir de una configuración seleccionada por el usuario.

## ¿Qué es?
LinceGenerator es una aplicación web que automatiza la creación del esqueleto y partes del código de una aplicación Lince (plantillas, vistas y estructura) tomando como entrada una configuración elegida por el usuario. Está pensada para acelerar el inicio de proyectos y estandarizar la estructura del código.

## ¿Para qué sirve?
- Generar rápidamente la estructura básica de una aplicación Lince.
- Producir archivos y plantillas (HTML/Jinja) según opciones seleccionadas.
- Ahorrar tiempo en la fase inicial de un proyecto y asegurar consistencia entre aplicaciones.

## Funcionalidades
- Interfaz web para seleccionar opciones de configuración.
- Generación de código y plantillas con Jinja.
- Descarga del proyecto generado en un paquete (ZIP) listo para usar.
- Posible integración con sistemas de despliegue (configurable por el usuario).

## Requisitos
- Python 3.8+ (recomendado)
- Pip
- (Opcional) Docker para contenedores

Nota: Si tu proyecto incluye un archivo `requirements.txt` o `pyproject.toml`, instala las dependencias listadas allí.

## Instalación (local)
1. Clona el repositorio:

   git clone https://github.com/Saiduts/LinceGenerator.git
   cd LinceGenerator

2. Crea y activa un entorno virtual (opcional pero recomendado):

   python -m venv venv
   # En macOS/Linux
   source venv/bin/activate
   # En Windows (PowerShell)
   .\venv\Scripts\Activate.ps1

3. Instala las dependencias:

   pip install -r requirements.txt

   Si no existe `requirements.txt`, instala las dependencias necesarias manualmente (por ejemplo Flask u otro framework web que use el proyecto).

4. Configura variables de entorno si la aplicación las requiere (por ejemplo PORT, FLASK_APP, configuración de base de datos, etc.). Consulta los archivos de configuración del repositorio para detalles.

## Ejecutar localmente
Dependiendo del framework usado en el proyecto, ejecuta uno de los comandos siguientes:

- Si es una aplicación Flask (ejemplo):

  export FLASK_APP=app.py
  export FLASK_ENV=development
  flask run

- Si hay un archivo `run.py` o `main.py`, puedes usar:

  python run.py

- Si usas Gunicorn (producción local):

  gunicorn -w 4 -b 0.0.0.0:8000 app:app

Ajusta `app:app` al punto de entrada real de la aplicación.

Después de iniciar la aplicación, abre tu navegador en http://localhost:5000 (o el puerto configurado) para usar la interfaz web del generador.

## Despliegue
A continuación hay varias opciones comunes para desplegar la aplicación.

1) Deploy con Docker
- Construye la imagen (si existe Dockerfile):

  docker build -t lincegenerator:latest .

- Ejecuta el contenedor:

  docker run -d -p 8000:8000 --name lincegenerator lincegenerator:latest

2) Deploy en un servidor con Gunicorn + Nginx
- Instala dependencias en el servidor
- Ejecuta la aplicación con Gunicorn:

  gunicorn -w 4 -b 127.0.0.1:8000 app:app

- Configura Nginx para hacer proxy inverso desde el puerto 80/443 hacia Gunicorn.

3) Plataformas PaaS (Heroku, Railway, Render, etc.)
- Sube el repositorio a la plataforma.
- Configura los buildpacks o Dockerfile según la plataforma.
- Añade las variables de entorno necesarias.

## Estructura sugerida del proyecto
- templates/  → plantillas Jinja/HTML
- static/     → CSS, JS, imágenes
- app.py / main.py / run.py → punto de entrada de la aplicación
- requirements.txt → dependencias

(Ajusta según la estructura real del repositorio.)

## Contribuir
Si quieres contribuir:
1. Haz un fork del repositorio.
2. Crea una rama con tu cambio: git checkout -b feat/mi-mejora
3. Haz commit de tus cambios y abre un Pull Request describiendo la mejora.

## Soporte y contacto
Si tienes problemas o preguntas, abre un issue en el repositorio o contacta al mantenedor.

---

*README generado y actualizado en español.*
