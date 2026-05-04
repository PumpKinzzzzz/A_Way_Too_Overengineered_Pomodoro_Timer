use crate::contracts::NotifierTrait;

pub struct TauriNotifierTool {
    app_handle: tauri::AppHandle,
}

impl TauriNotifierTool {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self { app_handle }
    }
}

impl NotifierTrait for TauriNotifierTool {
    fn notify(&self, title: &str, message: &str) {
        use tauri_plugin_notification::NotificationExt;

        let _ = self
            .app_handle
            .notification()
            .builder()
            .title(title)
            .body(message)
            .show();
    }
}
