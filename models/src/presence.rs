/// Represents a user's current activity on Discord.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Activity {
    /// The name of the activity.
    pub name: String,
    /// The type of activity.
    pub r#type: i32

}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Presence {

}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[repr(u8)]
pub enum ActivityType {
    Game,
    Streaming,
    Listening
}