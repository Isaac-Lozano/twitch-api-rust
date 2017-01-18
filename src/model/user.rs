#[derive(Clone,Deserialize,Debug)]
pub struct User
{
    #[serde(rename = "type")]
    pub user_type: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub logo: String,
    #[serde(rename = "_id")]
    pub id: UserId,
    pub display_name: String,
    pub bio: String,
}

#[derive(Clone,Deserialize,Debug)]
pub struct Users
{
    #[serde(rename = "users")]
    pub users: Vec<User>,
}

#[derive(Clone,Deserialize,Debug)]
pub struct UserId(pub u64);
