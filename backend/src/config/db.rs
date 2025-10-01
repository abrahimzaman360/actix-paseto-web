use std::sync::LazyLock;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

// Single Database Instance:
pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);
