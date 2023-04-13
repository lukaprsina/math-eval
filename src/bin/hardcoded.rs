use std::rc::Rc;

use color_eyre::eyre::Result;
use itertools::Itertools;
use math_eval::ast::{app::App, context::Context};
use once_cell::sync::Lazy;
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

#[allow(dead_code, unused_variables)]
fn main() -> Result<()> {
    color_eyre::install()?;
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = App::new()?;

    let context = Context::new(Rc::clone(&app));

    let ctx_uuid = app.borrow_mut().add_context(context);

    let mut uuids: Vec<Uuid> = vec![];

    for equation in EQUATIONS.iter() {
        let uuid = App::try_add_equation(Rc::clone(&app), ctx_uuid, equation.as_str())?;
        uuids.push(uuid);
        let mut borrowed_app = app.borrow_mut();
        let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();
    }

    {
        let mut borrowed_app = app.borrow_mut();
        let context = borrowed_app.get_context_mut(ctx_uuid).unwrap();

        context.solve();
    }

    for (pos, uuid) in uuids.into_iter().enumerate() {
        {
            let mut borrowed_app = app.borrow_mut();
            let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();
            let eq = ctx.get_equation_mut(uuid).unwrap();

            for expr in &mut eq.equation_sides {
                expr.flatten()
            }

            ctx.solve();
        }

        // debug!("{expr:#?}");
        // debug!("{expr}");
        // debug!("{}", EQUATIONS[pos]);
        // debug!("{expr:#?}");
        // debug!("{expr}");
        // println!("\n\n");

        {
            let mut borrowed_app = app.borrow_mut();

            let mut simplify = {
                borrowed_app
                    .strategies
                    .remove("solve_one_variable")
                    .unwrap()
            };

            let ctx = borrowed_app.get_context_mut(ctx_uuid).unwrap();
            let eq = ctx.get_equation_mut(uuid).unwrap();

            let func = &mut simplify.equation.as_deref_mut().unwrap();
            let mut cloned_eq = eq.clone();
            func(&mut cloned_eq, "x");
            // debug!("{cloned_eq:#?}");
            debug!("{cloned_eq}");
        }

        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
    }

    Ok(())
}

static EQUATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    let strings = vec![
        "sin(x+1)=(x^2+1+(f(x)/2))/2",
        // "a*(b+c)",
        // "-2/(-a/-8)",
        // "f(g(h, x+2))",
        // "(-1-2)-3",
        // "(-1*(-2))-3",
        "1-((-2-3)*(-4-5))/((-6-7)*(-8-9))",
        "(-1-2)*3",
        "(-1-2)*3 - 3",
        "(-1*(-2))*3 - 3",
        "(1/2)/(3/4)",
    ];
    strings
        .into_iter()
        .map(|string| string.to_string())
        .collect_vec()
});
