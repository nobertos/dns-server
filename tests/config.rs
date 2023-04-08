use cdn_dns::{config::get_config, errors::failed_config_read};

#[test]
fn config_test() {
    let config = get_config().expect(failed_config_read());
    println!("{:#?}", config);
}
