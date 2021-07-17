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
        .action(|_| println!("Welcome to pmrustcli"));

    let app = init_commands(app);

    app.run(args);
}

fn get_api() -> PMRest {
    let conf = AppConfig::load();
    let rest = PMRest::new(&conf.token);
    rest
}

fn init_commands(base_app: App) -> App {
    let mut commands: Vec<Command> = vec![
     Command::new("token")
        .description("set session token")
        .alias("t")
        .usage("cli token(t) [token]")
        .action(token_action),

     Command::new("me")
        .description("whoami")
        .usage("cli me")
        .action(me_action),
        
     Command::new("item")
        .description("Get an item")
        .usage("cli item [id]")
        .action(item_action),

     Command::new("search")
        .description("Search items by text")
        .usage("cli search \"text\"")
        .action(search_action),

     Command::new("alerts")
        .description("Displays items that need your attention")
        .usage("cli alerts")
        .action(alerts_action),
    ];

    let mut app = base_app;
    while let Some(c) = commands.pop() {
        app = app.command(c);
    }
    return app;
}

fn token_action(c: &Context) {
    let mut conf = AppConfig::load();
    let token = c.args.get(0).expect("Missing token").to_string();
    conf.token = token;
    conf.save();
    println!("Token saved");
} 

fn me_action(_c: &Context) {
    let rest = get_api();
    let me = block_on(rest.get_me());
    let me = me.expect("Cannot decode");
    println!("{}", &me);
} 

fn item_action(c: &Context) {
    let id: u64 = c.args.get(0).expect("Missing id").parse().expect("Should be an integer");
    let rest = get_api();
    let item = block_on(rest.get_item(id));
    let item = item.expect("Error getting item");
    item.detailed_print();
} 

fn search_action(c: &Context) {
    let text: String = c.args.get(0).expect("Missing text").parse().expect("Should be an integer");
    let rest = get_api();
    let result = block_on(rest.search(&text));
    let result = result.expect("Response is ready");
    for i in &result.objects {
        println!("{}", i);
    }
} 

fn alerts_action(_c: &Context) {
    let rest = get_api();
    let result = block_on(rest.get_alerts());
    let result = result.expect("Response is ready");
    for i in &result.objects {
        println!("{}", i);
    }
} 
