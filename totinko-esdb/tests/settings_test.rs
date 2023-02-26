use totinko_esdb::settings::Settings;

#[test]
fn test_settings() {
    let settings = Settings::new().unwrap();

    assert!(!settings.esdb.url.is_empty());
}
