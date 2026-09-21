
#![allow(dead_code)] //sileciar os warning:

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
