use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::app::App;
use crate::model::CommentType;
use crate::ui::styles;

pub fn render_comment_input(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 40, frame.area());

    frame.render_widget(Clear, area);

    let comment_kind = if app.comment_is_file_level {
        "File Comment"
    } else {
        "Line Comment"
    };

    let block = Block::default()
        .title(format!(
            " {} [{}] (Ctrl-S to save, Ctrl-C/Esc to cancel) ",
            comment_kind,
            app.comment_type.as_str()
        ))
        .borders(Borders::ALL)
        .border_style(styles::border_style(true));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Build content with type selector hint and input area
    let type_style = match app.comment_type {
        CommentType::Note => Style::default()
            .fg(styles::COMMENT_NOTE)
            .add_modifier(Modifier::BOLD),
        CommentType::Suggestion => Style::default()
            .fg(styles::COMMENT_SUGGESTION)
            .add_modifier(Modifier::BOLD),
        CommentType::Issue => Style::default()
            .fg(styles::COMMENT_ISSUE)
            .add_modifier(Modifier::BOLD),
        CommentType::Praise => Style::default()
            .fg(styles::COMMENT_PRAISE)
            .add_modifier(Modifier::BOLD),
    };
    let type_hint = Line::from(vec![
        Span::styled("Type: ", styles::dim_style()),
        Span::styled(app.comment_type.as_str(), type_style),
        Span::styled(" (Tab to cycle)", styles::dim_style()),
    ]);

    let separator = Line::from(Span::styled(
        "─".repeat(inner.width as usize),
        styles::dim_style(),
    ));

    // Build content lines with cursor
    let mut lines = vec![type_hint, separator, Line::from("")];

    if app.comment_buffer.is_empty() {
        // Show placeholder with cursor at start
        lines.push(Line::from(vec![
            Span::styled(
                "│",
                Style::default()
                    .fg(styles::CURSOR_COLOR)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("Type your comment...", styles::dim_style()),
        ]));
    } else {
        // Split buffer into lines and render with cursor
        let buffer_lines: Vec<&str> = app.comment_buffer.split('\n').collect();
        let mut char_offset = 0;

        for (line_idx, text) in buffer_lines.iter().enumerate() {
            let line_start = char_offset;
            let line_end = char_offset + text.len();

            // Check if cursor is on this line
            let cursor_on_this_line = app.comment_cursor >= line_start
                && (app.comment_cursor <= line_end
                    || (line_idx == buffer_lines.len() - 1
                        && app.comment_cursor == app.comment_buffer.len()));

            if cursor_on_this_line {
                let cursor_pos_in_line = app.comment_cursor - line_start;
                let before_cursor = &text[..cursor_pos_in_line.min(text.len())];
                let after_cursor = if cursor_pos_in_line < text.len() {
                    &text[cursor_pos_in_line..]
                } else {
                    ""
                };

                lines.push(Line::from(vec![
                    Span::raw(before_cursor.to_string()),
                    Span::styled(
                        "│",
                        Style::default()
                            .fg(styles::CURSOR_COLOR)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(after_cursor.to_string()),
                ]));
            } else {
                lines.push(Line::from(Span::raw(text.to_string())));
            }

            // Account for newline character (except for last line)
            char_offset = line_end + 1;
        }
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });

    frame.render_widget(paragraph, inner);
}

pub fn format_comment_line(
    comment_type: CommentType,
    content: &str,
    line_num: Option<u32>,
) -> Line<'static> {
    let type_style = match comment_type {
        CommentType::Note => Style::default()
            .fg(styles::COMMENT_NOTE)
            .add_modifier(Modifier::BOLD),
        CommentType::Suggestion => Style::default()
            .fg(styles::COMMENT_SUGGESTION)
            .add_modifier(Modifier::BOLD),
        CommentType::Issue => Style::default()
            .fg(styles::COMMENT_ISSUE)
            .add_modifier(Modifier::BOLD),
        CommentType::Praise => Style::default()
            .fg(styles::COMMENT_PRAISE)
            .add_modifier(Modifier::BOLD),
    };

    let line_info = line_num.map(|n| format!("L{}: ", n)).unwrap_or_default();

    Line::from(vec![
        Span::styled("  💬 ", Style::default()),
        Span::raw(line_info),
        Span::styled(format!("[{}] ", comment_type.as_str()), type_style),
        Span::raw(content.to_string()),
    ])
}

pub fn render_confirm_dialog(frame: &mut Frame, message: &str) {
    let area = centered_rect(50, 20, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Confirm ")
        .borders(Borders::ALL)
        .border_style(styles::border_style(true));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::raw(message)),
        Line::from(""),
        Line::from(vec![
            Span::styled("  [Y]", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("es    "),
            Span::styled("[N]", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("o"),
        ]),
    ];

    let paragraph = Paragraph::new(lines).alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(paragraph, inner);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
