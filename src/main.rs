mod app_options;
mod ignore_file;
mod logger;
mod my_app;
mod ruleset;
mod type_counter;

use anyhow::Result;

fn main() -> Result<()> {
    let app = my_app::MyApp::new()?;
    let count = app.run()?;

    Ok(println!("{}", count))
}
