pub type Token = String;

#[derive(Debug, Clone)]
pub struct UserData {
    pub user_id: i32,
    pub token: Token,
}


