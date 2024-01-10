use crate::config::Containeropts;

fn child(config: Containeropts) -> isize{
   log::info!(" Starting with command {} and arg {:?}", config.path.to_str().unwrap(), config.argv);
    0
}