

#![allow(dead_code)]

// restante do código...
use axum::{Router, routing::get};



use crate::handlers::{
    atualizar_tarefa, buscar_tarefa, criar_tarefa, listar_tarefas, remover_tarefa,
    rota_nao_encontrada,
};
use crate::models::EstadoApp;


pub fn criar_roteador(estado: EstadoApp) -> Router {
    Router::new()
        .route("/tarefas", get(listar_tarefas).post(criar_tarefa))
        .route(
            "/tarefas/{id}", // Se usar Axum 0.7 ou anterior, mude para "/tarefas/:id"
            get(buscar_tarefa)
                .put(atualizar_tarefa)
                .delete(remover_tarefa),
        )
        .fallback(rota_nao_encontrada)
        .with_state(estado)
}
