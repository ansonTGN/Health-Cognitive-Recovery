# 🤖 Guía de Creación de Agentes y Herramientas

Esta guía detalla cómo extender las capacidades de **LaMuralla Health** mediante la definición de Agentes de IA y Herramientas dinámicas.

El sistema utiliza una arquitectura basada en configuración (**YAML**) que permite crear nuevos comportamientos y conexiones sin necesidad de recompilar el código fuente en Rust. El motor subyacente utiliza `rig-core` para orquestar los LLMs.

## 📂 Estructura de Directorios

El sistema busca las definiciones en la carpeta `config/` situada en la raíz del proyecto:

```text
/
├── config/
│   ├── agents/          # Definiciones de Agentes (.yaml)
│   └── tools/           # Definiciones de Herramientas (.yaml)
├── src/
├── Cargo.toml
└── ...
```

---

## 🛠️ Creación de Herramientas (Tools)

Las herramientas son funciones que el Agente puede decidir ejecutar. Se definen en `config/tools/`.

### Estructura Base del YAML

```yaml
id: identificador_unico  # Debe coincidir con el nombre del archivo (sin .yaml)
name: Nombre Legible
description: Descripción detallada para la IA (Crucial: la IA usa esto para saber cuándo usarla)
type: [http | cypher | cli]
input_schema:            # Esquema JSON para los argumentos
  type: object
  properties:
    parametro_1:
      type: string
      description: Qué debe poner la IA aquí.
  required: [parametro_1]
```

### Tipos de Herramientas Soportadas

#### 1. Herramienta HTTP (`type: http`)
Permite al agente conectarse a APIs externas (REST). Soporta interpolación de variables usando la sintaxis `{{input.nombre_variable}}`.

**Ejemplo: Consultar el Clima**
*Archivo: `config/tools/weather.yaml`*
```yaml
id: weather
name: Servicio Meteorológico
description: Obtiene el clima actual basado en latitud y longitud. Útil para planificar salidas.
type: http
method: GET
# La IA rellenará 'lat' y 'lon'
url: "https://api.open-meteo.com/v1/forecast?latitude={{input.lat}}&longitude={{input.lon}}&current_weather=true"
input_schema:
  type: object
  properties:
    lat:
      type: string
      description: Latitud geográfica.
    lon:
      type: string
      description: Longitud geográfica.
  required: [lat, lon]
```

#### 2. Herramienta Cypher/Grafo (`type: cypher`)
Conecta directamente con la base de datos **Neo4j** interna de LaMuralla. Actualmente, está configurada para explorar el vecindario de un concepto clínico.

**Ejemplo: Explorador de Grafo**
*Archivo: `config/tools/graph_explorer.yaml`*
```yaml
id: graph_explorer
name: Explorador Clínico
description: Busca información en el grafo de conocimiento sobre pacientes, síntomas o intervenciones.
type: cypher
input_schema:
  type: object
  properties:
    concept_name:
      type: string
      description: El nombre exacto de la entidad a buscar (ej. "Juan", "Ansiedad", "Taller Pintura").
  required: [concept_name]
```

> **Nota:** El backend Rust inyecta automáticamente el repositorio de Neo4j en estas herramientas.

#### 3. Herramienta CLI (`type: cli`)
*Estado: Deshabilitada por defecto por seguridad.*
Permitiría ejecutar comandos de terminal en el servidor.

---

## 🕵️ Creación de Agentes

Los agentes son perfiles de IA con instrucciones específicas y acceso a un set de herramientas. Se definen en `config/agents/`.

### Estructura del YAML

*Archivo: `config/agents/social_worker.yaml`*
```yaml
id: social_worker
name: Trabajador Social Senior
description: Especialista en intervención comunitaria.
model: gpt-4o           # (Opcional) Sobrescribe el modelo global del .env
tools:                  # Lista de IDs de herramientas (deben existir en config/tools)
  - graph_explorer
  - weather

system_prompt: |
  Eres un Trabajador Social con 20 años de experiencia.
  Tu objetivo es analizar la situación de los pacientes y proponer intervenciones.
  
  REGLAS:
  1. Si te preguntan por un paciente, USA SIEMPRE la herramienta 'graph_explorer' primero.
  2. Si vas a proponer una actividad al aire libre, verifica el clima con 'weather'.
  3. Mantén un tono profesional y empático.
```

