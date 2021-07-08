mod appconfig;
use appconfig::AppConfig;

fn main() {
    println!("Hello, world!");
    let conf = AppConfig::load();

    println!("{}", &conf.token);
}
