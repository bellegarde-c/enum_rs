use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::{self, prelude::*};
use once_cell::sync::OnceCell;

#[derive(glib::Enum, Debug, Copy, Clone, PartialEq, Eq)]
#[enum_type(name = "ModelType")]
pub enum ModelType {
    Genre,
    Artist,
    Album,
    Track,
    Playlist,
}

mod imp {
    use super::*;
    use glib::Properties;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::TagBar)]
    pub struct TagBar {
        #[property(get, set)]
        pub model_type: OnceCell<ModelType>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TagBar {
        const NAME: &'static str = "TagBar";
        type ParentType = glib::Object;
        type Type = super::TagBar;
    }

    impl ObjectImpl for TagBar {
    }
}

glib::wrapper! {
    pub struct TagBar(ObjectSubclass<imp::TagBar>)
        @extends gtk::Widget;
}

fn main() {}
