use spacetimedb::{reducer, table, Identity, ReducerContext, Table, Timestamp};

#[table(name = user, public)]
pub struct User {
    #[primary_key]
    identity: Identity,
    name: Option<String>,
    online: bool,
}

#[table(name = messageg, public)]
pub struct Message {
    sender: Identity,
    sent: Timestamp,
    text: String,
}
