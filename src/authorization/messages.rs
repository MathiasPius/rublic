use chrono::Duration;
use super::models::*;

actor_command! (AuthorizeUser(friendly_name: String, password: String) -> Vec<Claim>);
actor_command! (AuthorizeToken(token: String) -> Vec<Claim>);
actor_command! (BuildTokenFromClaims(claims: Vec<Claim>, lifetime: Duration) -> String);