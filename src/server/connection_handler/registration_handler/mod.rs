use std::sync::{atomic::AtomicBool, Arc};

use crate::server::{connection::Connection, database::DatabaseHandle};

struct RegistrationHandler<C: Connection> {
    stream: C,
    database: DatabaseHandle<C>,
    servername: String,
    online: Arc<AtomicBool>,
}
