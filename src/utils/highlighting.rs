use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
};
use syntect;

pub fn convert_syn_line_to_line(syn_line: &Vec<(syntect::highlighting::Style, String)>) -> Line {
    Line::from_iter(syn_line.iter().map(|sr| convert_syn_region_to_span(sr)))
}

fn convert_syn_region_to_span(syn_region: &(syntect::highlighting::Style, String)) -> Span {
    let mut style = Style::default()
        .fg(ratatui::style::Color::Rgb(
            syn_region.0.foreground.r,
            syn_region.0.foreground.g,
            syn_region.0.foreground.b,
        ))
        .bg(ratatui::style::Color::Rgb(
            syn_region.0.background.r,
            syn_region.0.background.g,
            syn_region.0.background.b,
        ));
    style = match syn_region.0.font_style {
        syntect::highlighting::FontStyle::BOLD => style.bold(),
        syntect::highlighting::FontStyle::ITALIC => style.italic(),
        syntect::highlighting::FontStyle::UNDERLINE => style.underlined(),
        _ => style,
    };
    Span::styled(syn_region.1.clone(), style)
}

pub fn convert_syn_color_to_ratatui_color(
    syn_color: syntect::highlighting::Color,
) -> ratatui::style::Color {
    ratatui::style::Color::Rgb(syn_color.r, syn_color.g, syn_color.b)
}
