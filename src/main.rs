mod my_app;

use anyhow::Result;
use my_app::MyApp;

fn main() -> Result<()> {
    let app = MyApp::new()?;
    let count = app.run()?;

    Ok(println!("\n{}", count))
}
