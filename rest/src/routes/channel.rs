/// Routes pertaining to channels in the Discord API.
pub struct ChannelsView(u64);

impl ChannelsView {
    pub fn base_channel(&self) -> String {
        format!("/channels/{}", self.0)
    }

    pub fn channel_messages(&self) -> String {
        format!("/channels/{}/messages", self.0)
    }

    pub fn get_channel_message(&self, mid: u64) -> String {
        format!("/channels/{}/messages/{}", self.0, mid)
    }
}