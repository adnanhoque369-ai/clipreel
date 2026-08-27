// ClipReel - AI Video Clipping Tool
// Clip model stub

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: String,
}
