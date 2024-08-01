use super::*;

/// Fetches a user by ID
///
/// # Arguments
/// `user_id` - The ID of the user to fetch
pub async fn fetch_user(user_id: &str) -> Result<User, Box<dyn std::error::Error>> {
    let url = format!("users/{}", user_id);
    let resp = request(Method::GET, &url, None).await;
    let mut user = User::deserialize_json(&resp.text().await?)?;
    user.mention = format!("<@{}>", user.id);
    Ok(user)
}

/// Fetches a member by ID
///
/// # Arguments
/// `guild_id` - The ID of the guild the member is in
/// `user_id` - The ID of the member to fetch
pub async fn fetch_member(
    guild_id: &str,
    user_id: &str,
) -> Result<Member, Box<dyn std::error::Error>> {
    let url = format!("guilds/{guild_id}/members/{user_id}");
    let resp = request(Method::GET, &url, None).await;
    let mut member = Member::deserialize_json(&resp.text().await?)?;
    member.mention = format!("<@{}>", user_id);
    Ok(member)
}
