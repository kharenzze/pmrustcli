mod appconfig;
use appconfig::AppConfig;

fn main() {
    println!("Hello, world!");
    let mut conf = AppConfig::load();
    println!("{}", &conf.token);
    conf.token = "settingToken".to_string();
    conf.save();

    println!("{}", &conf.token);
}
