use common::project::Project;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

use crate::editor::editor_project::EditorProject;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_snake_case)]
struct UpdateLayerArgs {
    newProject: Project,
}

pub(crate) fn update_project(editor_project: EditorProject) {
    spawn_local(async move {
        let js_value = serde_wasm_bindgen::to_value(&UpdateLayerArgs {
            newProject: editor_project.into(),
        })
        .unwrap();
        gloo_console::log!(js_value.clone());
        invoke("update_project", js_value).await;
    });
}
