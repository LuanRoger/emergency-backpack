use emergency_backpack::{algorithms::dynamic_programming, backpack::Item, ui::render_backpack};
use ratatui::DefaultTerminal;

const BACKPACK_CAPACITY: u16 = 6;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let items = vec![
        Item::new(String::from("Caderno"), 3, 2),
        Item::new(String::from("Livro"), 4, 3),
        Item::new(String::from("Caneca"), 5, 4),
        Item::new(String::from("Caneta"), 1, 1),
        Item::new(String::from("Garrafa"), 2, 2),
    ];

    let backpack = dynamic_programming(&items, BACKPACK_CAPACITY);

    loop {
        terminal.draw(|frame| render_backpack(frame, &backpack))?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}
