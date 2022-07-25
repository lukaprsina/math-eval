use std::rc::Rc;

use color_eyre::eyre::Result;
use math_eval::ast::{app::App, context::Context};
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    color_eyre::install()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Started the logger crate");

    let app = App::new()?;

    let context = Context::new(Rc::clone(&app));

    let ctx_uuid = app.borrow_mut().add_context(context);

    App::try_add_equation(Rc::clone(&app), ctx_uuid, "4 + 4x + x^2 + 5")?;

    let mut borrowed_app = app.borrow_mut();
    let context = borrowed_app.get_context_mut(ctx_uuid).unwrap();

    context.solve();

    Ok(())
}
