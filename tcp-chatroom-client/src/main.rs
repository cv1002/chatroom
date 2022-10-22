use common::Message;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{collections::VecDeque, error::Error, io, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::Mutex,
    time::{sleep, Duration},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// History of recorded messages
    messages: VecDeque<Message>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            messages: VecDeque::with_capacity(50),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app).await;

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

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: App) -> io::Result<()> {
    let app = Arc::new(Mutex::new(app));
    let (client, mut server) = TcpStream::connect("localhost:8080").await?.into_split();

    server.write("Hello: GoToGroup\n\0".as_bytes()).await?;

    tokio::spawn({
        let app = app.clone();
        async move {
            let mut lines = BufReader::new(client).lines();
            loop {
                while let Ok(Some(line)) = lines.next_line().await {
                    if let Ok(msg) = serde_json::from_str::<Message>(line.as_str()) {
                        let mut app = app.lock().await;
                        app.messages.push_back(msg);
                        if app.messages.len() > 50 {
                            app.messages.pop_front();
                        }
                    }
                }
                sleep(Duration::from_secs(1)).await;
            }
        }
    });

    loop {
        let mut app = app.lock().await;
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    use chrono::prelude::*;
                    let msg = serde_json::to_string(&Message {
                        sender: "Man".to_string(),
                        send_time: Utc::now().to_string(),
                        message: app.input.clone(),
                    })
                    .unwrap();
                    server.write(msg.as_bytes()).await?;
                    server.write("\n".as_bytes()).await?;
                    app.input.clear();
                }
                KeyCode::Char(c) => {
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                KeyCode::Esc => {
                    return Ok(());
                }
                _ => {}
            }
        }
        terminal.draw(|f| ui(f, &app))?;
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = (
        vec![
            Span::raw("Press "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to quit, "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to record the message"),
        ],
        Style::default(),
    );
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(input, chunks[1]);
    // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
    f.set_cursor(
        // Put cursor past the end of the input text
        chunks[1].x + app.input.chars().collect::<Vec<char>>().len() as u16 + 1,
        // Move one line down, from the border to the input line
        chunks[1].y + 1,
    );

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .rev()
        .take((chunks[2].height / 4).into())
        .rev()
        .map(|(id, msg)| {
            let color = match id % 4 {
                0 => Style::default().fg(Color::Red),
                1 => Style::default().fg(Color::Magenta),
                2 => Style::default().fg(Color::Yellow),
                3 => Style::default().fg(Color::Blue),
                _ => Style::default(),
            };
            let header = Spans::from(vec![
                Span::styled(format!("{:<9}", msg.sender), color),
                Span::raw(" "),
                Span::styled(
                    msg.send_time.clone(),
                    Style::default().add_modifier(Modifier::ITALIC),
                ),
            ]);
            let content = vec![
                Spans::from("-".repeat(chunks[2].width as usize)),
                header,
                Spans::from(""),
                Spans::from(Span::raw(format!("{}", msg.message))),
            ];
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, chunks[2]);
}
