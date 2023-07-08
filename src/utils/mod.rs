use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;


#[derive(Deserialize)]
pub struct IdPathParams {
    pub id: Option<Uuid>
}

#[derive(Deserialize, Validate)]
pub struct PaginationQueryParams {
    #[validate(range(max = 100, message="Limit must be less than or equal 100"))]
    pub limit: i64,
    pub offset: i64,
}