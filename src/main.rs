use exitfailure::ExitFailure;

mod config;

fn main() -> Result<(), ExitFailure> {
    let init_config = config::init()?;
    println!("{:?}", init_config);

    Ok(())
}
