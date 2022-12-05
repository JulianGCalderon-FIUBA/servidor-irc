use super::*;

#[test]
fn display_only_command() {
    let message = Message {
        prefix: None,
        command: "COMMAND".to_string(),
        parameters: vec![],
        trailing: None,
    };

    let actual = message.to_string();
    let expected = "COMMAND";

    assert_eq!(&actual, expected);
}

#[test]
fn display_with_prefix() {
    let message = Message {
        prefix: Some("prefix".to_string()),
        command: "COMMAND".to_string(),
        parameters: vec![],
        trailing: None,
    };

    let actual = message.to_string();
    let expected = ":prefix COMMAND";

    assert_eq!(&actual, expected);
}

#[test]
fn display_with_one_parameter() {
    let message = Message {
        prefix: None,
        command: "COMMAND".to_string(),
        parameters: vec!["param1".to_string()],
        trailing: None,
    };

    let actual = message.to_string();
    let expected = "COMMAND param1";

    assert_eq!(&actual, expected);
}

#[test]
fn display_with_two_parameters() {
    let message = Message {
        prefix: None,
        command: "COMMAND".to_string(),
        parameters: vec!["param1".to_string(), "param2".to_string()],
        trailing: None,
    };

    let actual = message.to_string();
    let expected = "COMMAND param1 param2";

    assert_eq!(&actual, expected);
}

#[test]
fn display_with_trailing() {
    let message = Message {
        prefix: None,
        command: "COMMAND".to_string(),
        parameters: vec![],
        trailing: Some("trailing".to_string()),
    };

    let actual = message.to_string();
    let expected = "COMMAND :trailing";

    assert_eq!(&actual, expected);
}

#[test]
fn display_with_trailing_with_spaces() {
    let message = Message {
        prefix: None,
        command: "COMMAND".to_string(),
        parameters: vec![],
        trailing: Some("trailing with spaces".to_string()),
    };

    let actual = message.to_string();
    let expected = "COMMAND :trailing with spaces";

    assert_eq!(&actual, expected);
}

#[test]
fn display_full_message() {
    let message = Message {
        prefix: Some("prefix".to_string()),
        command: "COMMAND".to_string(),
        parameters: vec!["param1".to_string(), "param2".to_string()],
        trailing: Some("trailing".to_string()),
    };

    let actual = message.to_string();
    let expected = ":prefix COMMAND param1 param2 :trailing";

    assert_eq!(&actual, expected);
}

#[test]
fn parsing_only_command() {
    let message = Message::new("COMMAND").unwrap();

    assert_eq!(None, message.prefix);
    assert_eq!("COMMAND", &message.command);
    assert_eq!(Vec::<String>::new(), message.parameters);
    assert_eq!(None, message.trailing);
}

#[test]
fn parsing_w_prefix() {
    let message = Message::new(":prefix COMMAND").unwrap();

    assert_eq!(Some("prefix".to_string()), message.prefix);
    assert_eq!("COMMAND", &message.command);
    assert_eq!(Vec::<String>::new(), message.parameters);
    assert_eq!(None, message.trailing);
}

#[test]
fn parsing_w_one_parameter() {
    let message = Message::new("COMMAND param1").unwrap();

    assert_eq!(None, message.prefix);
    assert_eq!("COMMAND", &message.command);
    assert_eq!(vec!["param1".to_string()], message.parameters);
    assert_eq!(None, message.trailing);
}

#[test]
fn parsing_w_two_parameters() {
    let message = Message::new("COMMAND param1 param2").unwrap();

    assert_eq!(None, message.prefix);
    assert_eq!("COMMAND", &message.command);
    assert_eq!(
        vec!["param1".to_string(), "param2".to_string()],
        message.parameters
    );
    assert_eq!(None, message.trailing);
}

#[test]
fn parsing_w_trailing() {
    let message = Message::new("COMMAND :trailing").unwrap();

    assert_eq!(None, message.prefix);
    assert_eq!("COMMAND", &message.command);
    assert_eq!(Vec::<String>::new(), message.parameters);
    assert_eq!(Some("trailing".to_string()), message.trailing);
}

#[test]
fn parsing_w_trailing_w_spaces() {
    let message = Message::new("COMMAND :trailing with spaces").unwrap();

    assert_eq!(None, message.prefix);
    assert_eq!("COMMAND", &message.command);
    assert_eq!(Vec::<String>::new(), message.parameters);
    assert_eq!(Some("trailing with spaces".to_string()), message.trailing);
}

#[test]
fn parsing_full_message() {
    let message = Message::new(":prefix COMMAND param1 param2 :trailing with spaces").unwrap();

    assert_eq!(Some("prefix".to_string()), message.prefix);
    assert_eq!("COMMAND", &message.command);
    assert_eq!(
        vec!["param1".to_string(), "param2".to_string()],
        message.parameters
    );
    assert_eq!(Some("trailing with spaces".to_string()), message.trailing);
}
