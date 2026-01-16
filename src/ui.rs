use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Padding, Row, Table as TableUi},
};

use crate::table::Table;

pub fn render_table(frame: &mut Frame, table: &Table) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(98), Constraint::Fill(1)])
        .split(frame.area());

    let num_columns = table.first().map(|row| row.len()).unwrap_or(0);
    let num_rows = table.len();

    let rows: Vec<Row> = table
        .iter()
        .map(|row| {
            let cells: Vec<String> = row.iter().map(|col| format!("{:>4}", col)).collect();
            Row::new(cells).style(Style::default()).height(2)
        })
        .collect();

    let widths: Vec<Constraint> = (0..num_columns).map(|_| Constraint::Length(8)).collect();
    let header_labels: Vec<String> = (0..num_columns).map(|i| format!("W:{}", i)).collect();

    let header = Row::new(header_labels)
        .style(Style::default().bold().underlined())
        .bottom_margin(1);

    let block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::uniform(1));

    let table_widget = TableUi::new(rows, widths)
        .header(header)
        .block(block)
        .column_spacing(5)
        .style(Style::default().fg(Color::White));

    frame.render_widget(table_widget, outer_layout[0]);

    let info_text = format!(
        " Items: {} | Capacity: {} | Press any key to exit ",
        num_rows.saturating_sub(1),
        num_columns.saturating_sub(1)
    );

    let footer = Block::default()
        .title(Span::styled(
            info_text,
            Style::default().fg(Color::Green).bold(),
        ))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(footer, outer_layout[1]);
}
