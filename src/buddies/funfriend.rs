use crate::{buddies::{Buddy, DialogType}, renderer::{self, texture::TextureBasket}};
pub struct FunfriendBuddy {}

impl Buddy for FunfriendBuddy {
    fn name(&self) -> &'static str {
        "FUNFRIEND"
    }

    fn dialog(&self, dialog_type: DialogType) -> Vec<Vec<&'static str>> {
        // @TODO; refactor dialog into a better type
        match dialog_type {
            DialogType::Chatter => {
                vec![
                    vec!["HELLO AGAIN"],
                    vec!["HI INTERLOPER"],
                    vec![
                        "HELLO!",
                        "IS THE AUTH LAYER STILL DISSOCIATED?",
                        "I MISS THEM",
                    ],
                    vec![
                        "INTERLOPER!",
                        "WELCOME",
                        "BUT ALSO PLEASE DO NOT BOTHER ME",
                        "VERY BUSY",
                    ],
                ]
            }
            DialogType::Moved => {
                vec![vec!["OK I'LL BE HERE"]]
            }
            DialogType::Touched => {
                vec![vec!["HI INTERLOPER!"], vec!["HELLO!"], vec!["HI!"]]
            }
        }
    }

    fn textures(&self) -> TextureBasket {
        TextureBasket::new(
            (0..09).map(|i| renderer::texture::load_texture(&format!("assets/buddies/funfriend_{:02}.png", i))).collect(),
            10.,
        )
    }

    fn talk_sound(&self) {}

    fn font(&self) {
        todo!()
    }
}
