use crate::guild::UnavailableGuild;
use crate::presence::Activity;
use crate::User;

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
    /// The op code for this payload.
    pub op: Opcodes,
    /// The JSON value for this payload.
    pub d: serde_json::Value,
    pub s: u64,
    pub t: GatewayEvent,
}

#[derive(Serialize, Deserialize, Debug)]
/// A JSON packet that the client would send over the Discord Gateway.
pub struct SendPacket {
    op: Opcodes,
    d: serde_json::Value
}

/// A JSON packet which defines the heartbeat the client should adhere to.
#[derive(Serialize, Deserialize, Debug)]
pub struct HelloPacket {
    pub heartbeat_interval: u64,
    pub _trace: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub _trace: Vec<String>,
    /// Information about the current shard, if applicable.
    #[serde(default)]
    pub shard: [u64; 2]
}

/// This packet is received when the client resumes an existing session.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResumedPacket {
    /// The guilds that a user is in, used for debugging.
    pub _trace: Vec<String>

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
            GatewayEvent::Hello => "HELLO",
            GatewayEvent::Ready => "READY",
            GatewayEvent::Resumed => "RESUMED",
            GatewayEvent::InvalidSession => "INVALID_SESSION",
            GatewayEvent::ChannelCreate => "CHANNEL_CREATE",
            GatewayEvent::ChannelUpdate => "CHANNEL_UPDATE",
            GatewayEvent::ChannelDelete => "CHANNEL_DELETE",
            GatewayEvent::ChannelPinsUpdate => "CHANNEL_PINS_UPDATE",
            GatewayEvent::GuildCreate => "GUILD_CREATE",
            GatewayEvent::GuildUpdate => "GUILD_UPDATE",
            GatewayEvent::GuildDelete => "GUILD_DELETE",
            GatewayEvent::GuildBanAdd => "GUILD_BAN_ADD",
            GatewayEvent::GuildBanRemove => "GUILD_BAN_REMOVE",
            GatewayEvent::GuildEmojisUpdate => "GUILD_EMOJIS_UPDATE",
            GatewayEvent::GuildIntegrationsUpdate => "GUILD_INTEGRATIONS_UPDATE",
            GatewayEvent::GuildMemberAdd => "GUILD_MEMBER_ADD",
            GatewayEvent::GuildMemberUpdate => "GUILD_MEMBER_UPDATE",
            GatewayEvent::GuildMemberRemove => "GUILD_MEMBER_REMOVE",
            GatewayEvent::GuildMembersChunk => "GUILD_MEMBERS_CHUNK",
            GatewayEvent::GuildRoleCreate => "GUILD_ROLE_CREATE",
            GatewayEvent::GuildRoleUpdate => "GUILD_ROLE_UPDATE",
            GatewayEvent::GuildRoleDelete => "GUILD_ROLE_DELETE",
            GatewayEvent::MessageCreate => "MESSAGE_CREATE",
            GatewayEvent::MessageUpdate => "MESSAGE_UPDATE",
            GatewayEvent::MessageDelete => "MESSAGE_DELETE",
            GatewayEvent::MessageDeleteBulk => "MESSAGE_DELETE_BULK",
            GatewayEvent::MessageReactionAdd => "MESSAGE_REACTION_ADD",
            GatewayEvent::MessageReactionRemove => "MESSAGE_REACTION_REMOVE",
            GatewayEvent::MessageReactionRemoveAll => "MESSAGE_REACTION_REMOVE_ALL",
            GatewayEvent::PresenceUpdate => "PRESENCE_UPDATE",
            GatewayEvent::TypingStart => "TYPING_START",
            GatewayEvent::UserUpdate => "USER_UPDATE",
            GatewayEvent::VoiceStateUpdate => "VOICE_STATE_UPDATE",
            GatewayEvent::VoiceServerUpdate => "VOICE_SERVER_UPDATE",
            GatewayEvent::WebhooksUpdate => "WEBHOOKS_UPDATE"
        }
    }
}
/// A set of possible Discord gateway opcodes.
#[derive(Deserialize, Serialize, Debug, Clone)]
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