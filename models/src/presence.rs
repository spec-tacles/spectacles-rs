//! Structures related to a user's presence on Discord.
use std::fmt::{Display, Formatter, Result as FmtResult};

use serde_json::Error as JsonError;

use crate::gateway::{Opcodes, SendablePacket, SendPacket};

/// Data about an activity that the user is participating in.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Activity {
    /// The name of the activity.
    pub activity: String,
    /// The type of activity.
    #[serde(rename = "type")]
    pub kind: ActivityType,
    /// If type is 1, the stream URL.
    pub url: Option<String>,
    /// Timestamps for this activity.
    pub timestamps: Option<ActivityTimestamps>,
    /// The application ID for the game, if any.
    pub application_id: Option<String>,
    /// What the player is currently doing.
    pub details: Option<String>,
    /// The user's current party status.
    pub state: Option<String>,
    /// The player's current party.
    pub party: ActivityParty,
    /// The Rich Presence assets.
    pub assets: ActivityAssets,
    /// The Rich Presence secrets.
    pub secrets: ActivitySecrets,
    /// Whether or not the activity is in a current game session.
    pub instanced: bool,
    /// Activity flags.
    pub flags: i32
}

/// Represents an Activity's timestamps.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ActivityTimestamps {
    /// When the activity started, in milliseconds.
    pub start: i32,
    /// When the activity ends, in milliseconds.
    pub end: i32
}

/// Information about the player's current party.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ActivityParty {
    /// The ID for this party.
    pub id: Option<String>,
    /// The party's current and maximum size.
    pub size: Option<[i32; 2]>
}

/// Rich Presence image and text assets.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ActivityAssets {
    /// The ID of the large image of this activity.
    pub large_image: Option<String>,
    /// The large image hover text.
    pub large_text: Option<String>,
    /// The ID of the small image of this activity.
    pub small_image: Option<String>,
    /// The small image hover text.
    pub small_text: Option<String>
}

/// Rich Presence secrets, used for joining and spectating.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ActivitySecrets {
    /// The secret for joining a party.
    pub join: Option<String>,
    /// The secret for spectating a game.
    pub spectate: Option<String>,
    /// The secret for an instanced match.
    #[serde(rename = "match")]
    pub match_type: Option<String>
}

/// The current presence of a user.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Presence {
    /// Milliseconds that the client went idle.
    pub since: Option<i32>,
    /// The user's current activity if any.
    pub activity: Option<Activity>,
    /// The status of the user.
    pub status: Status
}

impl SendablePacket for Presence {
    fn to_json(self) -> Result<String, JsonError> {
        serde_json::to_string(&SendPacket {
            op: Opcodes::StatusUpdate,
            d: self
        })
    }
}

/// A list of possible activity types.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[repr(u8)]
pub enum ActivityType {
    Game,
    Streaming,
    Listening
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