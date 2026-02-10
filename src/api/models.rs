// Data models for API requests and responses
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    pub uuid: String,
    pub data: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignInRequest {
    pub uuid: String,
    pub data: String,
}

#[derive(Deserialize, Debug)]
pub struct SignInResponse {
    pub token: String,
}

#[derive(Serialize, Debug)]
pub struct TaskRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "dataStartDate")]
    pub data_start_date: String,
    #[serde(rename = "dataEndDate")]
    pub data_end_date: String,
    #[serde(rename = "format")]
    pub format: String,
    #[serde(rename = "periodicity")]
    pub periodicity: String,
    #[serde(rename = "params")]
    pub params: String,
    #[serde(rename = "productGroupCode")]
    pub product_group_code: i32,
}

#[derive(Deserialize, Debug)]
pub struct TaskResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "createDate")]
    pub create_date: String,
    #[serde(rename = "currentStatus")]
    pub current_status: String,
    #[serde(rename = "dataStartDate")]
    pub data_start_date: String,
    #[serde(rename = "dataEndDate")]
    pub data_end_date: String,
    #[serde(rename = "orgInn")]
    pub org_inn: String,
    #[serde(rename = "periodicity")]
    pub periodicity: String,
    #[serde(rename = "productGroupCode")]
    pub product_group_code: i32,
    #[serde(rename = "timeoutSecs")]
    pub timeout_secs: i32,
}

#[derive(Deserialize, Debug)]
pub struct TaskStatusResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "createDate")]
    pub create_date: String,
    #[serde(rename = "currentStatus")]
    pub current_status: String,
    #[serde(rename = "orgInn")]
    pub org_inn: String,
    #[serde(rename = "productGroupCode")]
    pub product_group_code: i32,
    #[serde(rename = "downloadingStorageDays")]
    pub downloading_storage_days: i32,
    #[serde(rename = "productGroups")]
    pub product_groups: Vec<ProductGroup>,
    #[serde(rename = "timeoutSecs")]
    pub timeout_secs: i32,
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProductGroup {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
}