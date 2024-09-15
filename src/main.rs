use std::time::Duration;
use color_eyre::{eyre::Context, Result};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    style::Color,
    widgets::{canvas::*, *},
    DefaultTerminal, Frame,
};

pub mod calc;
pub mod objects;
pub mod templ;
pub mod util;

/// This is a bare minimum example. There are many approaches to running an application loop, so
/// this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
/// teardown of a terminal application.
///
/// This example does not handle events or update the application state. It just draws a greeting
/// and exits when the user presses 'q'.
fn main() -> Result<()> {
    color_eyre::install()?; // augment errors / panics with easy to read messages
    let terminal = ratatui::init();
    let app_result = run(terminal).context("app loop failed");
    ratatui::restore();
    app_result
}

/// Run the application loop. This is where you would handle events and update the application
/// state. This example exits when the user presses 'q'. Other styles of application loops are
/// possible, for example, you could have multiple application states and switch between them based
/// on events, or you could have a single application state and update it based on events.
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

/// Render the application. This is where you would draw the application UI. This example draws a
/// greeting.
fn draw(frame: &mut Frame) {
    let canvas = Canvas::default()
        .block(Block::bordered().title("Canvas"))
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.layer();
            ctx.draw(&Line {
                x1: -50.0,
                y1: 0.0,
                x2: 50.0,
                y2: 0.0,
                color: Color::Red,
            });
            ctx.draw(&Line {
                x1: 0.0,
                y1: -50.0,
                x2: 0.0,
                y2: 50.0,
                color: Color::Red,
            });
            ctx.draw(&Points {
                coords: &calc::get_view_pos(objects::POINTS),
                color: Color::White,
            })
        });
    frame.render_widget(canvas, frame.area());
}

/// Check if the user has pressed 'q'. This is where you would handle events. This example just
/// checks if the user has pressed 'q' and returns true if they have. It does not handle any other
/// events. There is a 250ms timeout on the event poll to ensure that the terminal is rendered at
/// least once every 250ms. This allows you to do other work in the application loop, such as
/// updating the application state, without blocking the event loop for too long.
fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
