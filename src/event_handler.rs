pub mod event
{
    use crate::error_handler::error;
    use crate::consts::const_;
    use crate::enums::enum_;
    use crate::models::model;
    use ratatui::{
        crossterm::event::{self, Event, KeyCode},
    };
    use std::{time::Duration};

    #[derive(PartialEq, Debug)]
    pub enum Message
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
        Edit,
        Delete,
        Input(char),
        Backspace,
        Submit,
        Cancel,

        Quit,
    }

    pub fn handle_event(model: &model::Model) -> Result<Option<Message>, error::ApplicationError>
    {
        if event::poll(Duration::from_millis(250)).map_err(error::ApplicationError::IoError)?
        {
            if let Event::Key(key) = event::read().map_err(error::ApplicationError::IoError)?
            {
                if key.kind == event::KeyEventKind::Press
                {
                    if model.is_inputting
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

    pub fn handle_input_key(key: event::KeyEvent) -> Option<Message>
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

    pub fn handle_cmd_key(key: event::KeyEvent) -> Option<Message>
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
            KeyCode::Char('e') => Some(Message::Edit),
            KeyCode::Char('d') => Some(Message::Delete),

            KeyCode::Char('q') => Some(Message::Quit),

            _ => None,
        }
    }

    pub fn update(model: &mut model::Model, msg: Message) -> Option<Message>
    {
        match msg
        {
            Message::Right =>
            {
                if model.col == const_::MAX_COLUMNS - 1
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
                model.is_inputting = true;

                None
            }
            Message::Edit =>
            {
                model.show_popup = true;
                model.is_inputting = true;
                model.input = model.items[model.col][model.row].title.clone();
                model.is_updating = true;

                None
            }
            Message::Delete =>
            {
                if model.items[model.col].is_empty()
                {
                    return None;
                }

                model.items[model.col].remove(model.row);

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

                if model.col == const_::MAX_COLUMNS - 1
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
                model.is_inputting = false;

                if model.is_updating
                {
                    model.items[model.col][model.row].title = model.input.clone()
                }
                else
                {
                    model.items[model.col].insert(
                        model.row,
                        model::TodoItem {
                            title: model.input.clone(),
                        },
                    );
                }

                model.is_updating = false;
                model.input.clear();

                None
            }
            Message::Cancel =>
            {
                model.show_popup = false;
                model.is_inputting = false;
                model.input.clear();
                None
            }
            Message::Quit =>
            {
                model.running_state = enum_::RunningState::Done;
                None
            }
        }
    }
}
