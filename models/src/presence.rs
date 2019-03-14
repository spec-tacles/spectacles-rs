//! Structures related to a user's presence on Discord.
use std::fmt::{Display, Formatter, Result as FmtResult};

use serde_json::Error as JsonError;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::gateway::{Opcodes, SendablePacket, SendPacket};
use crate::Snowflake;

/// Data about an activity that the user is participating in.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Activity {
    /// The name of the activity.
    pub name: String,
    /// The type of activity.
    #[serde(rename = "type")]
    pub kind: ActivityType,
    /// If type is 1, the stream URL.
    #[serde(default)]
    pub url: String,
    /// Timestamps for this activity.
    #[serde(default)]
    pub timestamps: ActivityTimestamps,
    /// The application ID for the game, if any.
    pub application_id: Snowflake,
    /// What the player is currently doing.
    pub details: Option<String>,
    /// The user's current party status.
    pub state: Option<String>,
    /// The player's current party.
    #[serde(default)]
    pub party: ActivityParty,
    /// The Rich Presence assets.
    #[serde(default)]
    pub assets: ActivityAssets,
    /// The Rich Presence secrets.
    #[serde(default)]
    pub secrets: ActivitySecrets,
    /// Whether or not the activity is in a current game session.
    #[serde(default)]
    pub instance: bool,
    /// Activity flags.
    #[serde(default)]
    pub flags: i32
}

impl Activity {
    /// Creates a new activity that is ready to be sent to the Discord Gateway.
    pub fn new(kind: ActivityType, name: &str, url: &str) -> Self {
        Activity {
            name: name.to_string(),
            kind,
            url: url.to_string(),
            timestamps: ActivityTimestamps::default(),
            application_id: String::new(),
            details: None,
            state: None,
            party: ActivityParty::default(),
            assets: ActivityAssets::default(),
            secrets: ActivitySecrets::default(),
            instance: false,
            flags: i32::default()
        }
    }
}

/// Represents the activity of the current client.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ClientActivity {
    /// The name of the current activity.
    pub name: String,
    /// The activity type.
    #[serde(rename = "type")]
    pub kind: ActivityType,
    /// The stream URL, if streaming.
    pub url: Option<String>,
}

impl ClientActivity {
    /// Create an activity with the PLAYING prefix.
    pub fn game(name: &str) -> Self {
        Self {
            name: name.to_string(),
            kind: ActivityType::Game,
            url: None
        }
    }

    /// Create an activity with the STREAMING prefix.
    pub fn streaming(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            kind: ActivityType::Streaming,
            url: Some(url.to_string())
        }
    }

    /// Create an activity with the LISTENING prefix.
    pub fn listening(name: &str) -> Self {
        Self {
            name: name.to_string(),
            kind: ActivityType::Listening,
            url: None
        }
    }
}

/// The current presence of the connected Client.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ClientPresence {
    /// Milliseconds that the client went idle.
    pub since: Option<i32>,
    /// The user's current activity if any.
    pub game: Option<ClientActivity>,
    /// The status of the user.
    pub status: String,
    /// Whether or not the client is AFK.
    pub afk: bool
}

/// Represents an Activity's timestamps.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ActivityTimestamps {
    /// When the activity started, in milliseconds.
    pub start: i32,
    /// When the activity ends, in milliseconds.
    pub end: i32
}

/// Information about the player's current party.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ActivityParty {
    /// The ID for this party.
    #[serde(default)]
    pub id: String,
    /// The party's current and maximum size.
    #[serde(default)]
    pub size: [i32; 2]
}

/// Rich Presence image and text assets.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ActivityAssets {
    /// The ID of the large image of this activity.
    #[serde(default)]
    pub large_image: String,
    /// The large image hover text.
    #[serde(default)]
    pub large_text: String,
    /// The ID of the small image of this activity.
    #[serde(default)]
    pub small_image: String,
    /// The small image hover text.
    #[serde(default)]
    pub small_text: String
}

/// Rich Presence secrets, used for joining and spectating.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ActivitySecrets {
    /// The secret for joining a party.
    #[serde(default)]
    pub join: String,
    /// The secret for spectating a game.
    #[serde(default)]
    pub spectate: String,
    /// The secret for an instanced match.
    #[serde(rename = "match", default)]
    pub match_type: String
}

/// The current presence of a user.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Presence {
    /// Milliseconds that the client went idle.
    pub since: Option<i32>,
    /// The user's current activity if any.
    pub game: Option<Activity>,
    /// The status of the user.
    pub status: String,
    /// Whether or not the client is AFK.
    pub afk: bool
}

impl SendablePacket for ClientPresence {
    fn to_json(self) -> Result<String, JsonError> {
        serde_json::to_string(&SendPacket {
            op: Opcodes::StatusUpdate,
            d: self
        })
    }
}
/// A list of possible activity types.
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum ActivityType {
    Game,
    Streaming,
    Listening
}

impl Default for ActivityType {
    fn default() -> Self {
        ActivityType::Game
    }
}
/// A list of possible statuses.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Status {
    Online,
    DnD,
    Idle,
    Invisible,
    Offline
}


impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Status::Online => write!(f, "online"),
            Status::DnD => write!(f, "dnd"),
            Status::Idle => write!(f, "idle"),
            Status::Invisible => write!(f, "invisible"),
            Status::Offline => write!(f, "offline")
        }
    }
}