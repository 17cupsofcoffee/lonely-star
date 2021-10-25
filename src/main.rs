#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::panic;

use tetra::ContextBuilder;

use lonely_star::{GameState, SCREEN_HEIGHT, SCREEN_WIDTH};

fn main() {
    panic::set_hook(Box::new(|e| {
        let msg = e.to_string();
        report_error(&msg);
    }));

    if let Err(e) = run() {
        let msg = format!("{}", e);
        report_error(&msg);
        std::process::exit(1);
    }
}

fn run() -> tetra::Result {
    ContextBuilder::new(
        "Lonely Star",
        SCREEN_WIDTH as i32 * 2,
        SCREEN_HEIGHT as i32 * 2,
    )
    .resizable(true)
    .quit_on_escape(true)
    .build()?
    .run(GameState::new)
}

fn report_error(msg: &str) {
    #[cfg(debug_assertions)]
    {
        println!("{}", msg);
    }

    #[cfg(not(debug_assertions))]
    {
        use std::fs::File;
        use std::io::Write;

        let mut crash_log = File::create("./crash_log.txt").unwrap();

        write!(
            crash_log,
            "Oh no! Lonely Star has crashed. Here's the error message:\n\n\
            {}\n\n\
            If you don't know how to resolve this, please provide a bug report at https://17cupsofcoffee.itch.io/lonely-star.",
            msg
        )
        .unwrap();
    }
}
