use ratatui::{
    style::{Style, Stylize},
    text::Span,
};
use syntect;

pub fn convert_syn_region_to_span<'a>(
    syn_region: (syntect::highlighting::Style, String),
    background: Option<syntect::highlighting::Color>,
) -> Span<'a> {
    let mut style =
        Style::default().fg(convert_syn_color_to_ratatui_color(syn_region.0.foreground));
    if let Some(background) = background {
        style = style.bg(convert_syn_color_to_ratatui_color(background));
    }
    style = match syn_region.0.font_style {
        syntect::highlighting::FontStyle::BOLD => style.bold(),
        syntect::highlighting::FontStyle::ITALIC => style.italic(),
        syntect::highlighting::FontStyle::UNDERLINE => style.underlined(),
        _ => style,
    };
    Span::styled(syn_region.1.clone(), style)
}

pub fn convert_syn_color_to_ratatui_color(
    color: syntect::highlighting::Color,
) -> ratatui::style::Color {
    ratatui::style::Color::Rgb(color.r, color.g, color.b)
}
