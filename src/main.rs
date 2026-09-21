
#![windows_subsystem = "windows"] // Oculta a janela do terminal no Windows ao executar

mod handlers;
mod models;
mod routes;


//#![windows_subsystem = "windows"]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tarefa {
    pub id: String,
    pub titulo: String,
    pub descricao: String,
    pub concluida: bool,
}

fn main() {
    LaunchBuilder::desktop().launch(App);
}

#[component]
fn App() -> Element {
    let mut tarefas = use_signal(Vec::<Tarefa>::new);
    let mut novo_titulo = use_signal(String::new);
    let mut nova_descricao = use_signal(String::new);

    let adicionar_tarefa = move |_| {
        let titulo = novo_titulo.read().trim().to_string();
        let descricao = nova_descricao.read().trim().to_string();

        if !titulo.is_empty() {
            tarefas.write().push(Tarefa {
                id: Uuid::new_v4().to_string(),
                titulo,
                descricao,
                concluida: false,
            });
            novo_titulo.set(String::new());
            nova_descricao.set(String::new());
        }
    };

    rsx! {
        // Importa o Tailwind CSS via CDN direto no componente
        script { src: "https://cdn.tailwindcss.com" }

        div { class: "min-h-screen bg-slate-100 py-10 px-4 flex justify-center items-start font-sans",
            div { class: "w-full max-w-lg bg-white rounded-2xl shadow-xl border border-slate-200 p-6 space-y-6",
                
                // Cabeçalho
                header { class: "border-b border-slate-100 pb-4",
                    h1 { class: "text-2xl font-bold text-slate-800 tracking-tight", "Gerenciador de Tarefas" }
                    p { class: "text-sm text-slate-500 mt-1", "Dioxus + Tailwind CSS" }
                }

                // Formulário de Entrada
                div { class: "space-y-3",
                    input {
                        r#type: "text",
                        class: "w-full px-4 py-2.5 rounded-lg border border-slate-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent text-sm transition",
                        placeholder: "Título da tarefa...",
                        value: "{novo_titulo}",
                        oninput: move |e| novo_titulo.set(e.value())
                    }
                    input {
                        r#type: "text",
                        class: "w-full px-4 py-2.5 rounded-lg border border-slate-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent text-sm transition",
                        placeholder: "Descrição (opcional)...",
                        value: "{nova_descricao}",
                        oninput: move |e| nova_descricao.set(e.value())
                    }
                    button {
                        class: "w-full bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-2.5 rounded-lg text-sm shadow-md transition duration-200 active:scale-[0.98]",
                        onclick: adicionar_tarefa,
                        "Adicionar Tarefa"
                    }
                }

                // Lista de Tarefas
                div { class: "space-y-3 pt-2",
                    if tarefas.read().is_empty() {
                        p { class: "text-center text-sm text-slate-400 py-6 italic", "Nenhuma tarefa cadastrada." }
                    }

                    for tarefa in tarefas.read().iter().cloned() {
                        {
                            let id = tarefa.id.clone();
                            let id_toggle = tarefa.id.clone();
                            
                            rsx! {
                                div {
                                    key: "{tarefa.id}",
                                    class: "flex items-center justify-between p-3.5 bg-slate-50 border border-slate-200 rounded-xl hover:bg-slate-100/80 transition",
                                    
                                    div { class: "flex items-start gap-3 pr-2",
                                        input {
                                            r#type: "checkbox",
                                            class: "mt-1 h-4 w-4 rounded border-slate-300 text-indigo-600 focus:ring-indigo-500 cursor-pointer",
                                            checked: tarefa.concluida,
                                            onchange: move |_| {
                                                let mut list = tarefas.write();
                                                if let Some(t) = list.iter_mut().find(|item| item.id == id_toggle) {
                                                    t.concluida = !t.concluida;
                                                }
                                            }
                                        }
                                        div { class: "flex flex-col",
                                            span {
                                                class: if tarefa.concluida { "line-through text-slate-400 font-medium text-sm" } else { "text-slate-700 font-medium text-sm" },
                                                "{tarefa.titulo}"
                                            }
                                            if !tarefa.descricao.is_empty() {
                                                span { class: "text-xs text-slate-500 mt-0.5", "{tarefa.descricao}" }
                                            }
                                        }
                                    }

                                    button {
                                        class: "text-xs font-semibold text-rose-500 hover:text-rose-700 hover:bg-rose-50 px-2.5 py-1.5 rounded-lg transition",
                                        onclick: move |_| {
                                            tarefas.write().retain(|item| item.id != id);
                                        },
                                        "Eliminar"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}