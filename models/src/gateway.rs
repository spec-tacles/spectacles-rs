//! Structs representing the various elements of the Discord gateway.
use std::fmt::{Display, Formatter, Result as FmtResult};

use serde_eetf::Error as EtfError;
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_value::Value;

use crate::{
    guild::UnavailableGuild,
    presence::Activity,
    User,
};
use crate::presence::Presence;

/// Event Payload trait which allows for simple serialization to binary.
pub trait EventPayload {
    fn to_bytes(self) -> Result<Vec<u8>, EtfError>;
}

#[derive(Serialize, Deserialize, Debug)]
/// A JSON packet that the client would send over the Discord Gateway.
pub struct SendPacket<T: EventPayload> {
    op: Opcodes,
    d: T
}

/// Returns useful information about the application from the gateway.
#[derive(Serialize, Deserialize, Debug)]
pub struct GatewayBot {
    /// The websocket URL that can be used to begin connecting to this gateway.
    pub url: String,
    /// The recommended number of shards to spawn when connecting.
    pub shards: u64,
    /// Information regarding the current session start limit.
    pub session_start_limit: SessionStartLimit
}
/// Useful information about a bot's session start limit.
#[derive(Serialize, Deserialize, Debug)]
pub struct SessionStartLimit {
    /// The total number of session starts the current user is allowed.
    pub total: i32,
    /// The remaining number of session starts the current user is allowed.
    pub remaining: i32,
    /// The time until the limit resets.
    pub reset_after: i32,
}

/// Used for identifying a shard with the gateway.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdentifyPacket {
    /// The token this shard will use.
    pub token: String,
    /// The properties of the client.
    pub properties: IdentifyProperties,
    /// The version of the gateway to use.
    #[serde(rename = "v")]
    pub version: u8,
    /// Whether or not to compress packets.
    pub compress: bool,
    /// The total number of members where the gateway will stop sending offline members in the guild member list.
    pub large_threshold: i32,
    /// Holds the sharding information for this shard.
    pub shard: [u64; 2],
    /// The initial presence of this shard.
    pub presence: Option<Presence>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdentifyProperties {
    /// The client's operating system.
    #[serde(rename = "$os")]
    pub os: String,
    /// The current name of the library.
    #[serde(rename = "$browser")]
    pub browser: String,
    /// The current name of the library.
    #[serde(rename = "$device")]
    pub device: String
}

/// A packet used to resume a gateway connection.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResumeSessionPacket {
    /// The client's session ID>
    pub session_id: String,
    /// The current sequence.
    pub seq: u64,
    /// The token of the client.
    pub token: String
}
/// A JSON packet used to send a heartbeat to the gateway.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeartbeatPacket {
    /// The shard's last sequence number.
    pub seq: u64
}
/// A JSON packet that the client would receive over the Discord gateway.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReceivePacket {
    /// The opcode for this payload.
    pub op: Opcodes,
    /// The JSON value for this payload.
    pub d: Value,
    pub s: Option<u64>,
    /// The name of the event that was fired, if applicable.
    pub t: Option<GatewayEvent>
}

/// A JSON packet which defines the heartbeat the client should adhere to.
#[derive(Serialize, Deserialize, Debug)]
pub struct HelloPacket {
    /// The current heartbeat interval that the shard should follow.
    pub heartbeat_interval: u64,
    /// A debug-list of guilds that the user is in.
    pub _trace: Vec<u64>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// A Request guild members packet.
pub struct RequestGuildMembers {
    /// The guild ID to get members for.
    guild_id: u64,
    /// A string that the username starts with. If omitted, returns all members.
    query: String,
    /// The maximum number of members to send. If omitted, requests all members.
    limit: i32
}

/// An Update Voice State packet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateVoiceState {
    /// The guild ID of the guild.
    guild_id: u64,
    /// The channel ID of the voice channel.
    channel_id: Option<u64>,
    /// Whether or not to mute the current user.
    self_mute: bool,
    /// Whether or not to deafen the current user.
    self_deaf: bool
}

/// A packet sent to indicate a status update.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateStatus {
    /// Milliseconds since the client went idle.
    since: Option<i32>,
    /// The activity object to set.
    game: Option<Activity>,
    /// The status object to set.
    status: String,
    /// Whether or not the client is AFK.
    afk: bool
}

/// The packet received when a client completes a handshake with the Discord gateway.
/// This packet is considered the largest and most complex packet sent.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReadyPacket {
    /// The current gateway version,
    pub v: i32,
    /// Information about the current user.
    pub user: User,
    /// An empty array of private channels.
    pub private_channels: [u64; 0],
    /// The guilds that the user is currently in.
    /// This will be an array of UnavailableGuild objects.
    pub guilds: Vec<UnavailableGuild>,
    /// The session ID that is used to resume a gateway connection.
    pub session_id: String,
    /// The guilds that a user is in, used for debugging.
    pub _trace: Vec<u64>,
    /// Information about the current shard, if applicable.
    #[serde(default)]
    pub shard: [u64; 2]
}

/// This packet is received when the client resumes an existing session.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResumedPacket {
    /// The guilds that a user is in, used for debugging.
    pub _trace: Vec<u64>
}

impl EventPayload for ResumedPacket {
    fn to_bytes(self) -> Result<Vec<u8>, EtfError> {
        serde_eetf::to_bytes(&SendPacket {
            op: Opcodes::Resume,
            d: self
        })
    }
}

impl EventPayload for HeartbeatPacket {
    fn to_bytes(self) -> Result<Vec<u8>, EtfError> {
        serde_eetf::to_bytes(&SendPacket {
            op: Opcodes::Heartbeat,
            d: self
        })
    }
}

