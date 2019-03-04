use crate::guild::UnavailableGuild;
use crate::presence::Activity;
use crate::User;

use super::{parse_snowflake, parse_snowflake_array};

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

/// A JSON packet that the client would receive over the Discord gateway.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReceivePacket {
    op: Opcodes,
    d: serde_json::Value,
    s: u64,
    t: GatewayEvent,
}

#[derive(Serialize, Deserialize, Debug)]
/// A JSON packet that the client would send over the Discord Gateway.
pub struct SendPacket {
    op: Opcodes,
    d: serde_json::Value
}

#[derive(Serialize, Deseralize, Debug, Clone)]
/// A Request guild members packet.
pub struct RequestGuildMembers {
    /// The guild ID to get members for.
    guild_id: String,
    /// A string that the username starts with. If omitted, returns all members.
    query: String,
    /// The maximum number of members to send. If omitted, requests all members.
    limit: i32
}

/// An Update Voice State packet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateVoiceState {
    guild_id: String,
    channel_id: Option<String>,
    self_mute: bool,
    self_deaf: bool
}

/// A packet sent to indicate a status update.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateStatus {
    since: Option<i32>,
    activity: Activity,
    status: String,
    afk: bool
}

/// A List of possible status types.
#[derive(Debug, Clone)]
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
    #[serde(deserialize_with = "parse_snowflake_array")]
    pub _trace: Vec<u64>,
    /// Information about the current shard, if applicable.
    #[serde(default)]
    pub shard: [u64; 2]
}

/// This packet is received when the client resumes an existing session.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResumedPacket {
    /// The guilds that a user is in, used for debugging.
    #[serde(deserialize_with = "parse_snowflake_array")]
    pub _trace: Vec<u64>

}
#[derive(Debug, Deserialize, Serialize, Clone)]
/// An organized list of Discord gateway events.
pub enum GatewayEvent {
    Hello,
    Ready,
    Resumed,
    InvalidSession,
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleCreate,
    GuildRoleUpdate,
    GuildRoleDelete,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    MessageDeleteBulk,
    MessageReactionAdd,
    MessageReactionRemove,
    MessageReactionRemoveAll,
    PresenceUpdate,
    TypingStart,
    UserUpdate,
    VoiceStateUpdate,
    VoiceServerUpdate,
    WebhooksUpdate
}

impl GatewayEvent {
    fn to_str(&self) -> &str {
        match *self {
            /// Defines the heartbeat interval.
            GatewayEvent::Hello => "HELLO",
            /// Contains the client's initial state.
            GatewayEvent::Ready => "READY",
            /// A response when a client resumes a session.
            GatewayEvent::Resumed => "RESUMED",
            /// A failure response to an Identify or Resume packet.
            GatewayEvent::InvalidSession => "INVALID_SESSION",
            /// Emitted when a new channel is created.
            GatewayEvent::ChannelCreate => "CHANNEL_CREATE",
            /// Emitted when a channel is updated.
            GatewayEvent::ChannelUpdate => "CHANNEL_UPDATE",
            /// Emitted when a channel is deleted.
            GatewayEvent::ChannelDelete => "CHANNEL_DELETE",
            /// Emitted when a channel's pins are updated.
            GatewayEvent::ChannelPinsUpdate => "CHANNEL_PINS_UPDATE",
            /// Emitted when a guild becomes available, a lazy loaded unavailable guild, or when the user joins a new guild.
            GatewayEvent::GuildCreate => "GUILD_CREATE",
            /// Emitted when a guild is updated.
            GatewayEvent::GuildUpdate => "GUILD_UPDATE",
            /// Emitted when a guild becomes available, or if the user leaves/is removed from the guild.
            GatewayEvent::GuildDelete => "GUILD_DELETE",
            /// Emitted when a user was banned from the guild.
            GatewayEvent::GuildBanAdd => "GUILD_BAN_ADD",
            /// Emitted when a user is unbanned from a guild
            GatewayEvent::GuildBanRemove => "GUILD_BAN_REMOVE",
            /// Emitted a guild's emojis are updated.
            GatewayEvent::GuildEmojisUpdate => "GUILD_EMOJIS_UPDATE",
            /// Emitted when a guild integration is updated.
            GatewayEvent::GuildIntegrationsUpdate => "GUILD_INTEGRATIONS_UPDATE",
            /// Emitted when a new user joins a guild.
            GatewayEvent::GuildMemberAdd => "GUILD_MEMBER_ADD",
            /// Emitted when a guild member has been updated.
            GatewayEvent::GuildMemberUpdate => "GUILD_MEMBER_UPDATE",
            /// Emitted when a user is removed from a guild.
            GatewayEvent::GuildMemberRemove => "GUILD_MEMBER_REMOVE",
            /// The response when requesting guild members.
            GatewayEvent::GuildMembersChunk => "GUILD_MEMBERS_CHUNK",
            /// Emitted when a guild role is created.
            GatewayEvent::GuildRoleCreate => "GUILD_ROLE_CREATE",
            /// Emitted when a guild role is updated.
            GatewayEvent::GuildRoleUpdate => "GUILD_ROLE_UPDATE",
            /// Emitted when a guild role is deleted.
            GatewayEvent::GuildRoleDelete => "GUILD_ROLE_DELETE",
            /// Emitted when a message was created.
            GatewayEvent::MessageCreate => "MESSAGE_CREATE",
            /// Emitted when a message is updated.
            GatewayEvent::MessageUpdate => "MESSAGE_UPDATE",
            /// Emitted when a message is deleted.
            GatewayEvent::MessageDelete => "MESSAGE_DELETE",
            /// Emitted when multiple messages were deleted at once.
            GatewayEvent::MessageDeleteBulk => "MESSAGE_DELETE_BULK",
            /// Emitted when a user reacts to a message.
            GatewayEvent::MessageReactionAdd => "MESSAGE_REACTION_ADD",
            /// Emitted when a user removes a reaction from a message.
            GatewayEvent::MessageReactionRemove => "MESSAGE_REACTION_REMOVE",
            /// Emitted when all reactions were removed from a message.
            GatewayEvent::MessageReactionRemoveAll => "MESSAGE_REACTION_REMOVE_ALL",
            /// Emitted when a user was updated.
            GatewayEvent::PresenceUpdate => "PRESENCE_UPDATE",
            /// Emitted when a user starts typing in a channel.
            GatewayEvent::TypingStart => "TYPING_START",
            /// Emitted when a user's properties change,
            GatewayEvent::UserUpdate => "USER_UPDATE",
            /// Emitted when someone joins, leaves, or moved a voice channel.
            GatewayEvent::VoiceStateUpdate => "VOICE_STATE_UPDATE",
            /// Emitted when a guild's voice server was updated.
            GatewayEvent::VoiceServerUpdate => "VOICE_SERVER_UPDATE",
            /// Emitted when a webhook is created, updated, or deleted in a guild.
            GatewayEvent::WebhooksUpdate => "WEBHOOKS_UPDATE"
        }
    }
}
/// A set of possible Discord gateway opcodes.
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
    Resume,
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
#[derive(Debug, Copy, Clone)]
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