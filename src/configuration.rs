#[derive(serde::Deserialize)]

pub struct Settings{
    pub database: DatabaseSetting,
    pub application_port:u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSetting{
    pub username:String,
    pub password:String,
    pub port:u16,
    pub host:String,
    pub database_name:String,
}