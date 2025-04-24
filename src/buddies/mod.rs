pub mod funfriend;

pub enum DialogType {
    Chatter,
    Moved,
    Touched
}

pub trait Buddy<'a> {
    fn name() -> &'a str;
    fn dialog(dialog_type: DialogType) -> Vec<Vec<&'a str>>;
    fn textures(); //todo 
    fn talk_sound(); //todo
    fn font(); //todo
}