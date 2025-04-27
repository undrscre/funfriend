use super::buddy::BuddyContext;

pub struct ChatterContext {
    pub parent: Option<BuddyContext>,
}