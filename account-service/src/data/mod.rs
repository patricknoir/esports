use crate::model::user::{NewUser, User};
use core::actix::{Message, Handler};
use core::prelude::*;
use core::data::DataService;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use crate::model::credentials::Credentials;
use core::error::AppError;
use serde_json::json;
use crate::api::{GetById, UpdateUser, GetByEmail};

impl Message for NewUser {
	type Result = Result<User>;
}

impl Handler<NewUser> for DataService {
	type Result = Result<User>;

	fn handle(&mut self, msg: NewUser, _ctx: &mut Self::Context) -> Self::Result {
		use crate::schema::users::dsl::*;

		let conn = self.0.get()?;
		diesel::insert_into(users)
			.values(msg)
			.get_result::<User>(&conn)
			.map_err(|e| e.into())
	}
}

impl Message for Credentials {
	type Result = Result<User>;
}

impl Handler<Credentials> for DataService {
	type Result = Result<User>;

	fn handle(&mut self, msg: Credentials, _ctx: &mut Self::Context) -> Self::Result {
		use crate::schema::users::dsl::*;
		let conn = self.0.get()?;
		let stored_user: User = users.filter(email.eq(msg.email)).first(&conn)?;
		if  msg.password.eq(stored_user.password.as_str()){
			Ok(stored_user)
		} else {
			Err(AppError::Unauthorized(json!({
				"error": "Wrong credentials"
			})))
		}
	}
}

impl Handler<GetById> for DataService {
	type Result = Result<User>;

	fn handle(&mut self, msg: GetById, _ctx: &mut Self::Context) -> Self::Result {
		use crate::schema::users::dsl::*;
		let conn = self.0.get()?;
		users.filter(id.eq(msg.0)).first(&conn).map_err(|e| e.into())
	}
}

impl Handler<GetByEmail> for DataService {
	type Result = Result<User>;

	fn handle(&mut self, msg: GetByEmail, _ctx: &mut Self::Context) -> Self::Result {
		use crate::schema::users::dsl::*;
		let conn = self.0.get()?;
		let stored_user: User = users.filter(email.eq(msg.0)).first(&conn)?;
		Ok(stored_user)
	}
}

impl Handler<UpdateUser> for DataService {
	type Result = Result<User>;

	fn handle(&mut self, msg: UpdateUser, _ctx: &mut Self::Context) -> Self::Result {
		use crate::schema::users::dsl::*;

		match diesel::update(users.find(msg.id))
			.set(&msg.changes)
			.get_result::<User>(&self.0.get().unwrap())
		{
			Ok(user) => Ok(user),
			Err(e) => Err(e.into()),
		}
	}
}