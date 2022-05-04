use crate::Commands;

#[test]
fn test_parse_commands() {
    assert_eq!(Commands::parse("w"), Commands::Save);
    assert_eq!(Commands::parse("wq"), Commands::SaveAnQuit);
    assert_eq!(Commands::parse("ln"), Commands::LineNumbers);
    assert_eq!(Commands::parse("help"), Commands::Help);
    assert_eq!(Commands::parse("stats"), Commands::Stats);
    assert_eq!(Commands::parse("wq!"), Commands::ForceQuit);
    assert_eq!(Commands::parse("debug"), Commands::Debug);
    assert_eq!(Commands::parse("o"), Commands::OpenShort);
    assert_eq!(Commands::parse("open"), Commands::Open);
    assert_eq!(Commands::parse("new"), Commands::New);
    assert_eq!(Commands::parse("q"), Commands::Quit);
    assert_eq!(Commands::parse("DERP"), Commands::Unknown);
}

#[test]
fn test_commands_as_str() {
    assert_eq!(Commands::Save.as_str(), "w");
    assert_eq!(Commands::SaveAnQuit.as_str(), "wq");
    assert_eq!(Commands::LineNumbers.as_str(), "ln");
    assert_eq!(Commands::Help.as_str(), "help");
    assert_eq!(Commands::Stats.as_str(), "stats");
    assert_eq!(Commands::ForceQuit.as_str(), "wq!");
    assert_eq!(Commands::Debug.as_str(), "debug");
    assert_eq!(Commands::OpenShort.as_str(), "o");
    assert_eq!(Commands::Open.as_str(), "open");
    assert_eq!(Commands::New.as_str(), "new");
    assert_eq!(Commands::Quit.as_str(), "q");
    assert_eq!(Commands::Unknown.as_str(), "");
}
