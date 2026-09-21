
- - - https://rustlang.com.br/projetos/api-rest-crud/


# vamos cria 8ma api para gerenciamentos de tarefas com as seguinte funcionalidadeas:

- get /tarefa - listar toda as tarefas:
- get/tarefas/:id Buscar um tarefa especifica:

- Post/tarefas - Cria uma nova tarefa:
- Put/ tarefa/:id Atualizar uma tarefa exitente:
- Delete/tarefa/:id Remover uma tarefa:

> serialixação e deserialização Json automatica:
> Tratamentos de erros com codigos HTTP apropriado(404, 400, 201,etc:)

> estado compartilhado thread-safe com Arc<Mutex<HashMap>>

Estrutura do projeto:

- api-rest-tust/
- Cargo.toml
- src/
- main.rs

- comandos itilizados no projetos:
- cargo add tokio --features  full
- cargo add axum
-  cargo add uuid --features v7 
- cargo add serde_json 
- cargo add serde --features derive
- - cargo add dioxus --features descktop






use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Intoresponse,
    routing::get,
    Json, Route,

};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tarefa {
    pub id: String,
    pub titulo: String,
    pub descricao: String,
    pub concuida: bool,
}

#[derive(Debug, Deserialize)]

pub  Struct NovaTarefa {
    pub titulo: String,
    pub decricao: String,
}

pub struct AtualizarTarefa {
    pub titulo: Option<String>,
    pub decricao: Option<String>,
    pub concluida: Options<bool>,

    
}
pub type EstadoApp = Arc<Mutex<Sytring, Tarefa>>>;



async fn listar_tarefas(State(estado): State<EstadoApp>) -> impl IntoResponse {
    let = estado.lock().unwrap();
    let lista: Vec<Tarefa> = tarefa.values().cloned().collect();
    Json(lista)
    
}




 - ############################################################################# -

 - 1 -

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;


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


- - - ############################################################################ 
- 2 - 


use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};


async fn listar_tarefas(
    State(estado): State<EstadoApp>,
) -> impl IntoResponse {
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

#############################################################################



use axum::Router;
use axum::routing::{get, post, put, delete};

// Handler para rotas não encontradas
async fn rota_nao_encontrada() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "erro": "Rota não encontrada",
            "codigo": 404
        })),
    )
}


fn criar_roteador(estado: EstadoApp) -> Router {
    Router::new()
        .route("/tarefas", get(listar_tarefas).post(criar_tarefa))
        .route(
            "/tarefas/{id}",
            get(buscar_tarefa)
                .put(atualizar_tarefa)
                .delete(remover_tarefa),
        )
        .fallback(rota_nao_encontrada)
        .with_state(estado)
}


##########################################################################



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


##########################################################
 
 
 - Invoke-RestMethod -Uri "http://localhost:3000/tarefas" -Method Post -ContentType "application/json" -Body '{"titulo": "Comprar leite", "descricao": "Ir ao supermercado"}'

- - listar tarefa
- - Invoke-RestMethod -Uri "http://localhost:3000/tarefas"

buscar tarfas:

Invoke-RestMethod -Uri "http://localhost:3000/tarefas/e58d97c0-0744-474c-9fcf-bea7c8568a04"

MARCAR TAREFA:

Invoke-RestMethod -Uri "http://localhost:3000/tarefas/e58d97c0-0744-474c-9fcf-bea7c8568a04" -Method Put -ContentType "application/json" -Body '{"concluida": true}'


Invoke-RestMethod -Uri "http://localhost:3000/tarefas/e58d97c0-0744-474c-9fcf-bea7c8568a04" -Method Delete