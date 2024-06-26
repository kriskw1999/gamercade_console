use tokio::sync::mpsc::{channel, Receiver};

use crate::local_directory::{PermissionLevel, PermissionLevelId, Tag, TagId};

use super::{
    AuthManager, AuthResponse, AuthorManager, GameManager, GameResponse, HttpManager, HttpResponse,
    PlatformManager, PlatformResponse, TagManager, SUPER_TASK_CHANNEL_SIZE,
};

#[derive(Debug)]
pub enum TaskNotification {
    GlobalTags(Vec<(TagId, Tag)>),
    GlobalPermissionLevels(Vec<(PermissionLevelId, PermissionLevel)>),
    //AuthStateChanged(AuthState),
    //LoginFailed,

    // Auth
    AuthResponse(AuthResponse),

    // Game
    GameResponse(GameResponse),

    // Platform
    PlatformResponse(PlatformResponse),

    // Http
    HttpResponse(HttpResponse),
}

#[derive(Debug)]
pub struct DownloadRomComplete {
    pub game_id: i64,
    pub data: Vec<u8>,
}

pub struct SuperTaskManager {
    pub events: Receiver<TaskNotification>,
    pub tags: TagManager,
    pub author: AuthorManager,
    pub auth: AuthManager,
    pub http: HttpManager,
    pub game: GameManager,
    pub platform: PlatformManager,
}

impl Default for SuperTaskManager {
    fn default() -> Self {
        let (event_tx, events) = channel(SUPER_TASK_CHANNEL_SIZE);

        Self {
            tags: TagManager::new(event_tx.clone()),
            author: AuthorManager::new(event_tx.clone()),
            auth: AuthManager::new(event_tx.clone()),
            http: HttpManager::new(event_tx.clone()),
            game: GameManager::new(event_tx.clone()),
            platform: PlatformManager::new(event_tx.clone()),
            events,
        }
    }
}
