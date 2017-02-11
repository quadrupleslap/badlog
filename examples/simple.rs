#[macro_use] extern crate log;
extern crate badlog;

fn main() {
    badlog::init_from_env("LOG_LEVEL");

    trace!("Unimportant details.");
    debug!("Debugging information.");
    info!("Hello, World!");

    surgery::complicated_procedure();
    
    warn!("Uh, captain, I think we're going down...");
    error!("IT'S THE END.");
}

mod surgery {
    pub fn complicated_procedure() {
        trace!("Started complicated procedure!");
        warn!("Missing scalpel, substituting with cleaver.");
        trace!("Complicated procedure complete!");
    }
}
