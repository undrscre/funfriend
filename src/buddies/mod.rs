pub mod funfriend;

use crate::config::Friends;
use funfriend::FunfriendBuddy;

pub enum DialogType {
    Chatter,
    Moved,
    Touched,
}

pub trait Buddy {
    fn name(&self) -> &'static str;
    fn dialog(&self, dialog_type: DialogType) -> Vec<Vec<&'static str>>;
    fn textures(&self); //todo 
    fn talk_sound(&self); //todo
    fn font(&self); //todo
}

// @TODO make this work
pub fn retrieve_buddy(buddy_type: &Friends) -> Box<dyn Buddy> {
    match buddy_type {
        Friends::FUNFRIEND => {
            Box::new(FunfriendBuddy {})
        }
    }
}
