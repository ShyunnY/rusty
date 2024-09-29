use super::people::info;
use super::{cat::*, dog::Dog};

impl Cat {
    pub fn say(&self) {
        println!("i'm {}", self.name);
        info::infos();
    }
}

impl Dog {
    pub fn say(&self) {
        println!("i'm {}", self.name);
        crate::animals::fish::_fish_say();
    }
}
