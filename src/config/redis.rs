use crate::error::{Error, Result};
use deadpool_redis::{
    redis::{cmd, FromRedisValue},
    Config, Connection, Pool,
};
use log::error;
use once_cell::sync::OnceCell;
use redis::ToRedisArgs;

static INSTANCE: OnceCell<Pool> = OnceCell::new();

pub fn init() {
    let cfg = Config::from_url(std::env::var("REDIS_URL").unwrap());
    let pool = cfg.create_pool().expect("create redis pool");

    if !INSTANCE.set(pool).is_ok() {
        panic!("init redis pool failed");
    }
}

pub async fn get_redis() -> Result<Connection> {
    let pool = INSTANCE.get();

    if let Some(pool) = pool {
        Ok(pool.get().await.map_err(|e| {
            let e = format!("Redis pool error:{}", e);
            error!("{}", e);
            Error::Service(e)
        })?)
    } else {
        Err(Error::Service("Redis pool not init".to_owned()))
    }
}

pub async fn set_ex<K, V>(k: K, v: V, seconds: usize) -> Result<()>
where
    K: ToRedisArgs,
    V: ToRedisArgs,
{
    let mut conn = get_redis().await?;
    cmd("SETEX")
        .arg(k)
        .arg(seconds)
        .arg(v)
        .query_async::<_, ()>(&mut conn)
        .await?;

    Ok(())
}

pub async fn get<K, T>(k: K) -> Result<T>
where
    K: ToRedisArgs,
    T: FromRedisValue,
{
    let mut conn = get_redis().await?;

    Ok(cmd("GET").arg(k).query_async(&mut conn).await?)
}
