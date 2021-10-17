use uuid::Uuid;
use chrono::{NaiveDateTime};
use serde::{Serialize, Deserialize};
use crate::schema::users;
use validator::Validate;
use regex::Regex;

lazy_static! {
 static ref USERNAME_REG_EX: Regex = Regex::new(r"^[a-zA-Z0-9]+([._]?[a-zA-Z0-9]+)*$").unwrap();
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable)]
pub struct User {
	pub id: Uuid,
	#[serde(rename = "profilePicture")]
	pub profile_picture: String,
	pub username: String,
	pub email: String,
	pub phone: String,
	#[serde(skip_serializing)]
	pub password: String,
	pub role: String,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "createdDate")]
	pub created_date: NaiveDateTime,
	#[serde(rename = "updatedDate")]
	pub updated_date: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate, Insertable)]
#[table_name = "users"]
pub struct NewUser {
	#[validate(length(min = 1, message = "Profile picture cannot be empty"))]
	#[serde(rename = "profilePicture")]
	pub profile_picture: String,
	#[validate(email(message = "fails validation - is not a valid email address"))]
	pub email: String,
	#[validate(
	length(min = 1, max = 20, message = "failed validation - must be 1-20 characters long"),
	regex(path = "USERNAME_REG_EX", message = "failed validation - value is not a valid username")
	)]
	pub username: String,
	#[validate(phone(message = "failed validation - phone number is not valid"))]
	pub phone: String,
	#[validate(length(
	min = 8,
	max = 72,
	message = "fails validation - must be 8-72 characters long"
	))]
	pub password: String,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
	#[serde(rename = "profilePicture")]
	pub profile_picture: Option<String>,
	pub username: Option<String>,
	pub phone: Option<String>,
	pub password: Option<String>,
}