use futures::future::Future;
use reqwest::Method;

use spectacles_model::channel::{Channel, CreateChannelOptions};
use spectacles_model::guild::{AddMemberOptions, CreateRoleOptions, Guild, GuildBan, GuildEmbed, GuildIntegration, GuildMember, GuildPrune, ListMembersOptions, ModifyGuildEmbedOptions, ModifyGuildIntegrationOptions, ModifyGuildOptions, ModifyMemberOptions, ModifyRoleOptions, Role};
use spectacles_model::invite::Invite;
use spectacles_model::snowflake::Snowflake;
use spectacles_model::voice::VoiceRegion;

use crate::{Error, RestClient};
use crate::Endpoint;

/// A view for interfacing with a Discord guild.
pub struct GuildView {
    id: u64,
    client: RestClient,
}

impl GuildView {
    pub(crate) fn new(id: u64, client: RestClient) -> Self {
        Self {
            id,
            client,
        }
    }
    /// Modifies the settings of this guild.
    pub fn modify(&self, opts: ModifyGuildOptions) -> impl Future<Item=Guild, Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::PATCH,
                format!("/guilds/{}", self.id),
            ).json(opts)
        )
    }

    /// Deletes this guild from Discord.
    pub fn delete(&self) -> impl Future<Item=(), Error=Error> {
        self.client.request(Endpoint::new(
            Method::DELETE,
            format!("/guilds/{}", self.id),
        ))
    }

    /// Fetches all channels in the current guild.
    pub fn get_channels(&self) -> impl Future<Item=Vec<Channel>, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/guilds/{}/channels", self.id),
        ))
    }

    /// Creates a new channel in this guild.
    pub fn create_channel(&self, opts: CreateChannelOptions) -> impl Future<Item=Channel, Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::POST,
                format!("/guilds/{}/channels", self.id),
            ).json(opts)
        )
    }

    /// Modifies a set of channel positions in this guild.
    pub fn modify_channel_positions(&self) {}

    /// Gets a guild member of the specified user id.
    pub fn get_member(&self, id: &Snowflake) -> impl Future<Item=GuildMember, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/guilds/{}/members/{}", self.id, id.0),
        ))
    }

    /// Lists all guild members in the current guild.
    pub fn list_members(&self, opts: ListMembersOptions) -> impl Future<Item=Vec<GuildMember>, Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::GET,
                format!("/guilds/{}/members", self.id),
            ).query(opts)
        )
    }

    /// Adds a member to the guild, using an Oauth2 access token.
    pub fn add_member(&self, id: &Snowflake, opts: AddMemberOptions) -> impl Future<Item=GuildMember, Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::PUT,
                format!("/guilds/{}/members/{}", self.id, id.0),
            ).json(opts)
        )
    }

    /// Adds a role to the specified guild member.
    pub fn add_member_role(&self, member: &Snowflake, role: &Snowflake) -> impl Future<Item=(), Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::PUT,
                format!("/guilds/{}/members/{}/roles/{}", self.id, member.0, role.0),
            )
        )
    }

    /// Gets a list of bans in the guild.
    pub fn get_bans(&self) -> impl Future<Item=Vec<GuildBan>, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/guilds/{}/bans", self.id),
        ))
    }

    /// Gets a a single ban for the provided user in the guild.
    pub fn get_ban(&self, user: &Snowflake) -> impl Future<Item=GuildBan, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/guilds/{}/bans/{}", self.id, user.0),
        ))
    }

    /// Removes a ban for the provided user from the guild.
    pub fn remove_ban(&self, user: &Snowflake) -> impl Future<Item=(), Error=Error> {
        self.client.request(Endpoint::new(
            Method::DELETE,
            format!("/guilds/{}/bans/{}", self.id, user.0),
        ))
    }

    /// Gets a collection of roles from the guild.
    pub fn get_roles(&self) -> impl Future<Item=Vec<Role>, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/guilds/{}/roles", self.id),
        ))
    }

    /// Creates a role in the guild.
    pub fn create_role(&self, opts: CreateRoleOptions) -> impl Future<Item=Role, Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::POST,
                format!("/guilds/{}/roles", self.id),
            ).json(opts)
        )
    }

    /// Modifies the provided role in the guild.
    pub fn modify_role(&self, role: &Snowflake, opts: ModifyRoleOptions) -> impl Future<Item=Role, Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::PATCH,
                format!("/guilds/{}/roles/{}", self.id, role.0),
            ).json(opts)
        )
    }

    /// Gets the number of members who would be pruned in a prune operation.
    pub fn get_prune_count(&self, days: i32) -> impl Future<Item=GuildPrune, Error=Error> {
        let query = json!({
            "days": days
        });

        self.client.request(
            Endpoint::new(
                Method::GET,
                format!("/guilds/{}/prune", self.id),
            ).query(query)
        )
    }

    /// Prunes guild members, according to the provided options.
    pub fn prune_members(&self, days: i32, compute: bool) -> impl Future<Item=GuildPrune, Error=Error> {
        let body = json!({
            "days": days,
            "compute_prune_count": compute
        });

        self.client.request(
            Endpoint::new(
                Method::POST,
                format!("/guilds/{}/prune", self.id),
            ).json(body)
        )
    }

    /// Gets a list of voice regions for the guild.
    pub fn get_voice_regions(&self) -> impl Future<Item=Vec<VoiceRegion>, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/guilds/{}/regions", self.id),
        ))
    }

    /// Gets a list of guild invites.
    pub fn get_invites(&self) -> impl Future<Item=Vec<Invite>, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/guilds/{}/invites", self.id),
        ))
    }

    /// Gets a list of guild integrations.
    pub fn get_integrations(&self) -> impl Future<Item=Vec<GuildIntegration>, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/guilds/{}/integrations", self.id),
        ))
    }

    /// Attaches an integration from the current user to the guild.
    pub fn add_integration(&self, kind: &str, id: &Snowflake) -> impl Future<Item=(), Error=Error> {
        let body = json!({
            "type": kind,
            "id": id
        });

        self.client.request(
            Endpoint::new(
                Method::POST,
                format!("/guilds/{}/integrations", self.id),
            ).json(body)
        )
    }


    /// Modifies the behavior and settings of a guild integration.
    pub fn modify_integration(&self, id: &Snowflake, opts: ModifyGuildIntegrationOptions) -> impl Future<Item=(), Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::PATCH,
                format!("/guilds/{}/integrations/{}", self.id, id.0),
            ).json(opts)
        )
    }

    /// Deletes a guild integration by the provided ID.
    pub fn delete_integration(&self, id: &Snowflake) -> impl Future<Item=(), Error=Error> {
        self.client.request(Endpoint::new(
            Method::DELETE,
            format!("/guilds/{}/integrations/{}", self.id, id.0),
        ))
    }

    /// Syncs guild integration by the provided ID.
    pub fn sync_integration(&self, id: &Snowflake) -> impl Future<Item=(), Error=Error> {
        self.client.request(Endpoint::new(
            Method::POST,
            format!("/guilds/{}/integrations/{}", self.id, id.0),
        ))
    }

    /// Gets the embed object of this guild.
    pub fn get_embed(&self) -> impl Future<Item=GuildEmbed, Error=Error> {
        self.client.request(Endpoint::new(
            Method::GET,
            format!("/guilds/{}/embed", self.id),
        ))
    }

    /// Modifies the current guild embed.
    pub fn modify_embed(&self, opts: ModifyGuildEmbedOptions) -> impl Future<Item=GuildEmbed, Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::PATCH,
                format!("/guilds/{}/embed", self.id),
            ).json(opts)
        )
    }

    /// Modifies a set of role positions in the guild.
    pub fn modify_role_positions(&self) {}

    /// Removes a role from the specified guild member.
    pub fn remove_member_role(&self, member: &Snowflake, role: &Snowflake) -> impl Future<Item=(), Error=Error> {
        self.client.request(Endpoint::new(
            Method::DELETE,
            format!("/guilds/{}/members/{}/roles/{}", self.id, member.0, role.0),
        ))
    }

    /// Modifies a guild member in this guild.
    pub fn modify_member(&self, id: &Snowflake, opts: ModifyMemberOptions) -> impl Future<Item=(), Error=Error> {
        self.client.request(
            Endpoint::new(
                Method::PATCH,
                format!("/guilds/{}/members/{}", self.id, id.0),
            ).json(opts)
        )
    }

    /// Removes a guild member from the guild.
    pub fn remove_member(&self, member: &Snowflake) -> impl Future<Item=(), Error=Error> {
        self.client.request(Endpoint::new(
            Method::DELETE,
            format!("/guilds/{}/members/{}", self.id, member.0),
        ))
    }

    /// Sets the nickname of the current client user.
    pub fn set_current_user_nick(&self, nick: &str) -> impl Future<Item=String, Error=Error> {
        let json = json!({
            "nick": nick
        });

        self.client.request(
            Endpoint::new(
                Method::PATCH,
                format!("/guilds/{}/members/@me/nick", self.id),
            ).json(json)
        )
    }
}