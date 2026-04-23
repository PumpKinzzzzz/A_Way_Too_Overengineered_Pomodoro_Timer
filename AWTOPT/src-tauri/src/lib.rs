// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// The new architecture layers
pub mod contracts; // The Architects: Blueprints (DTOs) and Common Language (Traits)
pub mod warehouse; // The Keepers: External tools (crates) that match the blueprints
pub mod workshop;  // The Workers: Specialized workers + their internal models
pub mod desk;      // The Office: Orchestration only

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
