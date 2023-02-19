use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::{openapi::Schema, ToResponse, ToSchema};
pub use uuid;
use uuid::Uuid;

#[derive(Serialize, Deserialize, thiserror::Error, Debug, PartialEq, Eq, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[error("Api returned an error: {description}")]
pub struct RspErr {
    pub code: usize,
    pub description: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "lowercase")]
pub enum RspData<T> {
    Ok(T),
    Err(RspErr),
}

impl<T> From<RspData<T>> for Result<T, RspErr> {
    fn from(v: RspData<T>) -> Self {
        match v {
            RspData::Ok(v) => Ok(v),
            RspData::Err(v) => Err(v),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(transparent)]
pub struct SecretString(pub String);

impl Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*********")
    }
}

#[cfg(feature = "openapi")]
impl<'s> ToSchema<'s> for SecretString {
    fn schema() -> (&'s str, utoipa::openapi::RefOr<Schema>) {
        (
            "SecretString",
            utoipa::openapi::ObjectBuilder::new()
                .schema_type(utoipa::openapi::SchemaType::String)
                .build()
                .into(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(ToResponse))]
pub struct Empty {}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct LoginRequest {
    pub password: SecretString,
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToResponse))]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct CreateListRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToResponse))]
pub struct CreateListResponse {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum ListStatus {
    Owned,
    SharedWrite,
    SharedRead,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Hash, Copy)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct ListInfo {
    pub id: Uuid,
    pub status: ListStatus,
    pub public: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(ToResponse))]
pub struct GetListsResponse {
    pub results: HashMap<String, ListInfo>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(ToResponse))]
pub struct SearchAccountResponse {
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub amount: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToResponse))]
pub struct ReadListResponse {
    pub items: Vec<Item>,
    pub readonly: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct AddToListRequest {
    pub name: String,
    pub amount: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(ToResponse))]
pub struct AddToListResponse {
    pub id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct ShareListRequest {
    pub share_with: Uuid,
    pub readonly: bool,
}

pub type ShareListResponse = Empty;

pub type DeleteItemResponse = Empty;

pub type DeleteShareResponse = Empty;

pub type DeleteListResponse = Empty;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

pub type RegisterResponse = Empty;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub amount: Option<String>,
}

pub type UpdateItemResponse = Empty;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToResponse))]
pub struct RecoveryInfoResponse {
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct RecoverPasswordRequest {
    pub password: String,
}

pub type RecoverPasswordResponse = Empty;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(ToResponse))]
pub struct GetSharesResponse {
    pub shared_with: HashMap<Uuid, bool>,
    pub public_link: Option<String>,
}

pub type UnshareResponse = Empty;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToResponse))]
pub struct GetAccountNameResponse {
    pub username: String,
}

pub type SetPublicResponse = crate::Empty;

pub type RemovePublicResponse = crate::Empty;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToResponse))]
pub struct GetHistoryResponse {
    pub matches: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct PantryItem {
    pub name: String,
    pub id: i32,
    pub amount: i32,
    pub target: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToResponse))]
pub struct GetPantryResponse {
    pub items: Vec<PantryItem>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct AddToPantryRequest {
    pub name: String,
    pub target: i32,
}

pub type AddToPantryResponse = crate::Empty;

pub type RefillPantryResponse = crate::Empty;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash, Clone)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct EditPantryItemRequest {
    pub target: Option<i32>,
    pub amount: Option<i32>,
}

pub type EditPantryItemResponse = crate::Empty;

pub type DeletePantryItemResponse = crate::Empty;
