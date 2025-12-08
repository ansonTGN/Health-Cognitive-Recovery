# LaMuralla Health: Cognitive Recovery Engine

<div align="center">

![Rust](https://img.shields.io/badge/Core-Rust_1.80+-orange?style=for-the-badge&logo=rust)
![Neo4j](https://img.shields.io/badge/Graph_DB-Neo4j_5+-008CC1?style=for-the-badge&logo=neo4j&logoColor=white)
![Frontend](https://img.shields.io/badge/Frontend-Tera_%26_Vis.js-yellow?style=for-the-badge&logo=javascript)
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

Si trabajas en **salud mental, trabajo social o integración comunitaria**, sabes que la parte más importante de tu trabajo queda escrita en notas de evolución, memorias de actividades y observaciones diarias. Sin embargo, esa información cualitativa a menudo se pierde en archivadores o es difícil de analizar en conjunto.

**LaMuralla Health** es un sistema de inteligencia cognitiva que lee y "comprende" esos textos. No solo guarda la información, sino que **conecta los puntos** para apoyar el **Modelo de Recuperación**:

*   **Evidencia Automática:** Detecta si un usuario que asiste al *Club Social* (Intervención) mejora su *Autoestima* (Resultado) basándose en las notas diarias.
*   **Consultas Naturales:** Permite preguntar: *"¿Qué actividades están generando mayor vínculo comunitario en el último mes?"*.
*   **Visión Holística:** Visualiza la red de apoyos, síntomas y recursos de cada persona, evitando que la información vital quede aislada.

---

### 💻 Documentación Técnica

#### Introducción: Motor GraphRAG
**LaMuralla Health** es un motor **GraphRAG (Retrieval-Augmented Generation)**. A diferencia de los chatbots tradicionales, este sistema construye un **Grafo de Conocimiento** dinámico. Utiliza una arquitectura hexagonal en **Rust (Axum)** para orquestar la ingesta de documentos, la vectorización (Embeddings) y el razonamiento lógico mediante LLMs, persistiendo los datos en **Neo4j**.

#### 🖥️ Capacidades del Frontend (Interfaz de Usuario)
La interfaz ha sido diseñada para ser intuitiva, reactiva y segura, renderizada desde el servidor (**SSR**) con **Tera** y potenciada por **Bootstrap 5** y **Vis.js**.

1.  **Panel de Visualización de Grafos (Interactivo):**
    *   Renderizado de nodos categorizados por colores según la ontología clínica (`Person`, `Condition`, `Intervention`, `Outcome`).
    *   Física de fuerzas para auto-organización del grafo.
    *   Zoom, paneo y selección de nodos para exploración profunda.

2.  **Chat Asistente con Evidencia (Citas Interactivas):**
    *   Interfaz tipo chat para consultas en lenguaje natural.
    *   **Sistema de Citas `[1]`:** Cada afirmación de la IA incluye referencias a las fuentes originales.
    *   **Navegación Bidireccional:** Al hacer clic en una cita o en "Ver Grafo", la cámara se mueve automáticamente para enfocar las entidades y relaciones mencionadas en esa evidencia.

3.  **Ficha Técnica de Entidad (Side-Panel):**
    *   Se despliega automáticamente al seleccionar un nodo.
    *   **Métricas en Tiempo Real:** Muestra el grado de conexión y relevancia (centralidad) del concepto.
    *   **Aislamiento de Contexto:** Botón para filtrar el grafo y mostrar solo el "vecindario" del nodo seleccionado (Subgrafo Contextual).
    *   Listado detallado de relaciones entrantes y salientes.

4.  **Gestión y Seguridad (Role-Based UI):**
    *   **Acceso Diferenciado:** La interfaz cambia según si el usuario es `User` (solo lectura/chat) o `Admin`.
    *   **Panel de Ingesta (Admin):** Subida de archivos (`PDF`, `DOCX`, `TXT`) o pegado de texto directo con barra de progreso en tiempo real via WebSockets/Streams.
    *   **Gestión de Equipo (Admin):** Panel completo para dar de alta profesionales, asignar roles y revocar accesos.

5.  **Herramientas de Exportación:**
    *   Descarga del grafo en formatos estándar: **JSON-LD** (Web Semántica), **RDF/Turtle** y **GraphML** (Gephi/Cytoscape).
    *   Captura de pantalla en alta resolución del estado actual del grafo.

#### 🛡️ Arquitectura de Seguridad (Backend)
*   **Autenticación:** JWT (JSON Web Tokens) en cookies `HttpOnly` + `Secure` + `SameSite=Strict`.
*   **Protección:** Hashing de contraseñas (Bcrypt) y saneamiento de inputs.
*   **Control de Acceso:** Middlewares en Rust para proteger rutas administrativas.

---

<a name="en"></a>
## 🇺🇸 English

### ❤️ For the Social Sector: What is LaMuralla?
> *"Turning life stories into evidence for recovery."*

If you work in **mental health, social work, or community integration**, you know that the most vital part of your job is written in progress notes, workshop reports, and daily observations. However, that information is often lost or hard to analyze as a whole.

**LaMuralla Health** is a cognitive intelligence system that reads and "understands" those texts. It doesn't just store information; it **connects the dots** to support the **Recovery Model**:

*   **Automatic Evidence:** It detects if a user attending the *Social Club* (Intervention) improves their *Self-esteem* (Outcome) based on daily notes.
*   **Natural Queries:** Allows you to ask: *"Which activities are generating the most community bonding?"*
*   **Holistic View:** Visualizes the network of support, symptoms, and resources for each person.

---

### 💻 Technical Documentation

#### Introduction: GraphRAG Engine
**LaMuralla Health** is an advanced **GraphRAG (Retrieval-Augmented Generation)** engine. Unlike traditional chatbots, this system builds a dynamic **Knowledge Graph**. It uses a Hexagonal Architecture in **Rust (Axum)** to orchestrate document ingestion, embedding generation, and LLM reasoning, persisting data in **Neo4j**.

#### 🖥️ Frontend Capabilities (User Interface)
The UI is designed to be intuitive, reactive, and secure, utilizing Server-Side Rendering (**SSR**) with **Tera**, **Bootstrap 5**, and **Vis.js**.

1.  **Graph Visualization Panel (Interactive):**
    *   Node rendering color-coded by clinical ontology (`Person`, `Condition`, `Intervention`, `Outcome`).
    *   Force-directed physics for graph self-organization.
    *   Zoom, pan, and node selection for deep exploration.

2.  **Evidence-Based Assistant Chat:**
    *   Natural language query interface.
    *   **Citation System `[1]`:** Every AI claim includes interactive references to original sources.
    *   **Bi-directional Navigation:** Clicking a citation or "View Graph" automatically moves the camera to focus on the entities and relationships mentioned in that evidence.

3.  **Entity Detail Card (Side-Panel):**
    *   Automatically unfolds when a node is selected.
    *   **Real-time Metrics:** Displays connection degree and relevance (centrality) of the concept.
    *   **Context Isolation:** Button to filter the graph and show only the selected node's neighborhood (Contextual Subgraph).
    *   Detailed list of incoming and outgoing relationships.

4.  **Management & Security (Role-Based UI):**
    *   **Differentiated Access:** The UI adapts based on the user role: `User` (Read-only/Chat) or `Admin`.
    *   **Ingestion Panel (Admin):** File upload (`PDF`, `DOCX`, `TXT`) or direct text input with real-time progress bars via streams.
    *   **Team Management (Admin):** Full panel to register professionals, assign roles, and revoke access.

5.  **Export Tools:**
    *   Graph export in standard formats: **JSON-LD** (Semantic Web), **RDF/Turtle**, and **GraphML** (Gephi).
    *   High-resolution screenshot capture of the current graph state.

---

<a name="ca"></a>
## 🏴󠁥󠁳󠁣󠁴󠁿 Català

### ❤️ Pel Sector Social: Què és LaMuralla?
> *"Transformant històries de vida en evidència per a la recuperació."*

Si treballes en **salut mental, treball social o integració comunitària**, saps que la part més important de la teva feina queda escrita en notes d'evolució, memòries de tallers i observacions diàries. No obstant això, aquesta informació sovint es perd o és difícil d'analitzar en conjunt.

**LaMuralla Health** és un sistema d'intel·ligència cognitiva que llegeix i "comprèn" aquests textos. No només guarda la informació, sinó que **connecta els punts**:

*   **Evidència Automàtica:** Detecta si un usuari que assisteix al *Club Social* (Intervenció) millora la seva *Autoestima* (Resultat) basant-se en les notes diàries.
*   **Consultes Naturals:** Et permet preguntar: *"Quines activitats estan generant més vincle comunitari?"*
*   **Visió Holística:** Visualitza la xarxa de suports, símptomes i recursos de cada persona.

---

### 💻 Documentació Tècnica

#### Introducció: Motor GraphRAG
**LaMuralla Health** és un motor **GraphRAG** avançat. A diferència dels xatbots tradicionals, aquest sistema construeix un **Graf de Coneixement** dinàmic. Utilitza una arquitectura hexagonal en **Rust** per orquestrar la ingesta i el raonament lògic, emmagatzemant-ho tot a **Neo4j**.

#### 🖥️ Capacitats del Frontend (Interfície d'Usuari)
Interfície intuïtiva, reactiva i segura, renderitzada amb **Tera**, **Bootstrap 5** i **Vis.js**.

1.  **Panell de Visualització de Grafs:**
    *   Renderitzat de nodes per colors segons ontologia (`Person`, `Condition`, `Intervention`).
    *   Física de forces i navegació interactiva (Zoom, Pan, Selecció).

2.  **Xat Assistent amb Evidència:**
    *   **Sistema de Citacions `[1]`:** Referències interactives a les fonts originals.
    *   **Navegació Bidireccional:** En clicar una cita, el graf s'enfoca automàticament en les entitats esmentades.

3.  **Fitxa Tècnica d'Entitat:**
    *   Es desplega en seleccionar un node.
    *   **Mètriques:** Mostra el grau de connexió i rellevància.
    *   **Aïllament de Context:** Botó per veure només el subgraf contextual del node seleccionat.

4.  **Gestió i Seguretat (UI per Rols):**
    *   Interfície adaptativa segons si l'usuari és `User` o `Admin`.
    *   **Ingesta (Admin):** Pujada d'arxius amb barra de progrés en temps real.
    *   **Gestió d'Equip (Admin):** Alta i baixa de professionals.

---

## 🚀 Despliegue / Deployment

**Variables de Entorno (.env):**
```env
PORT=3000
JWT_SECRET=super_secret_key
NEO4J_URI=neo4j+s://xxxxxxxx.databases.neo4j.io
NEO4J_USER=neo4j
NEO4J_PASS=password
AI_PROVIDER=openai
AI_API_KEY=sk-...
```

**Run / Ejecutar:**
```bash
cargo run --release
```

---

## 👨‍💻 Autor & Contacto / Author & Contact

**Ángel A. Urbina**  
*Lead Architect & Developer*  
Projecte d'Innovació Tecnològica per al Tercer Sector Social.

🌐 **Website / Portfolio:** [https://angelurbinacv.netlify.app/](https://angelurbinacv.netlify.app/)  
📧 **GitHub:** [https://github.com/Angel-Urbina](https://github.com/Angel-Urbina)

© 2025 LaMuralla Health Project. All Rights Reserved.