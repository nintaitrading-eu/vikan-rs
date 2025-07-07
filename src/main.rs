mod ui;
mod errors;

use std::{io, iter::zip, time::Duration, iter::Iterator};
use std::fs;
use std::fs::File;
use std::io::Write;

use ui::tui;
use errors::error;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::Stylize,
    widgets::{Block, Clear, Paragraph},
    Frame,
};

//use serde::{Serialize, Deserialize};
//use serde_json::{Result, Error};


//#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[derive(Debug, Default, Clone)]
struct TodoItem
{
    title: String,
}

//#[derive(Serialize, Deserialize, Debug, Default)]
#[derive(Debug, Default)]
struct Model
{
    col: usize,
    row: usize,
    items: Vec<Vec<TodoItem>>,
    running_state: RunningState,
    show_popup: bool,
    inputting: bool,
    input: String,
}

//#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState
{
    #[default]
    Running,
    Done,
}

#[derive(PartialEq, Debug)]
enum Message
{
    Left,
    Right,
    Up,
    Down,

    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,

    Add,
    Input(char),
    Backspace,
    Submit,
    Cancel,

    Quit,
}

const COLUMNS: [&str; 3] = ["TODO", "IN PROGRESS", "DONE"];
const MAX_COLUMNS: usize = COLUMNS.len();

fn main() -> Result<(), error::DataError>
{
    //let json_data = fs::read_to_string("data.json").ok()?;
    //let m: Model = serde_json::from_str(&json_data).ok()?;
    //println!("model.row: {}", m.row);
    //println!("model.col: {}", m.col);

    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut model = Model
    {
        items: vec![vec![], vec![], vec![]],
        ..Default::default()
    };

    while model.running_state != RunningState::Done
    {
        // Render the current view
        terminal.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some()
        {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;
    Ok(())
}

fn view(model: &mut Model, frame: &mut Frame)
{
    use Constraint::Fill;
    let horizontal = Layout::horizontal([Fill(1); MAX_COLUMNS]);
    let areas: [Rect; MAX_COLUMNS] = horizontal.areas(frame.area());

    for (idx, (area, column)) in zip(areas, COLUMNS).enumerate()
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
            .block(Block::bordered().title("Add Item"));
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

fn handle_event(model: &Model) -> Result<Option<Message>, io::Error>
{
    if event::poll(Duration::from_millis(250))?
    {
        if let Event::Key(key) = event::read()?
        {
            if key.kind == event::KeyEventKind::Press
            {
                if model.inputting
                {
                    return Ok(handle_input_key(key));
                }
                else
                {
                    return Ok(handle_cmd_key(key));
                }
            }
        }
    }
    Ok(None)
}

fn handle_input_key(key: event::KeyEvent) -> Option<Message>
{
    match key.code
    {
        KeyCode::Char(c) => Some(Message::Input(c)),
        KeyCode::Backspace => Some(Message::Backspace),
        KeyCode::Enter => Some(Message::Submit),
        KeyCode::Esc => Some(Message::Cancel),
        _ => None,
    }
}

fn handle_cmd_key(key: event::KeyEvent) -> Option<Message>
{
    match key.code
    {
        KeyCode::Char('h') => Some(Message::Left),
        KeyCode::Char('j') => Some(Message::Down),
        KeyCode::Char('k') => Some(Message::Up),
        KeyCode::Char('l') => Some(Message::Right),

        KeyCode::Char('H') => Some(Message::MoveLeft),
        KeyCode::Char('J') => Some(Message::MoveDown),
        KeyCode::Char('K') => Some(Message::MoveUp),
        KeyCode::Char('L') => Some(Message::MoveRight),

        KeyCode::Char('a') => Some(Message::Add),

        KeyCode::Char('q') => Some(Message::Quit),

        _ => None,
    }
}

fn update(model: &mut Model, msg: Message) -> Option<Message>
{
    match msg
    {
        Message::Right =>
        {
            if model.col == MAX_COLUMNS - 1
            {
                return None;
            }

            model.col += 1;

            if model.items[model.col].is_empty()
            {
                model.row = 0
            }
            else
            {
                model.row = model.row.min(model.items[model.col].len() - 1);
            }

            None
        }
        Message::Left =>
        {
            if model.col == 0
            {
                return None;
            }

            model.col = model.col.saturating_sub(1);
            if model.items[model.col].is_empty()
            {
                model.row = 0
            }
            else
            {
                model.row = model.row.min(model.items[model.col].len() - 1);
            }

            None
        }
        Message::Up =>
        {
            if model.row == 0
            {
                return None;
            }

            model.row = model.row.saturating_sub(1);
            None
        }
        Message::Down =>
        {
            if model.items[model.col].is_empty()
            {
                return None;
            }

            model.row = (model.row + 1).min(model.items[model.col].len() - 1);
            None
        }
        Message::Add =>
        {
            model.show_popup = true;
            model.inputting = true;

            None
        }
        Message::MoveLeft =>
        {
            if model.items[model.col].is_empty()
            {
                return None;
            }

            if model.col == 0
            {
                return None;
            }

            let to_col = model.col - 1;

            let to_col_len = model.items[to_col].len();

            let item = model.items[model.col].remove(model.row);
            model.items[to_col].insert(model.row.min(to_col_len), item);
            Some(Message::Left)
        }
        Message::MoveRight =>
        {
            if model.items[model.col].is_empty()
            {
                return None;
            }

            if model.col == MAX_COLUMNS - 1
            {
                return None;
            }

            let to_col = model.col + 1;

            let to_col_len = model.items[to_col].len();

            let item = model.items[model.col].remove(model.row);
            model.items[to_col].insert(model.row.min(to_col_len), item);
            Some(Message::Right)
        }

        Message::MoveUp =>
        {
            if model.items[model.col].is_empty()
            {
                return None;
            }

            if model.row == 0
            {
                return None;
            }

            model.items[model.col].swap(model.row, model.row - 1);

            Some(Message::Up)
        }
        Message::MoveDown =>
        {
            if model.items[model.col].is_empty()
            {
                return None;
            }

            if model.row >= model.items[model.col].len() - 1
            {
                return None;
            }

            model.items[model.col].swap(model.row, model.row + 1);

            Some(Message::Down)
        }
        Message::Input(c) =>
        {
            model.input.push(c);
            None
        }
        Message::Backspace =>
        {
            model.input.pop();
            None
        }
        Message::Submit =>
        {
            model.show_popup = false;
            model.inputting = false;

            model.items[model.col].insert(
                model.row,
                TodoItem {
                    title: model.input.clone(),
                },
            );

            model.input.clear();

            None
        }
        Message::Cancel =>
        {
            model.show_popup = false;
            model.inputting = false;
            model.input.clear();
            None
        }
        Message::Quit =>
        {
            model.running_state = RunningState::Done;
            //let json_data = serde_json::to_string_pretty(&model).unwrap();
            //let mut file = File::create("data.json").ok()?;
            //file.write_all(json_data.as_bytes()).ok()?;
            None
        }
    }
}