impl EventPayload for HelloPacket {
    fn to_bytes(self) -> Result<Vec<u8>, EtfError>  {
        serde_eetf::to_bytes(&SendPacket {
            op: Opcodes::Hello,
            d: self
        })
    }
}

impl EventPayload for ResumeSessionPacket {
    fn to_bytes(self) -> Result<Vec<u8>, EtfError>  {
        serde_eetf::to_bytes(&SendPacket {
            op: Opcodes::Resume,
            d: self
        })
    }
}


impl EventPayload for ReadyPacket {
    fn to_bytes(self) -> Result<Vec<u8>, EtfError>  {
        serde_eetf::to_bytes(&SendPacket {
            op: Opcodes::Dispatch,
            d: self
        })
    }
}

impl EventPayload for IdentifyPacket {
    fn to_bytes(self) -> Result<Vec<u8>, EtfError>  {
        serde_eetf::to_bytes(&SendPacket {
            op: Opcodes::Identify,
            d: self
        })
    }
}

impl EventPayload for UpdateStatus {
    fn to_bytes(self) -> Result<Vec<u8>, EtfError>  {
        serde_eetf::to_bytes(&SendPacket {
            op: Opcodes::StatusUpdate,
            d: self
        })
    }
}

impl EventPayload for UpdateVoiceState {
    fn to_bytes(self) -> Result<Vec<u8>, EtfError>  {
        serde_eetf::to_bytes(&SendPacket {
            op: Opcodes::VoiceStatusUpdate,
            d: self
        })
    }
}

impl EventPayload for RequestGuildMembers {
    fn to_bytes(self) -> Result<Vec<u8>, EtfError> {
        serde_eetf::to_bytes(&SendPacket {
            op: Opcodes::RequestGuildMembers,
            d: self
        })
    }
}


/// A List of possible status types.
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum StatusType {
    Online,
    DnD,
    Idle,
    Invisible,
    Offline
}

impl StatusType {
    fn as_str(&self) -> &str {
        match *self {
            StatusType::Online => "online",
            StatusType::DnD => "dnd",
            StatusType::Idle => "idle",
            StatusType::Invisible => "invisible",
            StatusType::Offline => "offline"
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_camel_case_types)]
/// An organized list of Discord gateway events.
pub enum GatewayEvent {
    HELLO,
    READY,
    RESUMED,
    INVALID_SESSION,
    CHANNEL_CREATE,
    CHANNEL_UPDATE,
    CHANNEL_DELETE,
    CHANNEL_PINS_UPDATE,
    GUILD_CREATE,
    GUILD_UPDATE,
    GUILD_DELETE,
    GUILD_BAN_ADD,
    GUILD_BAN_REMOVE,
    GUILD_EMOJIS_UPDATE,
    GUILD_INTEGRATIONS_UPDATE,
    GUILD_MEMBER_ADD,
    GUILD_MEMBER_REMOVE,
    GUILD_MEMBER_UPDATE,
    GUILD_MEMBERS_CHUNK,
    GUILD_ROLE_CREATE,
    GUILD_ROLE_UPDATE,
    GUILD_ROLE_DELETE,
    MESSAGE_CREATE,
    MESSAGE_UPDATE,
    MESSAGE_DELETE,
    MESSAGE_DELETE_BULK,
    MESSAGE_REACTION_ADD,
    MESSAGE_REACTION_REMOVE,
    MESSAGE_REACTION_REMOVE_ALL,
    PRESENCE_UPDATE,
    TYPING_START,
    USER_UPDATE,
    VOICE_STATE_UPDATE,
    VOICE_SERVER_UPDATE,
    WEBHOOKS_UPDATE
}

impl Display for GatewayEvent {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}
/// A set of possible Discord gateway opcodes.
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum Opcodes {
    /// Dispatches a gateway event.
    Dispatch,
    /// Used for sending ping and heartbeats.
    Heartbeat,
    /// Used for obtaining a client handshake.
    Identify,
    /// Used to update the shard's status.
    StatusUpdate,
    /// Used to join and leave voice channels.
    VoiceStatusUpdate,
    /// Used to resume a closed connection.
    Resume = 6,
    /// Tells clients to reconnect to the gateway.
    Reconnect,
    /// used to request guild members.
    RequestGuildMembers,
    /// Used to notify the client of an invlaid session.
    InvalidSession,
    /// Sent immediately after connecting, contains heartbeat information.
    Hello,
    /// Sent immediately after receiving a heartbeat.
    HeartbeatAck
}

/// Codes that denote the cause of the gateway closing.
#[derive(Debug, Copy, Deserialize_repr, Clone)]
#[repr(u16)]
pub enum CloseCodes {
    /// The cause of the error is unknown.
    UnknownError = 4000,
    /// The opcode or the payload for an opcode sent was invalid.
    UnknownOpcode,
    /// An invalid payload was sent.
    DecodeError,
    /// A payload was sent prior to identifying.
    NotAuthenticated,
    /// The token sent with the payload was invalid.
    AuthenticationFailed,
    /// More than one identify payload was sent.
    AlreadyAuthenticated,
    /// The sequence sent when resuming the session was invalid.
    InvalidSeq,
    /// A ratelimit caused by sending payloads too quickly.
    Ratelimited,
    /// The session timed out, a reconnect is required.
    SessionTimeout,
    /// An invalid shard was sent when identifying.
    InvalidShard,
    /// The session would have had too many guilds, which indicated that sharding is required.
    ShardingRequired,
}