use serde::Deserialize;
use uuid::Uuid;


#[derive(Deserialize)]
pub struct IdParams {
    pub id: Option<Uuid>
}