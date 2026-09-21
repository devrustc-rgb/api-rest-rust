# 🚀 API Rest para Gerenciamento de Tarefas em Rust

Uma API REST robusta desenvolvida em **Rust** para o gerenciamento de tarefas, utilizando boas práticas e controle de concorrência.

## 🛠️ Funcionalidades e Rotas

A API conta com os seguintes endpoints para manipulação das tarefas:

* **GET** `/tarefas` — Listar todas as tarefas
* **GET** `/tarefas/:id` — Buscar uma tarefa específica
* **POST** `/tarefas` — Criar uma nova tarefa
* **PUT** `/tarefas/:id` — Atualizar uma tarefa existente
* **DELETE** `/tarefas/:id` — Remover uma tarefa existente

## ⚙️ Diferenciais Técnicos

* **Serialização e Deserialização:** Manipulação automática de JSON.
* **Tratamento de Erros:** Respostas com códigos HTTP apropriados (`200 OK`, `201 Created`, `400 Bad Request`, `404 Not Found`).
* **Concorrência Segura:** Estado compartilhado *thread-safe* na memória utilizando `Arc<Mutex<HashMap>>`.

## 📂 Estrutura do Projeto

```text
api-rest-rust/
├── Cargo.toml
└── src/
    └── main.rs
```