---

## 🔌 Uso de la API (Backend Integration)

Para interactuar con los agentes desde el frontend o sistemas externos, se utilizan los siguientes endpoints.

### Autenticación
El sistema utiliza seguridad basada en **Cookies (JWT)**.
1.  Debes iniciar sesión primero en `POST /` o tener una cookie válida `lamuralla_jwt`.
2.  Para pruebas con `curl` o Postman, incluye la cabecera: `Cookie: lamuralla_jwt=TU_TOKEN_JWT`.

### 1. Listar Agentes Disponibles
Obtiene la lista de todos los agentes definidos en la carpeta `config/agents`.

*   **Método:** `GET`
*   **URL:** `/api/agents`

**Respuesta (JSON):**
```json
[
  {
    "id": "social_worker",
    "name": "Trabajador Social Senior",
    "description": "Especialista en intervención comunitaria.",
    "system_prompt": "...",
    "tools": ["graph_explorer", "weather"]
  }
]
```

### 2. Chatear con un Agente
Envía un mensaje a un agente específico. El backend instanciará el agente, cargará sus herramientas, ejecutará el razonamiento (Chain of Thought) y devolverá la respuesta.

*   **Método:** `POST`
*   **URL:** `/api/agents/chat`
*   **Headers:** `Content-Type: application/json`

**Cuerpo de la Petición:**
```json
{
  "agent_id": "social_worker",
  "message": "Revisa el caso del paciente Juan y dime si podemos hacer una actividad en el parque hoy."
}
```

**Flujo Interno:**
1.  El sistema carga `social_worker.yaml`.
2.  El sistema carga `graph_explorer.yaml` y `weather.yaml`.
3.  El LLM recibe el prompt.
4.  El LLM decide llamar a `graph_explorer` con `{"concept_name": "Juan"}`.
5.  Rust ejecuta la consulta en Neo4j y devuelve los datos al LLM.
6.  El LLM ve los datos de Juan, luego decide llamar a `weather` con las coordenadas (si las sabe o las deduce).
7.  Rust hace la petición HTTP a la API del clima.
8.  El LLM procesa todo y genera la respuesta final.

**Respuesta (JSON):**
```json
{
  "response": "He revisado el expediente de Juan. Actualmente presenta síntomas de ansiedad leve. Respecto a la actividad en el parque, el servicio meteorológico indica lluvia, por lo que sugiero cambiar la actividad al Taller de Pintura interior.",
  "used_tools": [] 
}
```

---

## ⚡ Solución de Problemas Comunes

1.  **Error 500: "Agent not found"**
    *   Verifica que el archivo `.yaml` exista en `config/agents/`.
    *   Verifica que el `id` dentro del YAML coincida con el `agent_id` enviado en el JSON.

2.  **El Agente alucina o no usa la herramienta**
    *   Revisa la `description` en el YAML de la herramienta. ¿Es lo suficientemente clara para que el modelo sepa cuándo usarla?
    *   Revisa el `system_prompt` del agente. ¿Le has instruido explícitamente para usar herramientas?

3.  **Error de Compilación `future cannot be shared between threads`**
    *   Esto ocurre si modificas el ejecutor de herramientas (`executor.rs`). Recuerda que las llamadas a Neo4j (que usan `async_trait`) deben envolverse en `tokio::task::spawn` para ser compatibles con `rig-core`.

4.  **Error de "Permissions" o "Auth"**
    *   Asegúrate de estar enviando la cookie de sesión válida. Si estás en modo desarrollo, puedes comentar temporalmente el middleware `auth_middleware` en `main.rs` para las rutas de `/api/agents`.