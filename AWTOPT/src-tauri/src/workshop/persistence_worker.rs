use crate::contracts::AppSave;
use crate::warehouse::PersistenceTool;

pub struct PersistenceWorker {
    tool: PersistenceTool,
}

impl PersistenceWorker {
    pub fn new(tool: PersistenceTool) -> Self {
        Self { tool }
    }

    pub fn save_app_state(&self, state: &AppSave) -> Result<(), String> {
        self.tool.save_state(state)
    }

    pub fn load_app_state(&self) -> Result<AppSave, String> {
        self.tool.load_state()
    }

    pub fn has_saved_state(&self) -> bool {
        self.tool.state_exists()
    }

    pub fn clear_saved_state(&self) -> Result<(), String> {
        self.tool.delete_state()
    }
}
