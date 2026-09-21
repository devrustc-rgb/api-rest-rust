use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::models::{AtualizarTarefa, EstadoApp, NovaTarefa, Tarefa};

pub async fn listar_tarefas(State(estado): State<EstadoApp>) -> impl IntoResponse {
    let tarefas = estado.lock().unwrap();
    let lista: Vec<Tarefa> = tarefas.values().cloned().collect();
    Json(lista)
}

pub async fn buscar_tarefa(
    State(estado): State<EstadoApp>,
    Path(id): Path<String>,
) -> Result<Json<Tarefa>, StatusCode> {
    let tarefas = estado.lock().unwrap();
    tarefas
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn criar_tarefa(
    State(estado): State<EstadoApp>,
    Json(nova): Json<NovaTarefa>,
) -> impl IntoResponse {
    let tarefa = Tarefa {
        id: Uuid::new_v4().to_string(),
        titulo: nova.titulo,
        descricao: nova.descricao,
        concluida: false,
    };
    let mut tarefas = estado.lock().unwrap();
    tarefas.insert(tarefa.id.clone(), tarefa.clone());
    (StatusCode::CREATED, Json(tarefa))
}

pub async fn atualizar_tarefa(
    State(estado): State<EstadoApp>,
    Path(id): Path<String>,
    Json(dados): Json<AtualizarTarefa>,
) -> Result<Json<Tarefa>, StatusCode> {
    let mut tarefas = estado.lock().unwrap();
    let tarefa = tarefas.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    if let Some(titulo) = dados.titulo {
        tarefa.titulo = titulo;
    }
    if let Some(descricao) = dados.descricao {
        tarefa.descricao = descricao;
    }
    if let Some(concluida) = dados.concluida {
        tarefa.concluida = concluida;
    }

    Ok(Json(tarefa.clone()))
}

pub async fn remover_tarefa(State(estado): State<EstadoApp>, Path(id): Path<String>) -> StatusCode {
    let mut tarefas = estado.lock().unwrap();
    if tarefas.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn rota_nao_encontrada() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "erro": "Rota não encontrada",
            "codigo": 404
        })),
    )
}
