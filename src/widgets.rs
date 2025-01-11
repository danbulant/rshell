use cushy::{styles::components::WidgetBackground, widget::MakeWidget, widgets::Container};

use crate::theme::BG_DEFAULT;

pub trait WidgetExt: MakeWidget {
    fn bar_pill(self) -> Container {
        self.expand_vertically()
            .with(&WidgetBackground, BG_DEFAULT)
            .pad()
    }
}

impl<T: MakeWidget> WidgetExt for T {}
