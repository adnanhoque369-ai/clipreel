// ClipReel - AI Video Clipping Tool
// Stage model stub

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stage {
    Queued,
    Downloading,
    Transcribing,
    Ranking,
    AwaitingReview,
    StyleSelected,
    Rendering,
    AwaitingExport,
    Exporting,
    Completed,
    Failed,
    Discarded,
}
