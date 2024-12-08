use raylib::math::Rectangle;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Button {
    pub content: String,
    pub size: Rectangle,
    highlighted: bool,
    clicked: bool,
}

impl Button {
    pub fn is_highlighted(&self) -> &bool {
        &self.highlighted
    }

    pub fn is_clicked(&self) -> &bool {
        &self.clicked
    }

    // TODO
    pub fn render(&self) {
    }
}
