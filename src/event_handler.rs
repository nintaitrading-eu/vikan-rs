/*
 * event_handler
 *     Handle the UI interactions.
 */
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
        CursorLeft,
        CursorRight,
        CursorHome,
        CursorEnd,
        Backspace,
        DeleteForward,
        Submit,
        Cancel,
        Help,

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
            KeyCode::Left => Some(Message::CursorLeft),
            KeyCode::Right => Some(Message::CursorRight),
            KeyCode::Home => Some(Message::CursorHome),
            KeyCode::End => Some(Message::CursorEnd),
            KeyCode::Backspace => Some(Message::Backspace),
            KeyCode::Delete => Some(Message::DeleteForward),
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

            KeyCode::Char('?') => Some(Message::Help),

            KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Esc => Some(Message::Cancel),

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
                model.input.clear();
                model.input_cursor = 0;
                model.popup_type = enum_::PopupType::Add;

                None
            }
            Message::Edit =>
            {
                if model.items[model.col].is_empty()
                {
                    return None;
                }

                model.show_popup = true;
                model.is_inputting = true;
                model.input = model.items[model.col][model.row].title.clone();
                model.input_cursor = model.input.len();
                model.popup_type = enum_::PopupType::Edit;

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
                model.input.insert(model.input_cursor, c);
                model.input_cursor += c.len_utf8();
                None
            }
            Message::CursorLeft =>
            {
                if model.input_cursor > 0
                {
                    model.input_cursor = model.input[..model.input_cursor]
                        .char_indices().last().map_or(0, |(index, _)| index);
                }
                None
            }
            Message::CursorRight =>
            {
                if model.input_cursor < model.input.len()
                {
                    model.input_cursor += model.input[model.input_cursor..]
                        .chars().next().unwrap().len_utf8();
                }
                None
            }
            Message::CursorHome =>
            {
                model.input_cursor = 0;
                None
            }
            Message::CursorEnd =>
            {
                model.input_cursor = model.input.len();
                None
            }
            Message::Backspace =>
            {
                if model.input_cursor > 0
                {
                    let previous = model.input[..model.input_cursor]
                        .char_indices().last().map_or(0, |(index, _)| index);
                    model.input.drain(previous..model.input_cursor);
                    model.input_cursor = previous;
                }
                None
            }
            Message::DeleteForward =>
            {
                if let Some(c) = model.input[model.input_cursor..].chars().next()
                {
                    model.input.drain(model.input_cursor..model.input_cursor + c.len_utf8());
                }
                None
            }
            Message::Submit =>
            {
                model.show_popup = false;
                model.is_inputting = false;

                if model.popup_type == enum_::PopupType::Add
                {
                    model.items[model.col].insert(
                        model.row,
                        model::TodoItem {
                            title: model.input.clone(),
                        },
                    )
                }
                else
                {
                    model.items[model.col][model.row].title = model.input.clone();
                }

                model.popup_type = enum_::PopupType::None;
                model.input.clear();
                model.input_cursor = 0;

                None
            }
            Message::Cancel =>
            {
                model.show_popup = false;
                model.is_inputting = false;
                model.input.clear();
                model.input_cursor = 0;
                None
            }
            Message::Help =>
            {
                model.show_popup = true;
                model.is_inputting = false;
                model.popup_type = enum_::PopupType::Help;
                model.input = String::from(const_::RS_HELP_COMMANDS);
                None
            }
            Message::Quit =>
            {
                model.running_state = enum_::RunningState::Done;
                model.show_popup = false;
                model.is_inputting = false;
                model.popup_type = enum_::PopupType::None;
                None
            }
        }
    }

    #[cfg(test)]
    mod tests
    {
        use super::*;

        #[test]
        fn edit_inserts_and_deletes_at_cursor()
        {
            let mut model = model::Model {
                items: vec![vec![model::TodoItem { title: "cat".into() }], vec![], vec![]],
                ..Default::default()
            };

            update(&mut model, Message::Edit);
            assert_eq!(model.input_cursor, 3);
            update(&mut model, Message::CursorLeft);
            update(&mut model, Message::Input('r'));
            assert_eq!(model.input, "cart");
            update(&mut model, Message::Backspace);
            assert_eq!(model.input, "cat");
            update(&mut model, Message::DeleteForward);
            assert_eq!(model.input, "ca");
            update(&mut model, Message::CursorHome);
            update(&mut model, Message::DeleteForward);
            assert_eq!(model.input, "a");
            update(&mut model, Message::Submit);
            assert_eq!(model.items[0][0].title, "a");
        }

        #[test]
        fn cursor_moves_over_multibyte_characters()
        {
            let mut model = model::Model {
                items: vec![vec![model::TodoItem { title: "aé日".into() }], vec![], vec![]],
                ..Default::default()
            };

            update(&mut model, Message::Edit);
            update(&mut model, Message::CursorLeft);
            update(&mut model, Message::CursorLeft);
            assert_eq!(model.input_cursor, 1);
            update(&mut model, Message::Backspace);
            assert_eq!(model.input, "é日");
            update(&mut model, Message::CursorRight);
            update(&mut model, Message::Input('!'));
            assert_eq!(model.input, "é!日");
            update(&mut model, Message::Cancel);
            assert_eq!(model.items[0][0].title, "aé日");
        }
    }
}
