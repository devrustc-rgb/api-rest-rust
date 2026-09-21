import pyautogui
import time
import pyperclip

# ====== COLE AQUI O BLOCO INTEIRO ======
texto = """

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// === Modelos ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tarefa {
    pub id: String,
    pub titulo: String,
    pub descricao: String,
    pub concluida: bool,
}

#[derive(Debug, Deserialize)]
pub struct NovaTarefa {
    pub titulo: String,
    pub descricao: String,
}

#[derive(Debug, Deserialize)]
pub struct AtualizarTarefa {
    pub titulo: Option<String>,
    pub descricao: Option<String>,
    pub concluida: Option<bool>,
}

pub type EstadoApp = Arc<Mutex<HashMap<String, Tarefa>>>;

// === Handlers ===

async fn listar_tarefas(State(estado): State<EstadoApp>) -> impl IntoResponse {
    let tarefas = estado.lock().unwrap();
    let lista: Vec<Tarefa> = tarefas.values().cloned().collect();
    Json(lista)
}

async fn buscar_tarefa(
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

async fn criar_tarefa(
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

async fn atualizar_tarefa(
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

async fn remover_tarefa(
    State(estado): State<EstadoApp>,
    Path(id): Path<String>,
) -> StatusCode {
    let mut tarefas = estado.lock().unwrap();
    if tarefas.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn rota_nao_encontrada() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "erro": "Rota não encontrada",
            "codigo": 404
        })),
    )
}

// === Aplicação ===

#[tokio::main]
async fn main() {
    let estado: EstadoApp = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/tarefas", get(listar_tarefas).post(criar_tarefa))
        .route(
            "/tarefas/{id}",
            get(buscar_tarefa)
                .put(atualizar_tarefa)
                .delete(remover_tarefa),
        )
        .fallback(rota_nao_encontrada)
        .with_state(estado);

    let endereco = "0.0.0.0:3000";
    println!("Servidor rodando em http://{}", endereco);

    let listener = tokio::net::TcpListener::bind(endereco).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

}



}"""

# ====== FIM ======

pyautogui.PAUSE = 0.400

for i in range(10, 0, -1):
    print(f"Iniciando em {i}...")
    time.sleep(1)

pyperclip.copy(texto)
pyautogui.hotkey('ctrl', 'v')
print("Pronto!")   