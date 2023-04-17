use std::rc::Rc;

use color_eyre::eyre::Result;
use itertools::Itertools;
use math_eval::{
    ast::{app::App, context::Context},
    initialize,
};
use once_cell::sync::Lazy;
use tracing::debug;
use uuid::Uuid;

fn main() -> Result<()> {
    initialize()?;

    let app = App::new()?;
    let mut contexts: Vec<Uuid> = vec![];

    for equation in EQUATIONS.iter() {
        let context = Context::new(Rc::clone(&app));
        let ctx_uuid = app.borrow_mut().add_context(context);
        // debug!("{equation}");
        App::try_add_equation(Rc::clone(&app), ctx_uuid, equation.as_str())?;
        contexts.push(ctx_uuid);
    }

    for uuid in contexts {
        let mut borrowed_app = app.borrow_mut();
        let _context = borrowed_app.get_context_mut(uuid).unwrap();

        /* for (_, equation) in &_context.equations {
            debug!("{}", equation.rpn());
        } */

        App::solve(&mut borrowed_app, uuid);

        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
    }

    Ok(())
}

static EQUATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    let strings = vec![
        "x=(1/2)",
        "sin(x)=2",
        "sin(1/x)=2",
        "sin(1/(x+f(3)))=2",
        "sin(x/2*3)=f(4)",
        "x*2=0",
        "x+1 = 0",
        "(1+0)/(3+1) = 0",
        "x*2/(3+1) = 0",
        "sin(x^3+1)=0",
        "(-1-2)*3",
        "1/(2+x)",
        "x+((y*a)-z)",
        "(1+2)*(3+4) ",
        "1+2*3+4",
        "1+2*(3+4)",
        "(1+2)*3+4",
        // "sin(x+1)=(x^2+1+(f(x)/2))/2",
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
