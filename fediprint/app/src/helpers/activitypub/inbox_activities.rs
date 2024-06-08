use shared::models::inbox::InboxEvent;
use tracing::debug;

pub async fn handle_create(event: InboxEvent) -> anyhow::Result<()> {
    debug!("Create: {:?}", event);
    Ok(())
}

pub async fn handle_update(event: InboxEvent) -> anyhow::Result<()> {
    debug!("Update: {:?}", event);
    Ok(())
}
pub async fn handle_delete(event: InboxEvent) -> anyhow::Result<()> {
    debug!("Delete: {:?}", event);
    Ok(())
}

pub async fn handle_follow(event: InboxEvent) -> anyhow::Result<()> {
    debug!("Follow: {:?}", event);
    Ok(())
}
pub async fn handle_accept(event: InboxEvent) -> anyhow::Result<()> {
    debug!("Accept: {:?}", event);
    Ok(())
}

pub async fn handle_reject(event: InboxEvent) -> anyhow::Result<()> {
    debug!("Reject: {:?}", event);
    Ok(())
}

pub async fn handle_add(event: InboxEvent) -> anyhow::Result<()> {
    debug!("Add: {:?}", event);
    Ok(())
}

pub async fn handle_remove(event: InboxEvent) -> anyhow::Result<()> {
    debug!("Remove: {:?}", event);
    Ok(())
}

pub async fn handle_like(event: InboxEvent) -> anyhow::Result<()> {
    debug!("Like: {:?}", event);
    Ok(())
}

pub async fn handle_announce(event: InboxEvent) -> anyhow::Result<()> {
    debug!("Announce: {:?}", event);
    Ok(())
}

pub async fn handle_undo(event: InboxEvent) -> anyhow::Result<()> {
    debug!("Undo: {:?}", event);
    Ok(())
}
