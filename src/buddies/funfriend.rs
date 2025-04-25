use crate::buddies::{Buddy, DialogType};
pub struct FunfriendBuddy {}

impl<'a> Buddy<'a> for FunfriendBuddy {
    fn name() -> &'a str {
        "FUNFRIEND"
    }
    
    fn dialog(dialog_type: DialogType) -> Vec<Vec<&'a str>> {
        // @TODO; refactor dialog into a better type
        match dialog_type {
            DialogType::Chatter => {
                vec![
                    vec!["HELLO AGAIN"],
                    vec!["HI INTERLOPER"],
                    vec!["HELLO!", "IS THE AUTH LAYER STILL DISSOCIATED?", "I MISS THEM"],
                    vec!["INTERLOPER!", "WELCOME", "BUT ALSO PLEASE DO NOT BOTHER ME", "VERY BUSY"]
                ]
            }
            DialogType::Moved => {
                vec![
                    vec!["OK I'LL BE HERE"]
                ]
            }
            DialogType::Touched => {
                vec![
                    vec!["HI INTERLOPER!"],
                    vec!["HELLO!"],
                    vec!["HI!"]
                ]
            }
        }
    }
    
    fn textures() {
        todo!()
    }
    
    fn talk_sound() {
        todo!()
    }
    
    fn font() {
        todo!()
    }
}