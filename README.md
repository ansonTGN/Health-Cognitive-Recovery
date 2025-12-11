# LaMuralla Health: Cognitive Recovery Engine

<div align="center">

![Rust](https://img.shields.io/badge/Core-Rust_1.75+-orange?style=for-the-badge&logo=rust)
![Neo4j](https://img.shields.io/badge/Graph_DB-Neo4j_5+-008CC1?style=for-the-badge&logo=neo4j&logoColor=white)
![AI](https://img.shields.io/badge/AI_Orchestration-Rig_Core-purple?style=for-the-badge)
![Security](https://img.shields.io/badge/Security-RBAC_%26_JWT-green?style=for-the-badge&logo=shield)
![Domain](https://img.shields.io/badge/Domain-Mental_Health-red?style=for-the-badge&logo=heart)

</div>

---

**[ 🇪🇸 Español ](#-español) | [ 🇺🇸 English ](#-english) | [ 🏴󠁥󠁳󠁣󠁴󠁿 Català ](#-català)**

---

<a name="es"></a>
## 🇪🇸 Español

### ❤️ Para el Sector Social: ¿Qué es LaMuralla?
> *"Transformando historias de vida en evidencia para la recuperación."*

Si trabajas en **salud mental, trabajo social o intervención comunitaria**, sabes que la parte más valiosa de tu trabajo queda atrapada en textos no estructurados: notas de evolución, informes psicosociales, memorias de actividades y grabaciones de audio. Esa información cualitativa contiene las claves de la recuperación de las personas, pero es difícil de analizar a gran escala.

**LaMuralla Health** no es un simple archivo digital. Es un **Motor Cognitivo** que lee, escucha y comprende la realidad de tus usuarios.

*   **Evidencia Basada en Datos:** El sistema lee tus notas y conecta automáticamente los puntos. Detecta, por ejemplo, si la asistencia al "Taller de Pintura" (Intervención) está correlacionada con una mejora en la "Autoestima" (Resultado).
*   **Visión Holística:** Genera un mapa visual (Grafo) de la red de apoyo, síntomas y recursos de cada persona, evitando que la información quede aislada en silos.
*   **Asistente Inteligente:** Puedes preguntarle en lenguaje natural: *"¿Qué intervenciones están funcionando mejor para reducir el aislamiento en usuarios mayores de 65 años?"* y el sistema responderá basándose en la evidencia acumulada en tu base de datos.

### 💻 Documentación Técnica

**LaMuralla Health** es un sistema **GraphRAG (Retrieval-Augmented Generation on Knowledge Graphs)** de alto rendimiento construido con una arquitectura hexagonal.

#### Arquitectura del Core (Backend)
*   **Lenguaje:** Rust (garantía de seguridad de memoria y concurrencia real).
*   **Framework Web:** Axum (Asíncrono, basado en Tokio).
*   **Base de Datos:** Neo4j. Utiliza un enfoque híbrido:
    *   **Grafo:** Almacena entidades (`Person`, `Condition`, `Intervention`, `Outcome`) y sus relaciones semánticas.
    *   **Vectores:** Almacena Embeddings de los fragmentos de texto para búsqueda semántica (`vector index`).
*   **Orquestación IA:** Implementado sobre `rig-core`, permitiendo la creación de **Agentes Autónomos**.
*   **Multimodalidad:** Capaz de procesar Texto, Imágenes (OCR/Visión) y Audio (Whisper) mediante `ffmpeg` y pipelines de ingesta.

#### Sistema de Agentes Dinámicos
El sistema no es estático. Utiliza archivos YAML en `/config/agents` y `/config/tools` para definir comportamientos sin recompilar:
*   **Social Worker Agent:** Planifica intervenciones y consulta el clima (Tool HTTP).
*   **Data Analyst Agent:** Genera consultas Cypher complejas para extraer estadísticas (Tool Cypher).
*   **Data Quality Auditor:** Verifica la integridad del grafo.

#### Frontend & Visualización
*   **Server-Side Rendering (SSR):** Renderizado rápido y seguro con **Tera**.
*   **Visualización de Grafos:** Integración con **Vis.js** para la exploración interactiva de nodos y relaciones.
*   **Interfaz Reactiva:** Chat en tiempo real con citas interactivas (el sistema indica exactamente qué documento justifica su respuesta).

---

<a name="en"></a>
## 🇺🇸 English

### ❤️ For the Social Sector: What is LaMuralla?
> *"Turning life stories into evidence for recovery."*

If you work in **mental health, social work, or community intervention**, you know that the most valuable part of your job is trapped in unstructured text: progress notes, psychosocial reports, activity logs, and audio recordings. This qualitative data holds the keys to recovery, but it is notoriously difficult to analyze at scale.

**LaMuralla Health** is not just a digital archive. It is a **Cognitive Engine** that reads, listens to, and understands the reality of your service users.

*   **Data-Driven Evidence:** The system reads your notes and automatically connects the dots. It detects, for example, if attendance at the "Art Workshop" (Intervention) correlates with an improvement in "Self-esteem" (Outcome).
*   **Holistic View:** Generates a visual map (Graph) of each person's support network, symptoms, and resources, preventing information from being siloed.
*   **Intelligent Assistant:** You can ask in natural language: *"Which interventions are working best to reduce isolation in users over 65?"* and the system answers based on the accumulated evidence in your database.

### 💻 Technical Documentation

**LaMuralla Health** is a high-performance **GraphRAG (Retrieval-Augmented Generation on Knowledge Graphs)** system built on a hexagonal architecture.

#### Core Architecture (Backend)
*   **Language:** Rust (memory safety and true concurrency).
*   **Web Framework:** Axum (Async, built on Tokio).
*   **Database:** Neo4j. Uses a hybrid approach:
    *   **Graph:** Stores entities (`Person`, `Condition`, `Intervention`, `Outcome`) and semantic relationships.
    *   **Vectors:** Stores text embeddings for semantic search (`vector index`).
*   **AI Orchestration:** Built on `rig-core`, allowing for **Autonomous Agents**.
*   **Multimodality:** Processes Text, Images (OCR/Vision), and Audio (Whisper) via ingestion pipelines.

#### Dynamic Agent System
The system is extensible via YAML configuration files in `/config/agents` and `/config/tools`:
*   **Social Worker Agent:** Plans interventions and checks weather APIs (HTTP Tool).
*   **Data Analyst Agent:** Generates complex Cypher queries for statistics (Cypher Tool).
*   **Data Quality Auditor:** Verifies graph integrity.

#### Frontend & Visualization
*   **Server-Side Rendering (SSR):** Fast and secure rendering with **Tera**.
*   **Graph Visualization:** Integration with **Vis.js** for interactive exploration of nodes and relationships.
*   **Reactive Interface:** Real-time chat with interactive citations (the system points to the exact source document justifying its answer).

---

<a name="ca"></a>
## 🏴󠁥󠁳󠁣󠁴󠁿 Català

### ❤️ Pel Sector Social: Què és LaMuralla?
> *"Transformant històries de vida en evidència per a la recuperació."*

Si treballes en **salut mental, treball social o intervenció comunitària**, saps que la part més valuosa de la teva feina queda atrapada en textos no estructurats: notes d'evolució, informes psicosocials, memòries d'activitats i gravacions d'àudio. Aquesta informació qualitativa conté les claus de la recuperació de les persones, però és difícil d'analitzar a gran escala.

**LaMuralla Health** no és un simple arxiu digital. És un **Motor Cognitiu** que llegeix, escolta i comprèn la realitat dels teus usuaris.

*   **Evidència Basada en Dades:** El sistema llegeix les teves notes i connecta automàticament els punts. Detecta, per exemple, si l'assistència al "Taller de Pintura" (Intervenció) està correlacionada amb una millora en l'"Autoestima" (Resultat).
*   **Visió Holística:** Genera un mapa visual (Graf) de la xarxa de suport, símptomes i recursos de cada persona.
*   **Assistent Intel·ligent:** Pots preguntar-li en llenguatge natural: *"Quines intervencions estan funcionant millor per reduir l'aïllament?"* i el sistema respon basant-se en l'evidència acumulada.

### 💻 Documentació Tècnica

**LaMuralla Health** és un sistema **GraphRAG** d'alt rendiment construït amb Rust.

#### Arquitectura del Core
*   **Llenguatge:** Rust & Axum.
*   **Base de Dades:** Neo4j (Híbrid Graf + Vectorial).
*   **IA:** Orquestració d'agents autònoms mitjançant `rig-core`.
*   **Multimodalitat:** Ingesta de documents, imatges i àudio.

---

## 🚀 Instal·lació / Installation / Instalación

### Prerequisites
*   Rust (Cargo) 1.75+
*   Neo4j Database (Local or AuraDB)
*   OpenAI API Key (or compatible provider like Ollama/Groq)

### Environment Setup (`.env`)
```bash
PORT=3000
# Database
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASS=password
# AI Provider
AI_PROVIDER=openai
AI_API_KEY=sk-proj-...
AI_MODEL=gpt-4o
# Security
JWT_SECRET=super_secure_secret
ADMIN_USER=admin
ADMIN_PASS=admin123
```

### Run
```bash
# Development
cargo run

# Production (Docker)
docker build -t lamuralla-health .
docker run -p 3000:3000 --env-file .env lamuralla-health
```

---

## 📂 Project Structure

```text
/
├── config/              # 🧠 Brain of the system
│   ├── agents/          # YAML definitions for AI Agents
│   └── tools/           # YAML definitions for Tools (HTTP, Cypher)
├── src/
│   ├── application/     # Business Logic (Ingestion, Reasoning)
│   ├── domain/          # Models & Ports (Hexagonal Arch)
│   ├── infrastructure/  # Neo4j, OpenAI/Rig, File System
│   └── interface/       # HTTP Handlers (Axum) & Templates
├── templates/           # HTML/Tera Views (UI)
└── Dockerfile
```

---

## 👨‍💻 Autor & Contacto / Author & Contact

**Ángel A. Urbina**  
*Lead Architect & Developer*  
Projecte d'Innovació Tecnològica per al Tercer Sector Social.

🌐 **Portfolio:** [https://angelurbinacv.netlify.app/](https://angelurbinacv.netlify.app/)  
📧 **GitHub:** [https://github.com/Angel-Urbina](https://github.com/Angel-Urbina)

© 2025 LaMuralla Health Project. All Rights Reserved.
