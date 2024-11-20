use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

#[derive(Debug, Default)]
pub struct Grid {
    style: Style,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            style: Style::default(),
        }
    }
}

impl Widget for Grid {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if area.width < 46 || area.height < 19 {
            return;
        }

        let (top, middle, bottom) = Self::build_grid_strings();
        let vertical = "│";

        let start_x = area.x;
        let start_y = area.y;
        let cell_width = 5;
        let cell_height = 2;

        buf.set_string(start_x, start_y, &top, self.style);

        for row in 1..9 {
            for col in 0..=9 {
                buf.set_string(
                    start_x + (col * cell_width),
                    start_y + (row * cell_height) - 1,
                    vertical,
                    self.style,
                );
            }

            buf.set_string(start_x, start_y + (row * cell_height), &middle, self.style);
        }

        for col in 0..=9 {
            buf.set_string(
                start_x + (col * cell_width),
                start_y + 17,
                vertical,
                self.style,
            );
        }

        buf.set_string(start_x, start_y + 18, &bottom, self.style);
    }
}

impl Grid {
    fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    fn build_grid_strings() -> (String, String, String) {
        let mut top = String::new();
        top.push_str("┌─");
        top.push_str("───┬─".repeat(8).as_str());
        top.push_str("───┐\n");

        let mut middle = String::new();
        middle.push_str("├─");
        middle.push_str("───┼─".repeat(8).as_str());
        middle.push_str("───┤\n");

        let mut bottom = String::new();
        bottom.push_str("└─");
        bottom.push_str("───┴─".repeat(8).as_str());
        bottom.push_str("───┘\n");

        (top, middle, bottom)
    }
}
