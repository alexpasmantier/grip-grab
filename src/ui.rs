use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, List, ListDirection, Paragraph,
    },
    Frame,
};

use crate::app::{self, App};

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

pub fn ui(frame: &mut Frame, app: &App) {
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

    let results_list = List::new(app.results.iter().map(|r| {
        Line::from(vec![
            Span::styled(r.path.to_string_lossy(), Style::default().blue()),
            Span::styled(
                format!(":{}", r.line_number),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(": "),
            Span::styled(r.line.clone(), Style::default().fg(Color::Rgb(90, 90, 110))),
        ])
    }))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().fg(Color::Red))
    //.highlight_symbol(">>")
    //.repeat_highlight_symbol(true)
    .direction(ListDirection::BottomToTop)
    .block(results_block);

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
    let result_title = if let Some(result) = &app.selected_result {
        result.path.to_string_lossy().to_string()
    } else {
        "No file selected".to_string()
    };
    let preview_file_path = Paragraph::new(result_title)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(get_border_style(false)),
        )
        .style(Style::default().dim())
        .alignment(Alignment::Left);

    // file preview
    let preview_block = Block::default()
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

    frame.render_widget(results_list, left_chunks[0]);
    frame.render_widget(input, left_chunks[1]);
    frame.render_widget(preview_file_path, right_chunks[0]);
    frame.render_widget(preview_block, right_chunks[1]);
}