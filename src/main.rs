use emergency_backpack::{
    app,
    cli::{cli, get_args},
};

const BACKPACK_CAPACITY: u16 = 10;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = get_args();
    if args.no_ui {
        cli(BACKPACK_CAPACITY);
        return Ok(());
    }

    ratatui::run(|terminal| app(terminal, BACKPACK_CAPACITY))?;
    Ok(())
}
