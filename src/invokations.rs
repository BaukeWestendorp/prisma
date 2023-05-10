use serde::{Deserialize, Serialize};

use common::project::PrismaProject;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

use crate::state::editor_project::EditorProject;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct UpdateLayerArgs {
    newProject: PrismaProject,
}

pub(crate) fn update_project(editor_project: &EditorProject) {
    spawn_local({
        let editor_project = editor_project.clone();
        async move {
            let js_value = serde_wasm_bindgen::to_value(&UpdateLayerArgs {
                newProject: editor_project.into(),
            })
            .unwrap();
            invoke("update_project", js_value).await;
        }
    });
}
