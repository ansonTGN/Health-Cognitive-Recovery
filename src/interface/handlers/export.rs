// FILE: src/interface/handlers/export.rs
use axum::{Json, extract::State};
use std::sync::Arc;
use serde_json::{json, Value};
use crate::domain::errors::AppError;
use super::admin::AppState;

#[utoipa::path(
    get,
    path = "/api/export/jsonld",
    responses(
        (status = 200, description = "Export Knowledge Graph as JSON-LD", body = Value)
    ),
    tag = "export"
)]
pub async fn export_jsonld(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, AppError> {
    
    let graph = state.repo.export_full_knowledge_graph().await?;
    
    // Transformación a JSON-LD (Schema.org vocabulary mapping where possible)
    let context = json!({
        "@context": {
            "schema": "http://schema.org/",
            "mhealth": "http://ontologies.lamuralla.org/mental-health#",
            "Person": "schema:Person",
            "Condition": "schema:MedicalCondition",
            "Intervention": "schema:TherapeuticProcedure",
            "CommunityResource": "schema:Organization",
            "name": "schema:name",
            "category": "@type"
        }
    });

    let mut graph_nodes: Vec<Value> = Vec::new();

    // Mapeo de Entidades
    for node in graph.nodes {
        let type_uri = match node.category.as_str() {
            "Person" => "mhealth:Person",
            "Condition" => "mhealth:Condition",
            "Intervention" => "mhealth:Intervention",
            _ => "schema:Thing"
        };

        graph_nodes.push(json!({
            "@id": format!("mhealth:{}", node.name.replace(" ", "_")),
            "@type": type_uri,
            "name": node.name,
            "category": node.category
        }));
    }

    // Mapeo de Relaciones (Reificación simple o propiedades de objeto)
    // En JSON-LD simple, las relaciones suelen anidarse, pero en grafos planos usamos @graph
    // Aquí exportamos una estructura plana compatible con herramientas de visualización RDF.
    
    // Para simplificar, añadimos las relaciones como objetos independientes o propiedades
    // Para este caso, añadiremos un campo 'relations' customizado ya que JSON-LD estricto
    // requiere modificar los nodos originales para incluir las aristas como propiedades.
    
    let json_output = json!({
        "@context": context["@context"],
        "@graph": graph_nodes,
        "meta:edges": graph.edges // Extensión no estándar para facilitar parsing simple
    });

    Ok(Json(json_output))
}