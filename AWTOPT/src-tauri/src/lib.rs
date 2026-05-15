pub mod contracts;
pub mod desk;
pub mod warehouse;
pub mod workshop;

use contracts::{SessionStatsDto, SettingsDto, SettingsUpdateDto, TimerStatusDto};
use desk::PomodoroOrchestrator;
use std::sync::Mutex;
use tauri::{Manager, State};
use warehouse::{PersistenceTool, TauriNotifierTool};
use workshop::{NotifierWorker, PersistenceWorker, SessionWorker, SettingsWorker, TimerWorker};

#[tauri::command]
fn start_timer(state: State<Mutex<PomodoroOrchestrator>>) -> Result<TimerStatusDto, String> {
    state
        .lock()
        .map_err(|_| "Failed to acquire state lock".to_string())?
        .handle_start_timer()
}

#[tauri::command]
fn pause_timer(state: State<Mutex<PomodoroOrchestrator>>) -> Result<TimerStatusDto, String> {
    state
        .lock()
        .map_err(|_| "Failed to acquire state lock".to_string())?
        .handle_pause_timer()
}

#[tauri::command]
fn resume_timer(state: State<Mutex<PomodoroOrchestrator>>) -> Result<TimerStatusDto, String> {
    state
        .lock()
        .map_err(|_| "Failed to acquire state lock".to_string())?
        .handle_resume_timer()
}

#[tauri::command]
fn reset_timer(state: State<Mutex<PomodoroOrchestrator>>) -> Result<TimerStatusDto, String> {
    state
        .lock()
        .map_err(|_| "Failed to acquire state lock".to_string())?
        .handle_reset_timer()
}

#[tauri::command]
fn tick(state: State<Mutex<PomodoroOrchestrator>>) -> Result<TimerStatusDto, String> {
    state
        .lock()
        .map_err(|_| "Failed to acquire state lock".to_string())?
        .handle_tick()
}

#[tauri::command]
fn update_settings(
    state: State<Mutex<PomodoroOrchestrator>>,
    request: SettingsUpdateDto,
) -> Result<SettingsDto, String> {
    state
        .lock()
        .map_err(|_| "Failed to acquire state lock".to_string())?
        .handle_update_settings(request)
}

#[tauri::command]
fn get_settings(state: State<Mutex<PomodoroOrchestrator>>) -> Result<SettingsDto, String> {
    state
        .lock()
        .map_err(|_| "Failed to acquire state lock".to_string())?
        .handle_get_settings()
}

#[tauri::command]
fn get_timer_status(state: State<Mutex<PomodoroOrchestrator>>) -> Result<TimerStatusDto, String> {
    state
        .lock()
        .map_err(|_| "Failed to acquire state lock".to_string())?
        .handle_get_timer_status()
}

#[tauri::command]
fn get_session_stats(state: State<Mutex<PomodoroOrchestrator>>) -> Result<SessionStatsDto, String> {
    state
        .lock()
        .map_err(|_| "Failed to acquire state lock".to_string())?
        .handle_get_session_stats()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let persistence_tool = PersistenceTool::new(app.handle())?;
            let persistence_worker = PersistenceWorker::new(persistence_tool);

            let notifier_tool = TauriNotifierTool::new(app.handle().clone());
            let notifier_worker = NotifierWorker::new(notifier_tool);

            let (settings_worker, session_worker) = if persistence_worker.has_saved_state() {
                let state = persistence_worker.load_app_state()?;
                (
                    SettingsWorker::from_save(state.settings),
                    SessionWorker::from_save(state.session_stats),
                )
            } else {
                (SettingsWorker::new(), SessionWorker::new())
            };

            let timer_worker = TimerWorker::new(settings_worker.get_domain_settings());

            let orchestrator = PomodoroOrchestrator::new(
                timer_worker,
                settings_worker,
                session_worker,
                persistence_worker,
                notifier_worker,
            );

            app.manage(Mutex::new(orchestrator));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_timer,
            pause_timer,
            resume_timer,
            reset_timer,
            tick,
            update_settings,
            get_settings,
            get_timer_status,
            get_session_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
