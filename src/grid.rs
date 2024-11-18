use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{BorderType, Widget},
};

#[derive(Debug, Default)]
pub struct Grid {
    rows: u16,
    cols: u16,
    // borders: Borders,
    // style: Style,
}

impl Grid {
    pub const fn new() -> Self {
        Self { rows: 0, cols: 0 }
    }
}

impl Widget for Grid {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.render_border(area, buf);
    }
}

impl Grid {
    fn render_border(&self, area: Rect, buf: &mut Buffer) {
        self.render_left_side(area, buf);
        self.render_top_side(area, buf);
        self.render_right_side(area, buf);
        self.render_bottom_side(area, buf);

        self.render_bottom_right_corner(area, buf);
        self.render_top_right_corner(area, buf);
        self.render_bottom_left_corner(area, buf);
        self.render_top_left_corner(area, buf);
    }

    fn render_left_side(&self, area: Rect, buf: &mut Buffer) {
        for y in area.top()..area.bottom() {
            buf[(area.left(), y)]
                .set_symbol(BorderType::Plain.to_border_set().vertical_left)
                .set_style(Style::default());
        }
    }

    fn render_top_side(&self, area: Rect, buf: &mut Buffer) {
        for x in area.left()..area.right() {
            buf[(x, area.top())]
                .set_symbol(BorderType::Plain.to_border_set().horizontal_top)
                .set_style(Style::default());
        }
    }

    fn render_right_side(&self, area: Rect, buf: &mut Buffer) {
        let x = area.right() - 1;
        for y in area.top()..area.bottom() {
            buf[(x, y)]
                .set_symbol(BorderType::Plain.to_border_set().vertical_right)
                .set_style(Style::default());
        }
    }

    fn render_bottom_side(&self, area: Rect, buf: &mut Buffer) {
        let y = area.bottom() - 1;
        for x in area.left()..area.right() {
            buf[(x, y)]
                .set_symbol(BorderType::Plain.to_border_set().horizontal_bottom)
                .set_style(Style::default());
        }
    }

    fn render_bottom_right_corner(&self, area: Rect, buf: &mut Buffer) {
        buf[(area.right() - 1, area.bottom() - 1)]
            .set_symbol(BorderType::Plain.to_border_set().bottom_right)
            .set_style(Style::default());
    }

    fn render_top_right_corner(&self, area: Rect, buf: &mut Buffer) {
        buf[(area.right() - 1, area.top())]
            .set_symbol(BorderType::Plain.to_border_set().top_right)
            .set_style(Style::default());
    }

    fn render_bottom_left_corner(&self, area: Rect, buf: &mut Buffer) {
        buf[(area.left(), area.bottom() - 1)]
            .set_symbol(BorderType::Plain.to_border_set().bottom_left)
            .set_style(Style::default());
    }

    fn render_top_left_corner(&self, area: Rect, buf: &mut Buffer) {
        buf[(area.left(), area.top())]
            .set_symbol(BorderType::Plain.to_border_set().top_left)
            .set_style(Style::default());
    }
}
