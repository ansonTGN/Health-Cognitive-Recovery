Aquí tienes el `README.md` profesional, estructurado en tres idiomas y diseñado específicamente para el contexto de la ONG y el código refactorizado.

---

# 🧠 LaMuralla Health: Cognitive Recovery Engine

![Rust](https://img.shields.io/badge/Core-Rust_1.80+-orange?style=for-the-badge&logo=rust)
![Neo4j](https://img.shields.io/badge/Graph_DB-Neo4j_5+-008CC1?style=for-the-badge&logo=neo4j&logoColor=white)
![Domain](https://img.shields.io/badge/Domain-Mental_Health-red?style=for-the-badge&logo=heart)
![AI](https://img.shields.io/badge/AI-Ontological_Reasoning-8A2BE2?style=for-the-badge)

---

**[ 🇪🇸 Español ](#-español) | [ 🇺🇸 English ](#-english) | [ 🏴󠁥󠁳󠁣󠁴󠁿 Català ](#-català)**

---

<a name="es"></a>
## 🇪🇸 Español

### 🏥 Introducción: Tecnología para el Modelo de Recuperación
**LaMuralla Health** no es simplemente un gestor documental; es un **Sistema de Inteligencia Cognitiva** diseñado específicamente para el ámbito de la **Salut Mental Comunitaria**.

En un sector donde la información cualitativa (notas de evolución, memorias de talleres, dinámicas de grupos) es vital pero difícil de estructurar, esta herramienta permite transformar texto libre en un **Grafo de Conocimiento** vivo. Su objetivo principal es visibilizar las conexiones invisibles del **Modelo de Recuperación**: vinculando *Intervenciones* (ej. Club Social) con *Resultados* (ej. Empoderamiento, Inclusión), facilitando así una toma de decisiones basada en la evidencia psicosocial y no solo en el diagnóstico clínico.

### ✨ Capacidades Principales
1.  **Ontología Especializada:** El sistema no "adivina"; aplica una estructura ontológica estricta (*Persona, Condición, Intervención, Recurso Comunitario, Resultado*) para organizar la información.
2.  **RAG Híbrido (GraphRAG):** Combina búsqueda vectorial (similitud semántica) con navegación de grafos para responder preguntas complejas con contexto profundo.
3.  **Motor de Inferencia:** Un módulo de IA analiza el grafo para descubrir relaciones implícitas (ej. "Si A participa en B y B promueve C, entonces A está trabajando en C").
4.  **Interoperabilidad Semántica:** Capacidad nativa para exportar el conocimiento adquirido en formato **JSON-LD**, permitiendo la integración con otros sistemas de salud y estándares de datos abiertos.
5.  **Privacidad y Rendimiento:** Backend de alto rendimiento escrito en **Rust**, garantizando velocidad y tipado seguro de datos.

### 🛠️ Stack Tecnológico
*   **Core:** Rust (Axum, Tokio).
*   **Base de Datos:** Neo4j (Almacenamiento híbrido: Vectorial + Grafo).
*   **IA & LLM:** Rig-Core (Orquestación) + OpenAI/Groq.
*   **Frontend:** Tera (SSR), Bootstrap 5, Vis.js (Visualización interactiva).

### 🚀 Instalación Rápida

1.  **Requisitos:** Tener instalado Rust, Docker (opcional) y una instancia de Neo4j.
2.  **Configuración:**
    Crea un archivo `.env` basado en el ejemplo:
    ```env
    NEO4J_URI=bolt://localhost:7687
    NEO4J_USER=neo4j
    NEO4J_PASS=tu_password
    AI_API_KEY=sk-...
    ```
3.  **Ejecución:**
    ```bash
    cargo run --release
    ```
    Accede a la plataforma en: `http://localhost:3000`

---

<a name="en"></a>
## 🇺🇸 English

### 🏥 Introduction: Technology for the Recovery Model
**LaMuralla Health** is more than a document management system; it is a **Cognitive Intelligence Engine** tailored for **Community Mental Health**.

In a sector where qualitative information (progress notes, workshop reports, group dynamics) is vital yet hard to structure, this tool transforms unstructured text into a living **Knowledge Graph**. Its core mission is to unveil the invisible connections within the **Recovery Model**: linking *Interventions* (e.g., Social Clubs) with *Outcomes* (e.g., Empowerment, Inclusion), thereby enabling decision-making based on psycho-social evidence rather than just clinical diagnosis.

### ✨ Key Features
1.  **Specialized Ontology:** The system enforces a strict ontological structure (*Person, Condition, Intervention, Community Resource, Outcome*) to organize data precisely.
2.  **Hybrid RAG (GraphRAG):** Combines vector search (semantic similarity) with graph traversal to answer complex questions with deep context.
3.  **Inference Engine:** An AI module analyzes the graph to discover implicit relationships (e.g., Transitivity between participation and health outcomes).
4.  **Semantic Interoperability:** Native capability to export acquired knowledge in **JSON-LD** format, allowing integration with other health systems and open data standards.
5.  **Privacy & Performance:** High-performance backend written in **Rust**, ensuring speed and type safety.

### 🛠️ Tech Stack
*   **Core:** Rust (Axum, Tokio).
*   **Database:** Neo4j (Hybrid storage: Vector + Graph).
*   **AI & LLM:** Rig-Core (Orchestration) + OpenAI/Groq.
*   **Frontend:** Tera (SSR), Bootstrap 5, Vis.js (Interactive visualization).

### 🚀 Quick Start

1.  **Prerequisites:** Rust, Docker (optional), and a running Neo4j instance.
2.  **Configuration:**
    Create a `.env` file:
    ```env
    NEO4J_URI=bolt://localhost:7687
    NEO4J_USER=neo4j
    NEO4J_PASS=your_password
    AI_API_KEY=sk-...
    ```
3.  **Run:**
    ```bash
    cargo run --release
    ```
    Access the platform at: `http://localhost:3000`

---

<a name="ca"></a>
## 🏴󠁥󠁳󠁣󠁴󠁿 Català

### 🏥 Introducció: Tecnologia pel Model de Recuperació
**LaMuralla Health** no és simplement un gestor documental; és un **Motor d'Intel·ligència Cognitiva** dissenyat específicament per a l'àmbit de la **Salut Mental Comunitària**.

En un sector on la informació qualitativa (notes d'evolució, memòries de tallers, dinàmiques de grups) és vital però difícil d'estructurar, aquesta eina permet transformar text lliure en un **Graf de Coneixement** viu. El seu objectiu principal és visibilitzar les connexions invisibles del **Model de Recuperació**: vinculant *Intervencions* (ex. Club Social) amb *Resultats* (ex. Empoderament, Inclusió), facilitant així una presa de decisions basada en l'evidència psicosocial i no només en el diagnòstic clínic.

### ✨ Capacitats Principals
1.  **Ontologia Especialitzada:** El sistema aplica una estructura ontològica estricta (*Persona, Condició, Intervenció, Recurs Comunitari, Resultat*) per organitzar la informació.
2.  **RAG Híbrid (GraphRAG):** Combina cerca vectorial (similitud semàntica) amb navegació de grafs per respondre preguntes complexes amb context profund.
3.  **Motor d'Inferència:** Un mòdul d'IA analitza el graf per descobrir relacions implícites (ex. "Si A participa en B i B promou C, aleshores A està treballant en C").
4.  **Interoperabilitat Semàntica:** Capacitat nativa per exportar el coneixement adquirit en format **JSON-LD**, permetent la integració amb altres sistemes de salut i estàndards de dades obertes.
5.  **Privacitat i Rendiment:** Backend d'alt rendiment escrit en **Rust**, garantint velocitat i seguretat de dades.

### 🛠️ Pila Tecnològica
*   **Nucli:** Rust (Axum, Tokio).
*   **Base de Dades:** Neo4j (Emmagatzematge híbrid: Vectorial + Graf).
*   **IA & LLM:** Rig-Core (Orquestració) + OpenAI/Groq.
*   **Frontend:** Tera (SSR), Bootstrap 5, Vis.js (Visualització interactiva).

### 🚀 Instal·lació Ràpida

1.  **Requisits:** Tenir instal·lat Rust, Docker (opcional) i una instància de Neo4j.
2.  **Configuració:**
    Crea un fitxer `.env`:
    ```env
    NEO4J_URI=bolt://localhost:7687
    NEO4J_USER=neo4j
    NEO4J_PASS=la_teva_contrasenya
    AI_API_KEY=sk-...
    ```
3.  **Execució:**
    ```bash
    cargo run --release
    ```
    Accedeix a la plataforma a: `http://localhost:3000`

---

## 👨‍💻 Crèdits / Credits

**Ángel A. Urbina**  
*Architecture & Development*  
Projecte d'Innovació Tecnològica per al Tercer Sector Social.

© 2025 LaMuralla Health Project. All Rights Reserved.