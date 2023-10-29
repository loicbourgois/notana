use std::{io::stdout, time::Duration};
use std::io::Stdout;
use std::io;
use futures::{future::FutureExt, select};
use futures_timer::Delay;
use serde::{Serialize, Deserialize};
use std::io::Write;
use std::sync::{Arc, RwLock};
use std::env;
use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use crossterm::style::{Color::{Green, Black, Grey,DarkGrey}, Colors, Print, SetColors};
use crossterm::{
    cursor::position,
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode, KeyModifiers},
    execute,
    cursor,
    terminal::{disable_raw_mode, enable_raw_mode, ClearType},
    terminal,
};
use std::cmp::min;
use taskini_common::{
    Tasks,
    Task
};
struct Command {
    kind: CommandKind,
    string: String,
}
#[derive(Debug)]
pub enum CommandKind {
    AddTask,
    Find,
}
pub struct State {
    command: Command,
    c: usize,
    tasks: Tasks,
    event: Option<Event>,
    cursor_x: usize,
}
fn print_at(stdout: &mut Stdout, x:u16, y:u16, str_: &String) {
    execute!(
        stdout, 
        SetColors(Colors::new(Grey, Black)), 
        cursor::MoveTo(x, y),
        Print(str_),
    );
}
fn print_at_highlighted(stdout: &mut Stdout, x:u16, y:u16, str_: &String) {
    execute!(
        stdout, 
        SetColors(Colors::new(Green, Black)), 
        cursor::MoveTo(x, y),
        Print(str_),
    );
}
fn print_at_cursor(stdout: &mut Stdout, x:u16, y:u16, str_: &String) {
    execute!(
        stdout, 
        SetColors(Colors::new(Green, DarkGrey)),
        cursor::MoveTo(x, y),
        Print(str_),
    );
}
fn render(shared_state: &SharedState) {
    let mut stdout = stdout();
    let (width, height) = terminal::size().unwrap();
    // for x in 0..height {
    //     execute!(stdout, cursor::MoveTo(0, x));
    //     for x in 0..width {
    //         print!(" ");
    //     }
    // }
    let state = shared_state.read().unwrap();
    print_at(&mut stdout, 0, 0, &format!("count: {}", state.c));
    print_at(&mut stdout, 16, 1, &format!("event: {:?}", state.event));
    print_at(&mut stdout, 0, 3, &format!("Tasks ({})", state.tasks.len()));
    for i in 0..min(state.tasks.len(), (height-9).into() ) {
        execute!(stdout, cursor::MoveTo(1, (3+i+1).try_into().unwrap() ));
        print!(" {}", i);
        execute!(stdout, cursor::MoveTo(8, (3+i+1).try_into().unwrap() ));
        print!(" {}", state.tasks[i].title);
    }
    let command_str_pre = format!("{:?}", state.command.kind);
    let command_str_right = format!("{} ", state.command.string);
    let mut command_str = format!("{}: {}", command_str_pre, command_str_right);
    for _ in 0..20 {
        command_str += " ";
    }
    print_at_highlighted(&mut stdout, 0, height-1, &command_str);
    print_at_cursor(
        &mut stdout, 
        (state.cursor_x+ command_str_pre.len() + 2).try_into().unwrap() , 
        height-1, 
        &command_str_right[state.cursor_x..state.cursor_x+1].to_string()
    );
}
async fn client_loop(
    tx: futures_channel::mpsc::UnboundedSender<Message>,
    shared_state: SharedState,
) {
    let mut stdout = stdout();
    let mut reader = EventStream::new();
    loop {
        let mut delay = Delay::new(Duration::from_millis(10)).fuse();
        let mut event = reader.next().fuse();
        select! {
            _ = delay => { 
                shared_state.write().unwrap().c += 1;
                render(&shared_state);
            },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        {
                        let mut state = shared_state.write().unwrap();
                        state.event = Some(event.clone());
                        }
                        if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }
                        match event {
                            Event::Key(crossterm::event::KeyEvent{code, modifiers, kind, state}) => {
                                
                                match (code, modifiers) {
                                    (KeyCode::Char(c), KeyModifiers::CONTROL) => {
                                        break;
                                    },
                                    (KeyCode::Char(c), _) => {
                                        let mut state = shared_state.write().unwrap();
                                        state.command.string += &format!("{}",c);
                                        state.cursor_x += 1;
                                        // tx.unbounded_send(Message::Binary(format!("{}",c).as_bytes().to_vec())).unwrap();
                                    },
                                    (KeyCode::Backspace, _) => {
                                        let mut state = shared_state.write().unwrap();
                                        if state.cursor_x > 0 {
                                            state.cursor_x -= 1;
                                        }
                                    },
                                    (KeyCode::Enter, _) => {
                                        let mut state = shared_state.write().unwrap();
                                        tx.unbounded_send(Message::Binary(state.command.string.as_bytes().to_vec())).unwrap();
                                        state.command.string = format!("");
                                        state.cursor_x = 0;
                                    },
                                    _ => {}
                                }
                            },
                            Event::Resize(w,h) => {
                                reset_screen(&mut stdout, w, h);
                            },
                            _ => {}
                        }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };
    }
}
async fn start_client(shared_state: SharedState) {
    let mut stdout = stdout();
    let url = url::Url::parse("ws://127.0.0.1:8080").unwrap();
    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(client_loop(stdin_tx, shared_state.clone()));
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    execute!(stdout, cursor::MoveTo(16, 0));
    print!("WebSocket handshake has been successfully completed");
    let (write, read) = ws_stream.split();
    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let mut stdout_ = io::stdout();
            let (width, height) = terminal::size().unwrap();
            match message {
                Ok(value) => {
                    let data = value.into_data();
                    let tasks: Tasks = serde_json::from_str(std::str::from_utf8(&data).unwrap()).unwrap();
                    shared_state.write().unwrap().tasks = tasks;
                },
                Err(e) => {
                    execute!(stdout_, cursor::MoveTo(16, 2));
                    print!("{}",e);
                }
            }
        })
    };
    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}
fn reset_screen(stdout: &mut Stdout, width: u16, height: u16) -> std::io::Result<()> {
    disable_raw_mode();
    for x in 0..height-1 {
        println!("{}",x);
    }
    print!("{}",height-1);
    enable_raw_mode()?;
    execute!(
        stdout,
        crossterm::cursor::Hide,
    );
    for x in 0..height {
        execute!(stdout, cursor::MoveTo(0, x));
        for x in 0..width {
            print!(" ");
        }
    }
    stdout.flush();
    Ok(())
}
type SharedState = Arc<RwLock<State>>;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut state = State {
        command: Command {
            kind: CommandKind::AddTask,
            string: "".to_string(),
        },
        c: 0,
        tasks: Tasks::new(),
        event: None,
        cursor_x: 0,
    };
    let shared_state = SharedState::new(RwLock::new(state));
    let mut stdout = stdout();
    let (width, height) = terminal::size().unwrap();
    reset_screen(&mut stdout, width, height);
    start_client(shared_state.clone()).await;
    execute!(
        stdout,
        crossterm::cursor::Show,
    );
    disable_raw_mode();
    execute!(stdout, cursor::MoveTo(0, height));
    Ok(())
}
