use crate::Config;

#[test]
fn test_config_toggle() {
    let mut conf = Config::default();
    assert!(!conf.display_stats);
    conf.display_stats = Config::toggle(conf.display_stats);
    assert!(conf.display_stats);
}
