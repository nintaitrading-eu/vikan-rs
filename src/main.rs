mod consts;
mod enums;
mod error_handler;
mod event_handler;
mod models;
mod ui;

use std::fs;
use std::fs::File;
use std::io::{Write};
use std::iter::zip;

use consts::const_;
use enums::enum_;
use error_handler::error;
use event_handler::event;
use models::model;
use ui::tui;

use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::Stylize,
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use serde::{Serialize, Deserialize};
use serde_json::Result;

fn main()
{
    let mut model = model::Model
    {
        items: vec![vec![], vec![], vec![]],
        ..Default::default()
    };

    match load_data()
    {
        Ok(_) => (),
        Err(ex) =>
        {
            println!("Error: {}", ex);
            std::process::exit(1);
        }
    };

    // TODO: map loaded data to the model

    match render(model)
    {
        Ok(_) => (),
        Err(ex) =>
        {
            println!("Error: {}", ex);
            std::process::exit(1);
        }
    };
}

fn load_data() -> Result<()>
{
    let json_data = fs::read_to_string("data.json").map_err(error::ApplicationError::IoError)?;
    let m: model::Model = serde_json::from_str(&json_data).map_err(error::ApplicationError::JsonError)?;
    println!("{:?}", m);
    Ok(())
}

fn render(mut model: model::Model) -> Result<()>
{
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal().map_err(error::ApplicationError::IoError)?;
    while model.running_state != enum_::RunningState::Done
    {
        // Render the current view
        terminal.draw(|f| view(&mut model, f)).map_err(error::ApplicationError::IoError)?;

        // Handle events and map to a Message
        let mut current_msg = event::handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some()
        {
            current_msg = event::update(&mut model, current_msg.unwrap());
        }
    }

    let json_data = serde_json::to_string_pretty(&model).unwrap();
    let mut file = File::create("data.json").map_err(error::ApplicationError::IoError)?;
    file.write_all(json_data.as_bytes()).map_err(error::ApplicationError::IoError)?;

    tui::restore_terminal().map_err(error::ApplicationError::IoError)?;
    Ok(())
}

fn view(model: &mut model::Model, frame: &mut Frame)
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

        if model.col == idx
        {
            list_component = list_component.green();
        }

        frame.render_widget(list_component, area);

        let item_height = 1;

        let item_slots: usize = (inner_area.height / item_height).into();

        let item_areas =
            Layout::vertical(vec![Constraint::Length(item_height); item_slots]).split(inner_area);

        for (item_idx, (item, item_area)) in
            zip(model.items[idx].clone(), item_areas.iter()).enumerate()
        {
            /*let mut container = Block::bordered().title("").reset();
            if model.col == idx && model.row == item_idx {
                container = container.green();
            }*/

            let item_component = Paragraph::new(format!("{}. {}", item_idx, item.title));

            frame.render_widget(item_component, *item_area);
        }
    }

    if model.show_popup
    {
        let input = Paragraph::new(model.input.as_str())
            .centered()
            .block(Block::bordered().title(const_::RsAddItem));
        let area = popup_area(frame.area(), 60);
        frame.render_widget(Clear, area);
        frame.render_widget(input, area);
    }
}

fn popup_area(area: Rect, percent_x: u16) -> Rect
{
    let vertical = Layout::vertical([Constraint::Length(3)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
