pub const HELP_CONTENT: &str = include_str!("assets/help.txt");

pub fn help_line_count() -> usize {
    HELP_CONTENT.lines().count()
}
