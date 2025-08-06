/*
 * ui
 *     Specific code related to the UI initialization.
 */
pub mod tui
{
    use ratatui::{
        backend::{Backend, CrosstermBackend},
        crossterm::{
            terminal::{
                disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
            },
            ExecutableCommand,
        },
        Terminal,
    };
    use std::{
        io::{self, stdout},
        panic,
    };

    pub fn init_terminal() -> Result<Terminal<impl Backend>, io::Error>
    {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        Ok(terminal)
    }

    pub fn restore_terminal() -> Result<(), io::Error>
    {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn install_panic_hook()
    {
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info|
        {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            original_hook(panic_info);
        }));
    }
}
