mod appconfig;
mod pm;

use seahorse::{App, Context, Command};
use std::env;
use appconfig::AppConfig;
use pm::rest::{PMRest};
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(|_| println!("Welcome to pmrustcli"))
        .command(me_command())
        .command(item_command())
        .command(token_command());

    app.run(args);
}

fn token_command() -> Command {
     Command::new("token")
        .description("set session token")
        .alias("t")
        .usage("cli token(t) [token]")
        .action(token_action)
}

fn token_action(c: &Context) {
    let mut conf = AppConfig::load();
    let token = c.args.get(0).expect("Missing token").to_string();
    conf.token = token;
    conf.save();
    println!("Token saved");
} 

fn me_command() -> Command {
     Command::new("me")
        .description("whoami")
        .usage("cli me")
        .action(me_action)
}

fn me_action(_c: &Context) {
    let conf = AppConfig::load();
    let rest = PMRest::new(&conf.token);
    let me = block_on(rest.get_me());
    let me = me.expect("Cannot decode");
    println!("{}", &me);
} 

fn item_command() -> Command {
     Command::new("item")
        .description("Get an item")
        .usage("cli item [id]")
        .action(item_action)
}

fn item_action(c: &Context) {
    let conf = AppConfig::load();
    let id: u64 = c.args.get(0).expect("Missing id").parse().expect("Should be an integer");
    let rest = PMRest::new(&conf.token);
    let item = block_on(rest.get_item(id));
    let item = item.expect("Error getting item");
    println!("{}", &item);
} 