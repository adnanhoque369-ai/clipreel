// ClipReel - AI Video Clipping Tool
// Job commands stub

use tauri::State;

#[tauri::command]
pub fn ping() -> String {
    "pong".to_string()
}
