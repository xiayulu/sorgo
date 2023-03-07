use mongodb::{Client, Collection, Database};
use once_cell::sync::OnceCell;

static INSTANCE: OnceCell<Database> = OnceCell::new();

pub async fn init() {
    let uri = std::env::var("MONGOURI").unwrap();
    let client = Client::with_uri_str(uri)
        .await
        .expect("error connecting to database");
    let db = client.database("projectMngt");

    INSTANCE.set(db).unwrap();
}

pub fn get_col<T>(collection_name: &str) -> Collection<T> {
    let db = INSTANCE.get().expect("get db from pool");
    db.collection(collection_name)
}
