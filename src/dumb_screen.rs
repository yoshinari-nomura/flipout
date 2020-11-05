use crate::ui_board::UiBoard;

pub struct DumbScreen(bool);

impl DumbScreen {
    pub fn new(dumb: bool) -> Self {
        DumbScreen(dumb)
    }

    pub fn update_screen(&self, board: &UiBoard) {
        if !self.0 {
            Self::clear_screen();
        }
        print!("{}", board);
    }

    fn clear_screen() {
        print!("\x1b[2J");
        Self::locate(1, 1);
    }

    fn locate(x: u32, y: u32) {
        print!("\x1b[{};{}H", x, y);
    }
}
