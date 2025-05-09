use spacetimedb::{reducer, sys::Result, table, Identity, ReducerContext, Table, Timestamp};

#[table(name = user, public)]
pub struct User {
    #[primary_key]
    identity: Identity,
    name: Option<String>,
    online: bool,
}

#[table(name = message, public)]
pub struct Message {
    sender: Identity,
    sent: Timestamp,
    text: String,
}

#[reducer]
pub fn set_name(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let name = validate_name(name)?;

    let Some(user) = ctx.db.user().identity().find(ctx.sender) else {
        return Err("Cannot set name for unkown user".to_string());
    };

    ctx.db.user().identity().update(User {
        name: Some(name),
        ..user
    });
    Ok(())
}

fn validate_name(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Names must not be empyt".to_string())
    } else {
        Ok(name)
    }
}

#[reducer]
pub fn send_message(ctx: &ReducerContext, text: String) -> Result<(), String> {
    let text = validate_message(text)?;
    log::info!("{}", text);
    ctx.db.message().insert(Message {
        sender: ctx.sender,
        text,
        sent: ctx.timestamp,
    });
    Ok(())
}

fn validate_message(text: String) -> Result<String, String> {
    if text.is_empty() {
        Err("Messages must not be empty".to_string())
    } else {
        Ok(text)
    }
}

#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    if let Some(user) = ctx.db.user().identity().find(ctx.sender) {
        ctx.db.user().identity().update(User {
            online: true,
            ..user
        });
    } else {
        ctx.db.user().insert(User {
            name: None,
            identity: ctx.sender,
            online: true,
        });
    };
}

#[reducer(client_disconnected)]
pub fn identity_disconnecte4d(ctx: &ReducerContext) {
    let Some(user) = ctx.db.user().identity().find(ctx.sender) else {
        log::warn!(
            "Disconnect event for unkown user with identity {:?}",
            ctx.sender
        );
        return;
    };

    ctx.db.user().identity().update(User {
        online: false,
        ..user
    });
}
