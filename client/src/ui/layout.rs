use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};

use super::{
    components::{Input, Instruction, UserName},
    layouts::{ChatRoom, Container, Textfield, UserContainer},
};
use crate::app_state::AppState;
use crate::controllers::StateEnum;

pub fn app_logs(value: String, logs: &mut Vec<String>) {
    logs.push(value);
}

pub fn initialize(app_state: &mut AppState) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, app_state);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app_state: &mut AppState) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app_state))?;
        if let Event::Key(key) = event::read()? {
            app_state
                .handles
                .update(key, app_state.state.current_state());

            app_logs(
                format!("current message {:?}", app_state.handles.message.0),
                &mut app_state.logs,
            );
            app_state.sender.update(
                key,
                app_state.state.current_state(),
                &mut app_state.handles.message.0,
            );
            if let StateEnum::Exit = app_state.state.key_state(key).unwrap() {
                return Ok(());
            }
            app_logs(
                format!("key userd {:?}", app_state.state),
                &mut app_state.logs,
            );
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let message = &app_state.handles.message;
    let intruction = Instruction::create(app_state.state.current_state());
    let input_component = Input::create(app_state.state.current_state());
    let username_component = UserName::create(app_state.state.current_state());
    let container = Container::create(f.size());
    let user_selection_container = UserContainer::create(f.size());
    if let StateEnum::NoUserName = app_state.state.current_state().unwrap() {
        let block2 = Block::default()
            .title("Enter username")
            .borders(Borders::NONE);
        f.render_widget(block2, user_selection_container.0[0]);
        f.render_widget(
            username_component.get_view(message.0.to_string()),
            user_selection_container.0[1],
        );
        return;
    }
    // container
    let logs: Vec<ListItem> = app_state
        .logs
        .iter()
        .rev()
        .map(|element| {
            let log = Spans::from(vec![Span::raw(element)]);
            ListItem::new(vec![log])
        })
        .collect();
    let list = List::new(logs).block(Block::default().borders(Borders::ALL));
    let block = Block::default().title("Chatt app").borders(Borders::NONE);

    f.render_widget(block, container.0[0]);
    f.render_widget(intruction.get_view(), container.0[1]);
    f.render_widget(list, container.0[3]);

    let chat_room = ChatRoom::create(container.0[2]);

    // chat room
    let user_block = Block::default().title("").borders(Borders::ALL);

    f.render_widget(user_block, chat_room.0[1]);

    // textfield and chat list
    let textfield = Textfield::create(chat_room.0[0]);
    let value = match app_state.messages.lock() {
        Ok(v) => {
            let messages: Vec<ListItem> = v
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let content = vec![Spans::from(Span::raw(format!(
                        "{}: {}",
                        i,
                        String::from_utf8_lossy(m)
                    )))];
                    ListItem::new(content)
                })
                .collect();
            List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"))
        }
        Err(_) => List::new(vec![]).block(Block::default().borders(Borders::ALL).title("Messages")),
    };
    f.render_widget(value, textfield.0[0]);
    f.render_widget(
        input_component.get_view(message.0.to_string()),
        textfield.0[1],
    );
}
