use cdn_dns::errors::failed_config_read;
use cdn_dns::settings::config::get_config;

#[test]
fn config_test() {
    let config = get_config().expect(failed_config_read());
    println!("{:#?}", config);
}
