// frontend/src/main.rs
use yew::prelude::*;
use web_sys::HtmlInputElement;
use serde::{Serialize, Deserialize};
use std::rc::Rc;
use gloo_net::http::Request;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

#[function_component(App)]
fn app() -> Html {
    let todos = use_state(|| vec![]);
    let input_ref = use_node_ref();
    
    let on_add = {
        let todos = todos.clone();
        let input_ref = input_ref.clone();
        
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let input = input_ref.cast::<HtmlInputElement>().unwrap();
            let text = input.value();
            
            if !text.is_empty() {
                let new_todo = Todo {
                    id: (todos.len() + 1) as usize,
                    text,
                    completed: false,
                };
                
                // API call to create todo
                wasm_bindgen_futures::spawn_local({
                    let todos = todos.clone();
                    let new_todo = new_todo.clone();
                    
                    async move {
                        let response = Request::post("/api/todos")
                            .json(&new_todo)
                            .unwrap()
                            .send()
                            .await
                            .unwrap()
                            .json::<Todo>()
                            .await
                            .unwrap();
                            
                        let mut updated_todos = (*todos).clone();
                        updated_todos.push(response);
                        todos.set(updated_todos);
                    }
                });
                
                input.set_value("");
            }
        })
    };
    
    // Load todos on component mount
    {
        let todos = todos.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_todos = Request::get("/api/todos")
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<Todo>>()
                    .await
                    .unwrap();
                todos.set(fetched_todos);
            });
            || ()
        }, ());
    }
    
    html! {
        <div class="container mx-auto p-4 max-w-md">
            <h1 class="text-2xl font-bold mb-4">{"Todo App"}</h1>
            
            <div class="flex mb-4">
                <input 
                    ref={input_ref}
                    type="text" 
                    placeholder="Add a new task..." 
                    class="flex-grow p-2 border rounded-l focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
                <button 
                    onclick={on_add}
                    class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-r"
                >
                    {"Add"}
                </button>
            </div>
            
            <ul class="space-y-2">
                {
                    todos.iter().map(|todo| {
                        let todo_id = todo.id;
                        let todos_state = todos.clone();
                        
                        let on_delete = Callback::from(move |_| {
                            let todos = todos_state.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                let _ = Request::delete(&format!("/api/todos/{}", todo_id))
                                    .send()
                                    .await;
                                    
                                let updated_todos = (*todos)
                                    .clone()
                                    .into_iter()
                                    .filter(|t| t.id != todo_id)
                                    .collect::<Vec<_>>();
                                    
                                todos.set(updated_todos);
                            });
                        });
                        
                        html! {
                            <li class="flex items-center justify-between p-3 bg-gray-100 rounded">
                                <span>{&todo.text}</span>
                                <button 
                                    onclick={on_delete}
                                    class="text-red-500 hover:text-red-700"
                                >
                                    {"Delete"}
                                </button>
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
