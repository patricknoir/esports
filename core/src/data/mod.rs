use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;
use actix::{Actor, SyncContext};

use crate::prelude::*;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct DataService(pub PgPool);

impl Actor for DataService {
	type Context = SyncContext<Self>;
}

impl DataService {
	pub fn new(db_url: String) -> Self {
		DataService(DataService::new_pool(db_url).expect("Error creating DB Pool"))
	}

	pub fn new_pool(db_url: String) -> Result<PgPool> {
		let manager = ConnectionManager::<PgConnection>::new(db_url);
		let pool = Pool::builder().build(manager)?;
		Ok(pool)
	}
}