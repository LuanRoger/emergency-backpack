use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListDirection, ListItem, Padding, Row, Table as TableUi},
};

use crate::{backpack::Backpack, default_dp_exec};

fn render_backpack(frame: &mut Frame, backpack: &Backpack) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Fill(4),
            Constraint::Fill(2),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let table = &backpack.table;
    let items = &backpack.items;
    let num_columns = backpack.capacity;

    let rows: Vec<Row> = table
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            let item_name = if row_idx == 0 {
                String::from("(base)")
            } else if row_idx <= items.len() {
                items[row_idx - 1].name.clone()
            } else {
                String::from("")
            };

            let mut cells: Vec<String> = vec![format!("{:<12}", item_name)];
            cells.extend(row.iter().map(|col| format!("{:>4}", col)));
            Row::new(cells).style(Style::default()).height(2)
        })
        .collect();

    let mut widths: Vec<Constraint> = vec![Constraint::Length(14)];
    widths.extend((0..num_columns).map(|_| Constraint::Length(8)));

    let mut header_labels: Vec<String> = vec![String::from("Item")];
    header_labels.extend((0..num_columns).map(|i| format!("W:{}", i)));

    let header = Row::new(header_labels)
        .style(Style::default().bold().underlined())
        .bottom_margin(1);

    let block = Block::default()
        .title("Tabela")
        .borders(Borders::ALL)
        .padding(Padding::uniform(1));

    let table_widget = TableUi::new(rows, widths)
        .header(header)
        .block(block)
        .column_spacing(5)
        .style(Style::default().fg(Color::White));

    frame.render_widget(table_widget, outer_layout[0]);

    let items_list = backpack
        .items
        .iter()
        .map(|item| {
            ListItem::new(format!(
                "{} -> (Pesso: {}; Importância: {})",
                &item.name, item.weight, item.value
            ))
        })
        .collect::<Vec<ListItem>>();

    let list = List::new(items_list)
        .block(Block::bordered().title("Itens"))
        .style(Style::new().white())
        .direction(ListDirection::TopToBottom);

    frame.render_widget(list, outer_layout[1]);

    let info_text = format!(
        " Items: {} | Capacity: {} | Aperte qualquer tecla para sair ",
        items.len(),
        num_columns.saturating_sub(1)
    );

    let footer = Block::default()
        .title(Span::styled(
            info_text,
            Style::default().fg(Color::Green).bold(),
        ))
        .style(Style::default().bg(Color::Black));

    frame.render_widget(footer, outer_layout[2]);
}

pub fn app(terminal: &mut DefaultTerminal, capacity: u16) -> std::io::Result<()> {
    let backpack = default_dp_exec!(capacity);

    loop {
        terminal.draw(|frame| render_backpack(frame, &backpack))?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}
