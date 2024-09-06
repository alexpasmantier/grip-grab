use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, List, ListDirection, Padding, Paragraph,
    },
    Frame,
};
use std::io::BufRead;
use syntect::{
    easy::HighlightFile,
    highlighting::{self, Theme, ThemeSet},
    parsing::SyntaxSet,
};

use crate::app::{self, App};
use crate::utils::highlighting::{convert_syn_color_to_ratatui_color, convert_syn_line_to_line};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn get_border_style(focused: bool) -> Style {
    if focused {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Rgb(90, 90, 110)).dim()
    }
}

pub fn ui(frame: &mut Frame, app: &App, syntax_set: &SyntaxSet, theme: &Theme) {
    let main_block = centered_rect(80, 80, frame.area());

    // split the main block into two vertical chunks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_block);

    // left block: results + input field
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(3)])
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)])
        .split(chunks[1]);

    // top left block: results
    let results_block = Block::default()
        .title(
            Title::from(" Results ")
                .position(Position::Top)
                .alignment(Alignment::Center),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(get_border_style(
            app::CurrentBlock::Results == app.current_block,
        ))
        .style(Style::default());

    let results_list = List::new(app.results_list.results.iter().map(|r| {
        let mut last_match_end = 0;
        let content_line_spans = r.matches.iter().map(|m| {
            let line_text = &r.line;
            let start = m.start;
            let end = m.end;
            let mut spans = vec![];
            if start > last_match_end {
                spans.push(Span::raw(&line_text[last_match_end..start]));
            }
            spans.push(Span::styled(
                &line_text[start..end],
                Style::default().fg(Color::Rgb(255, 150, 150)),
            ));
            last_match_end = end;
            spans
        });
        let mut content_spans = content_line_spans.flatten().collect::<Vec<Span>>();
        if last_match_end < r.line.len() {
            content_spans.push(Span::raw(&r.line[last_match_end..]));
        }
        let mut line_spans = vec![
            Span::styled(r.path.to_string_lossy(), Style::default().blue()),
            Span::styled(
                format!(":{}", r.line_number),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(": "),
        ];
        line_spans.extend(content_spans);
        Line::from(line_spans)
    }))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().bg(Color::Rgb(50, 50, 70)))
    .highlight_symbol(">>")
    //.repeat_highlight_symbol(true)
    .direction(ListDirection::BottomToTop)
    .block(results_block);

    frame.render_stateful_widget(
        results_list,
        left_chunks[0],
        &mut app.results_list.state.clone(),
    );

    // bottom left block: input field
    let input_block = Block::default()
        .title(
            Title::from(" Pattern ")
                .position(Position::Top)
                .alignment(Alignment::Center),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(get_border_style(
            app::CurrentBlock::Search == app.current_block,
        ))
        .style(Style::default());

    // input field
    let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

    let scroll = app.pattern.visual_scroll(width as usize);
    let input = Paragraph::new(app.pattern.value())
        .scroll((0, scroll as u16))
        .block(input_block);

    if let app::CurrentBlock::Search = app.current_block {
        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        frame.set_cursor_position((
            // Put cursor past the end of the input text
            left_chunks[1].x + ((app.pattern.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            // Move one line down, from the border to the input line
            left_chunks[1].y + 1,
        ))
    }

    // current file name
    let result_title: String;
    if !app.results_list.results.is_empty() {
        if let Some(selected_index) = app.results_list.state.selected() {
            let index = selected_index % app.results_list.results.len();
            let result = &app.results_list.results[index];
            result_title = result.path.to_string_lossy().to_string()
        } else {
            result_title = "Nothing selected".to_string();
        }
    } else {
        result_title = "No results".to_string();
    }
    //let preview_file_path = Paragraph::new(result_title)
    let preview_file_path = Paragraph::new(app.preview_state.scroll.0.to_string())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(get_border_style(false)),
        )
        .style(Style::default().dim())
        .alignment(Alignment::Left);

    // file preview
    let theme_bg = convert_syn_color_to_ratatui_color(
        theme
            .settings
            .background
            .unwrap_or(syntect::highlighting::Color::BLACK),
    );
    let preview_outer_block = Block::default()
        .title(
            Title::from(" Preview ")
                .position(Position::Top)
                .alignment(Alignment::Center),
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(get_border_style(
            app::CurrentBlock::Preview == app.current_block,
        ))
        .style(Style::default());

    let preview_inner_block = Block::default()
        .style(Style::default().bg(theme_bg))
        .padding(Padding {
            top: 0,
            right: 1,
            bottom: 0,
            left: 1,
        });
    let inner = preview_outer_block.inner(right_chunks[1]);
    frame.render_widget(preview_outer_block, right_chunks[1]);

    // maybe this should go into the run_app function and be cached somehow
    if !app.results_list.results.is_empty() {
        if let Some(selected_index) = app.results_list.state.selected() {
            let index = selected_index % app.results_list.results.len();
            let result = &app.results_list.results[index];
            let line_number = result.line_number;
            let mut highlighter =
                HighlightFile::new(result.path.clone(), &syntax_set, &theme).unwrap();
            let mut line = String::new();
            let mut syn_lines: Vec<Vec<(syntect::highlighting::Style, String)>> = Vec::new();
            while highlighter.reader.read_line(&mut line).unwrap() > 0 {
                {
                    let line_regions = highlighter
                        .highlight_lines
                        .highlight_line(&line, &syntax_set)
                        .unwrap();

                    let mut cloned_regions = Vec::new();
                    for region in line_regions.iter() {
                        cloned_regions.push((region.0, region.1.to_owned()));
                    }

                    syn_lines.push(cloned_regions);
                }
                line.clear();
            }

            let preview_lines: Vec<Line> = syn_lines.iter().map(convert_syn_line_to_line).collect();
            let preview_text = Text::from(preview_lines);
            let scroll: u16 = (line_number as isize - (right_chunks[1].height as isize) / 2
                + 1
                + app.preview_state.scroll.0 as isize)
                .max(0)
                // add min to prevent overflow
                .try_into()
                .unwrap();
            let preview_paragraph = Paragraph::new(preview_text)
                .block(preview_inner_block)
                .alignment(Alignment::Left)
                .scroll((scroll, 0));
            frame.render_widget(preview_paragraph, inner);
        }
    }

    frame.render_widget(input, left_chunks[1]);
    frame.render_widget(preview_file_path, right_chunks[0]);
}
