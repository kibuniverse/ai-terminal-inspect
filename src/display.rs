use crossterm::style::Color;
use termimad::MadSkin;

pub fn print_error_detected() {
    let mut skin = MadSkin::default();
    skin.paragraph.set_fg(Color::Red);
    skin.print_inline("Error detected:");
    let skin = MadSkin::default();
    skin.print_text(" AI is currently analyzing...");
}

pub fn print_analysis(response: &str) {
    let mut skin = MadSkin::default();
    skin.paragraph.set_fg(Color::Green);
    skin.print_text("Analysis completed");
    let skin = MadSkin::default();
    skin.print_text(response);
}
