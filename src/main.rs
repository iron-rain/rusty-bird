#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Bird")
        .build()?;
    
    main_loop(context, State::new())
}

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::End;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcom to Rusty Bird");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are deadish!");
        ctx.print_centered(8, "(P) Be Reborn");
        ctx.print_centered(9, "(Q) Give Up");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

