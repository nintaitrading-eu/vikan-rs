mod consts;
mod enums;
mod config_handler;
mod error_handler;
mod event_handler;
mod data_handler;
mod models;
mod ui;

use std::iter::zip;

use consts::const_;
use enums::enum_;
use config_handler::config;
use error_handler::error;
use event_handler::event;
use data_handler::data;
use models::model;
use ui::tui;

use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Stylize, Color},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

fn main()
{
    let mut model = model::Model
    {
        items: vec![vec![], vec![], vec![]],
        ..Default::default()
    };

    match config::ensure_config(&mut model)
    {
        Ok(()) => (),
        Err(ex) =>
        {
            println!("Error: {}", ex);
            std::process::exit(1);
        }
    }

    model = match data::load(&mut model)
    {
        Ok(Some(m)) => m,
        Ok(None) => model,
        Err(ex) =>
        {
            println!("Error: {}", ex);
            std::process::exit(1);
        }
    };

    let config: model::Configuration = match config::load()
    {
        Ok(c) => c,
        Err(ex) =>
        {
            println!("Error: {}", ex);
            std::process::exit(1);
        }
    };

    model.running_state = enum_::RunningState::Running;
    match render(model, &config)
    {
        Ok(_) => (),
        Err(ex) =>
        {
            println!("Error: {}", ex);
            std::process::exit(1);
        }
    };
}

fn render(mut model: model::Model, config: &model::Configuration) -> Result<(), error::ApplicationError>
{
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal().map_err(error::ApplicationError::IoError)?;
    while model.running_state != enum_::RunningState::Done
    {
        // Render the current view
        terminal.draw(|f| view(&mut model, config, f)).map_err(error::ApplicationError::IoError)?;

        // Handle events and map to a Message
        let mut current_msg = event::handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some()
        {
            current_msg = event::update(&mut model, current_msg.unwrap());
        }
        data::save(&mut model)?;
    }
    data::save(&mut model)?;

    tui::restore_terminal().map_err(error::ApplicationError::IoError)?;
    Ok(())
}

fn create_paragraph(item: model::TodoItem, fgcolor: u8, _bgcolor: u8) -> Paragraph<'static>
{
    Paragraph::new(format!("{}", item.title))
        .block(Block::bordered())
        //.bg(Color::Indexed(bgcolor)) // TODO: Cleanup, after I figure out what styling option I like best.
        .fg(Color::Indexed(fgcolor))
        .alignment(Alignment::Center)
}

fn view(model: &mut model::Model, config: &model::Configuration, frame: &mut Frame)
{
    use Constraint::Fill;
    let horizontal = Layout::horizontal([Fill(1); const_::MAX_COLUMNS]);
    let areas: [Rect; const_::MAX_COLUMNS] = horizontal.areas(frame.area());

    for (idx, (area, column)) in zip(areas, const_::COLUMNS).enumerate()
    {
        let mut list_component = Block::bordered()
            .title(column)
            .title_alignment(Alignment::Center);

        let inner_area = list_component.inner(area);

        // TODO: Implement themed colors from the config.
        if model.col == idx
        {
            list_component = list_component.fg(Color::Indexed(const_::LIGHT_GREEN)).bold()
        }
        else
        {
            list_component = list_component.fg(Color::Indexed(const_::GREEN));
        }

        frame.render_widget(list_component, area);

        let item_height = const_::ITEM_HEIGHT;

        let item_slots: usize = (inner_area.height / item_height).into();

        let item_areas =
            Layout::vertical(vec![Constraint::Length(item_height); item_slots]).split(inner_area);

        for (itemidx, (item, item_area)) in
            zip(model.items[idx].clone(), item_areas.iter()).enumerate()
        {
            let selectedcol = model.col;
            let selectedrow = model.row;

            let item_component = if selectedcol == idx
            {
                if selectedrow == itemidx
                {
                    create_paragraph(item, const_::LIGHT_WHITE, const_::LIGHT_YELLOW).bold()
                }
                else
                {
                    create_paragraph(item, const_::LIGHT_YELLOW, const_::LIGHT_BLACK).bold()
                }
            }
            else
            {
                if selectedrow == itemidx
                {
                    create_paragraph(item, const_::YELLOW, const_::BLACK)
                }
                else
                {
                    create_paragraph(item, const_::YELLOW, const_::BLACK)
                }
            };

            frame.render_widget(item_component, *item_area);
        }
    }

    if model.show_popup
    {
        let title = match model.popup_type
        {
            enum_::PopupType::Edit => const_::RS_EDITITEM,
            enum_::PopupType::Add => const_::RS_ADDITEM,
            enum_::PopupType::Help => const_::RS_HELP,
            _ => "",
        };
        let height = match model.popup_type
        {
            enum_::PopupType::Help => 20,
            _ => 3,
        };

        let input = Paragraph::new(model.input.as_str())
            .centered()
            .block(Block::bordered().title(title));
        let area = popup_area(frame.area(), height, 60);
        frame.render_widget(Clear, area);
        frame.render_widget(input, area);
    }
}

fn popup_area(area: Rect, height: u16, percent_x: u16) -> Rect
{
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
