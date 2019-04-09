use serde_json::value::RawValue;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::message::Webhook;
use crate::snowflake::Snowflake;
use crate::User;

/// Represents a guild's audit log on Discord.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildAuditLog {
    /// A list of webhooks that were found in this audit log.
    pub webhooks: Vec<Webhook>,
    /// A list of users that were found in this audit log.
    pub users: Vec<User>,
    /// A collection of guild audit log entries.
    #[serde(rename = "audit_log_entries")]
    pub entries: Vec<GuildAuditLogEntry>,
}

/// An entry contained in a guild's audit log, which holds details about the audit log action.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildAuditLogEntry {
    /// The snowflake ID of the affected entity.
    pub target_id: Option<String>,
    /// The changes that were made to the target ID.
    #[serde(default)]
    pub changes: Option<Vec<GuildAuditLogChange>>,
    /// The user who performaned the changes.
    pub user_id: Snowflake,
    /// The ID that this entry belongs to.
    pub id: Snowflake,
    /// The type of action which occured.
    pub action_type: GuildAuditLogEvent,
    /// Additional options for certain action types.
    pub options: Option<GuildAuditEntryInfo>,
    /// The reason for this change.
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildAuditLogChange {
    /// The new value of the key.
    /// Note: Due to the mixed nature of this field, a raw JSON string is returned, instead of an object. You are responsible for parsing the JSON to the appropriate struct.
    pub new_value: Box<RawValue>,
    /// The old value of this key.
    /// /// Note: Due to the mixed nature of this field, a raw JSON string is returned, instead of an object. You are responsible for parsing the JSON to the appropriate struct.
    pub old_value: Box<RawValue>,
    // The audit log change key.
    // pub key: GuildAuditLogChangeKey
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildAuditEntryInfo {
    /// The number of days after which inactive members were kicked from the guild.
    /// This action is sent with the MEMBER_PRUNE audit log event.
    #[serde(default)]
    pub delete_member_days: Option<String>,
    /// The number of members removed by the prune.
    /// This action is sent with the MEMBER_PRUNE audit log event.,
    #[serde(default)]
    pub members_removed: Option<String>,
    /// The channel ID in which the messages were deleted. Sent with the MESSAGE_DELETE event.
    #[serde(default)]
    pub channel_id: Option<Snowflake>,
    /// The number of messages which were deleted. Sent with the MESSAGE_DELETE event.
    #[serde(default)]
    pub count: Option<String>,
    /// The ID of the overwritten entity, found in a channel overwrite.
    #[serde(default)]
    pub id: Option<Snowflake>,
    /// The type of the overwritten entity, sent with the channel overwrite events.
    #[serde(default, rename = "type")]
    pub kind: Option<String>,
    /// The name of the role found in the channel overwrite.
    #[serde(default)]
    pub role_name: Option<String>,
}

// TODO: Figure out change key
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GuildAuditLogChangeKey {
    Name(String),
    IconHash(String),
    SplashHash(String),
}

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u16)]
pub enum GuildAuditLogEvent {
    GuildUpdate = 1,
    ChannelCreate = 10,
    ChannelUpdate,
    ChannelDelete,
    ChannelOverwriteCreate,
    ChannelOverwriteUpdate,
    ChannelOverwriteDelete,
    MemberKick = 20,
    MemberPrune,
    MemberBanAdd,
    MemberBanRemove,
    MemberUpdate,
    MemberRoleUpdate,
    RoleCreate = 30,
    RoleUpdate,
    RoleDelete,
    InviteCreate = 40,
    InviteUpdate,
    InviteDelete,
    WebhookCreate = 50,
    WebhookUpdate,
    WebhookDelete,
    EmojiCreate = 60,
    EmojiUpdate,
    EmojiDelete,
    MessageDelete = 72,
}

#[derive(Serialize, Debug, Default)]
pub struct GetAuditLogOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_type: Option<GuildAuditLogEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Snowflake>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
}

impl GetAuditLogOptions {
    /// Sets the user ID to filter audit log entries by.
    pub fn user_id(mut self, id: Snowflake) -> Self {
        self.user_id = Some(id);
        self
    }

    /// Sets the action type of the audit logs being returned.
    pub fn action_type(mut self, event: GuildAuditLogEvent) -> Self {
        self.action_type = Some(event);
        self
    }

    /// Sets the entry ID to filter log entries before.
    pub fn before(mut self, id: Snowflake) -> Self {
        self.before = Some(id);
        self
    }

    /// Sets the amount of audit log entries to be returned in the request.
    pub fn limit(mut self, num: i32) -> Self {
        self.limit = Some(num);
        self
    }
}