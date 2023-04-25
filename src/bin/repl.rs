use color_eyre::eyre::Result;
use math_eval::{
    ast::{app::App, context::Context},
    initialize,
};
use rustyline::{error::ReadlineError, DefaultEditor};
use std::rc::Rc;

// TODO: vec remove unwrap
fn main() -> Result<()> {
    initialize()?;

    let app = App::new()?;
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let context = Context::new(Rc::clone(&app));
                let ctx_uuid = app.borrow_mut().add_context(context);

                App::try_add_equation(Rc::clone(&app), ctx_uuid, line.as_str())?;
                let mut borrowed_app = app.borrow_mut();
                // let context = borrowed_app.get_context_mut(ctx_uuid).unwrap();
                App::solve(&mut borrowed_app, ctx_uuid);
                // borrowed_app.solve(ctx_uuid);

                /* let eq = ctx.remove_equation(uuid).unwrap();

                println!("{}", eq);
                // println!("{:#?}", eq); */
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
