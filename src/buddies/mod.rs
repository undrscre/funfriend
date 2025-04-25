pub mod funfriend;
pub mod context;

use crate::config::Friends;
use funfriend::FunfriendBuddy;

pub enum DialogType {
    Chatter,
    Moved,
    Touched
}

pub trait Buddy<'a>: Sized {
    fn name() -> &'a str;
    fn dialog(dialog_type: DialogType) -> Vec<Vec<&'a str>>;
    fn textures(); //todo 
    fn talk_sound(); //todo
    fn font(); //todo
}

// @TODO make this work
pub fn retrieve_buddy(_buddy_type: Friends) -> Option<FunfriendBuddy> {
    Some(FunfriendBuddy {})
}