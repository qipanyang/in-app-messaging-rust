//! Database-related functions
use crate::config::{CONFIG};
use diesel::{mysql::MysqlConnection, r2d2::{ConnectionManager, PoolError}};
use r2d2::Pool;

pub type PoolType = Pool<ConnectionManager<MysqlConnection>>;

pub fn init_mysql_pool() -> Result<PoolType, PoolError>
{
    let config = CONFIG.clone();
    let manager = ConnectionManager::<MysqlConnection>::new(config.database_url);
    Pool::builder().build(manager)
}
