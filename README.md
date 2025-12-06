# LaMuralla Health: Cognitive Recovery Engine

![Rust](https://img.shields.io/badge/Core-Rust_1.80+-orange?style=for-the-badge&logo=rust)
![Neo4j](https://img.shields.io/badge/Graph_DB-Neo4j_5+-008CC1?style=for-the-badge&logo=neo4j&logoColor=white)
![Security](https://img.shields.io/badge/Security-RBAC_%26_JWT-green?style=for-the-badge&logo=shield)
![Domain](https://img.shields.io/badge/Domain-Mental_Health-red?style=for-the-badge&logo=heart)

---

**[ 🇪🇸 Español ](#-español) | [ 🇺🇸 English ](#-english) | [ 🏴󠁥󠁳󠁣󠁴󠁿 Català ](#-català)**

---

<a name="es"></a>
## 🇪🇸 Español

### ❤️ Para el Sector Social: ¿Qué es LaMuralla?
> *"Transformando historias de vida en evidencia para la recuperación."*

Si trabajas en **salud mental, trabajo social o integración comunitaria**, sabes que la parte más importante de tu trabajo queda escrita en notas de evolución, memorias de actividades y observaciones diarias. Sin embargo, esa información a menudo se pierde o es difícil de analizar en conjunto.

**LaMuralla Health** es un asistente inteligente que lee y "comprende" esos textos. No solo guarda la información, sino que **conecta los puntos**:
*   Detecta automáticamente si un usuario que asiste al *Club Social* (Intervención) mejora su *Autoestima* (Resultado).
*   Te permite preguntar: *"¿Qué actividades están generando mayor vínculo comunitario?"*
*   Ayuda a justificar decisiones basándose en la evidencia real del día a día, apoyando el **Modelo de Recuperación**.

---

### 💻 Documentación Técnica

#### Introducción: Sistema de Inteligencia Cognitiva
**LaMuralla Health** es un motor RAG (Retrieval-Augmented Generation) avanzado que transforma texto libre clínico/social en un **Grafo de Conocimiento**. Utiliza una ontología estricta (*Persona, Condición, Intervención, Recurso, Resultado*) para estructurar datos no estructurados y permitir razonamiento complejo mediante IA.

#### 🛡️ Arquitectura de Seguridad (Nueva v2.0)
El sistema ha sido fortificado para cumplir con estándares de protección de datos y despliegue en producción:
1.  **RBAC (Control de Acceso Basado en Roles):**
    *   **Administrador:** Capacidad total (Ingesta de documentos, Configuración de IA, Gestión del Grafo).
    *   **Usuario (Profesional):** Acceso de solo lectura (Chat Asistente, Visualización, Exportación).
2.  **Autenticación Robusta:**
    *   Hashing de contraseñas con **Bcrypt**.
    *   Sesiones *stateless* mediante **JWT (JSON Web Tokens)**.
3.  **Protección de Sesión:**
    *   Cookies firmadas con atributos `HttpOnly`, `Secure` y `SameSite=Strict` para prevenir ataques XSS y CSRF.
4.  **Defensa Activa:**
    *   **Rate Limiting:** Protección contra ataques de fuerza bruta y DoS.
    *   **Secure Headers:** Cabeceras HTTP estrictas para prevenir Clickjacking y sniffing.

#### ✨ Capacidades Principales
1.  **GraphRAG Híbrido:** Búsqueda vectorial + Navegación de grafos para respuestas contextuales profundas.
2.  **Motor de Inferencia:** Deduce relaciones implícitas (ej. Transitividad entre participación y mejora de salud).
3.  **Interoperabilidad:** Exportación nativa a **JSON-LD** y **RDF/Turtle** (Web Semántica).
4.  **Rendimiento:** Backend escrito en **Rust (Axum)**, garantizando seguridad de memoria y velocidad.

#### 🚀 Despliegue (Docker & Render)

El proyecto está dockerizado para un despliegue sencillo en plataformas como Render o Kubernetes.

**Variables de Entorno Requeridas:**
```env
# Servidor
PORT=3000
RUST_LOG=info
JWT_SECRET=super_secret_key_base64

# Base de Datos (Neo4j AuraDB o Local)
NEO4J_URI=neo4j+s://xxxxxxxx.databases.neo4j.io
NEO4J_USER=neo4j
NEO4J_PASS=tu_password

# Credenciales Iniciales
ADMIN_USER=admin
ADMIN_PASS=password_seguro

# Inteligencia Artificial
AI_PROVIDER=openai
AI_API_KEY=sk-...
AI_MODEL=gpt-4o
```

**Ejecución Local:**
```bash
cargo run --release
```

---

<a name="en"></a>
## 🇺🇸 English

### ❤️ For the Social Sector: What is LaMuralla?
> *"Turning life stories into evidence for recovery."*

If you work in **mental health, social work, or community integration**, you know that the most vital part of your job is written in progress notes, workshop reports, and daily observations. However, that information is often lost or hard to analyze as a whole.

**LaMuralla Health** is an intelligent assistant that reads and "understands" those texts. It doesn't just store information; it **connects the dots**:
*   It automatically detects if a user attending the *Social Club* (Intervention) improves their *Self-esteem* (Outcome).
*   It allows you to ask: *"Which activities are generating the most community bonding?"*
*   It helps justify decisions based on real daily evidence, supporting the **Recovery Model**.

---

### 💻 Technical Documentation

#### Introduction: Cognitive Intelligence Engine
**LaMuralla Health** is an advanced RAG (Retrieval-Augmented Generation) engine that transforms unstructured clinical/social text into a **Knowledge Graph**. It uses a strict ontology (*Person, Condition, Intervention, Resource, Outcome*) to structure unstructured data and enable complex AI reasoning.

#### 🛡️ Security Architecture (New v2.0)
The system has been hardened to meet data protection standards and production deployment needs:
1.  **RBAC (Role-Based Access Control):**
    *   **Admin:** Full capabilities (Data Ingestion, AI Configuration, Graph Management).
    *   **User (Professional):** Read-only access (Chat Assistant, Visualization, Export).
2.  **Robust Authentication:**
    *   Password hashing using **Bcrypt**.
    *   Stateless sessions via **JWT (JSON Web Tokens)**.
3.  **Session Protection:**
    *   Signed cookies with `HttpOnly`, `Secure`, and `SameSite=Strict` attributes to prevent XSS and CSRF attacks.
4.  **Active Defense:**
    *   **Rate Limiting:** Protection against brute-force and DoS attacks.
    *   **Secure Headers:** Strict HTTP headers to prevent Clickjacking and sniffing.

#### ✨ Key Features
1.  **Hybrid GraphRAG:** Vector search + Graph traversal for deep contextual answers.
2.  **Inference Engine:** Deduces implicit relationships (e.g., Transitivity between participation and health outcomes).
3.  **Interoperability:** Native export to **JSON-LD** and **RDF/Turtle** (Semantic Web).
4.  **Performance:** Backend written in **Rust (Axum)**, ensuring memory safety and speed.

#### 🚀 Deployment (Docker & Render)

The project is Dockerized for easy deployment on platforms like Render or Kubernetes.

**Required Environment Variables:**
```env
# Server
PORT=3000
RUST_LOG=info
JWT_SECRET=super_secret_key_base64

# Database (Neo4j AuraDB or Local)
NEO4J_URI=neo4j+s://xxxxxxxx.databases.neo4j.io
NEO4J_USER=neo4j
NEO4J_PASS=your_password

# Initial Credentials
ADMIN_USER=admin
ADMIN_PASS=secure_password

# Artificial Intelligence
AI_PROVIDER=openai
AI_API_KEY=sk-...
AI_MODEL=gpt-4o
```

**Local Run:**
```bash
cargo run --release
```

---

<a name="ca"></a>
## 🏴󠁥󠁳󠁣󠁴󠁿 Català

### ❤️ Pel Sector Social: Què és LaMuralla?
> *"Transformant històries de vida en evidència per a la recuperació."*

Si treballes en **salut mental, treball social o integració comunitària**, saps que la part més important de la teva feina queda escrita en notes d'evolució, memòries de tallers i observacions diàries. No obstant això, aquesta informació sovint es perd o és difícil d'analitzar en conjunt.

**LaMuralla Health** és un assistent intel·ligent que llegeix i "comprèn" aquests textos. No només guarda la informació, sinó que **connecta els punts**:
*   Detecta automàticament si un usuari que assisteix al *Club Social* (Intervenció) millora la seva *Autoestima* (Resultat).
*   Et permet preguntar: *"Quines activitats estan generant més vincle comunitari?"*
*   Ajuda a justificar decisions basant-se en l'evidència real del dia a dia, donant suport al **Model de Recuperació**.

---

### 💻 Documentació Tècnica

#### Introducció: Motor d'Intel·ligència Cognitiva
**LaMuralla Health** és un motor RAG (Retrieval-Augmented Generation) avançat que transforma text lliure clínic/social en un **Graf de Coneixement**. Utilitza una ontologia estricta (*Persona, Condició, Intervenció, Recurs, Resultat*) per estructurar dades no estructurades i permetre raonament complex mitjançant IA.

#### 🛡️ Arquitectura de Seguretat (Nova v2.0)
El sistema ha estat fortificat per complir amb estàndards de protecció de dades i desplegament en producció:
1.  **RBAC (Control d'Accés Basat en Rols):**
    *   **Administrador:** Capacitat total (Ingesta de documents, Configuració d'IA, Gestió del Graf).
    *   **Usuari (Professional):** Accés de només lectura (Xat Assistent, Visualització, Exportació).
2.  **Autenticació Robusta:**
    *   Hashing de contrasenyes amb **Bcrypt**.
    *   Sessions *stateless* mitjançant **JWT (JSON Web Tokens)**.
3.  **Protecció de Sessió:**
    *   Cookies signades amb atributs `HttpOnly`, `Secure` i `SameSite=Strict` per prevenir atacs XSS i CSRF.
4.  **Defensa Activa:**
    *   **Rate Limiting:** Protecció contra atacs de força bruta i DoS.
    *   **Secure Headers:** Capçaleres HTTP estrictes per prevenir Clickjacking i sniffing.

#### ✨ Capacitats Principals
1.  **GraphRAG Híbrid:** Cerca vectorial + Navegació de grafs per a respostes contextuals profundes.
2.  **Motor d'Inferència:** Dedueix relacions implícites (ex. Transitivitat entre participació i millora de salut).
3.  **Interoperabilitat:** Exportació nativa a **JSON-LD** i **RDF/Turtle** (Web Semàntica).
4.  **Rendiment:** Backend escrit en **Rust (Axum)**, garantint seguretat de memòria i velocitat.

#### 🚀 Desplegament (Docker & Render)

El projecte està dockeritzat per a un desplegament senzill en plataformes com Render o Kubernetes.

**Variables d'Entorn Requerides:**
```env
# Servidor
PORT=3000
RUST_LOG=info
JWT_SECRET=super_secret_key_base64

# Base de Dades (Neo4j AuraDB o Local)
NEO4J_URI=neo4j+s://xxxxxxxx.databases.neo4j.io
NEO4J_USER=neo4j
NEO4J_PASS=la_teva_contrasenya

# Credencials Inicials
ADMIN_USER=admin
ADMIN_PASS=contrasenya_segura

# Intel·ligència Artificial
AI_PROVIDER=openai
AI_API_KEY=sk-...
AI_MODEL=gpt-4o
```

**Execució Local:**
```bash
cargo run --release
```

---

## 👨‍💻 Crèdits / Credits

**Ángel A. Urbina**  
*Architecture & Development*  
Projecte d'Innovació Tecnològica per al Tercer Sector Social.

© 2025 LaMuralla Health Project. All Rights Reserved.