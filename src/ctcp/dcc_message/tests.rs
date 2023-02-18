use super::*;

#[test]
fn chat_request_is_parsed_correctly() {
    let raw_message = "DCC CHAT chat 127.0.0.1 9000".to_string();

    let dcc_message = DccMessage::parse(raw_message).unwrap();

    matches!(dcc_message, DccMessage::Chat { .. });

    if let DccMessage::Chat { address } = dcc_message {
        assert_eq!("127.0.0.1:9000", address.to_string());
    }
}

#[test]
fn chat_accept_is_parsed_correctly() {
    let raw_message = "DCC CHAT accept".to_string();

    let dcc_message = DccMessage::parse(raw_message).unwrap();

    matches!(dcc_message, DccMessage::ChatAccept);
}

#[test]
fn chat_decline_is_parsed_correctly() {
    let raw_message = "DCC CHAT decline".to_string();

    let dcc_message = DccMessage::parse(raw_message).unwrap();

    matches!(dcc_message, DccMessage::ChatDecline);
}

#[test]
fn close_is_parsed_correctly() {
    let raw_message = "DCC CLOSE".to_string();

    let dcc_message = DccMessage::parse(raw_message).unwrap();

    matches!(dcc_message, DccMessage::Close);
}

#[test]
fn send_is_parsed_correctly() {
    let raw_message = "DCC SEND filename 127.0.0.1 9000 4096".to_string();

    let dcc_message = DccMessage::parse(raw_message).unwrap();

    matches!(dcc_message, DccMessage::Send { .. });

    if let DccMessage::Send {
        filename,
        address,
        filesize,
    } = dcc_message
    {
        assert_eq!("filename", filename);
        assert_eq!("127.0.0.1:9000", address.to_string());
        assert_eq!(4096, filesize);
    }
}

#[test]
fn send_accept_is_parsed_correctly() {
    let raw_message = "DCC SEND accept".to_string();

    let dcc_message = DccMessage::parse(raw_message).unwrap();

    matches!(dcc_message, DccMessage::SendAccept);
}

#[test]
fn send_decline_is_parsed_correctly() {
    let raw_message = "DCC SEND decline".to_string();

    let dcc_message = DccMessage::parse(raw_message).unwrap();

    matches!(dcc_message, DccMessage::SendDecline);
}

#[test]
fn resume_is_parsed_correctly() {
    let raw_message = "DCC RESUME filename 9000 256".to_string();

    let dcc_message = DccMessage::parse(raw_message).unwrap();

    matches!(dcc_message, DccMessage::Resume { .. });

    if let DccMessage::Resume {
        filename,
        port,
        position,
    } = dcc_message
    {
        assert_eq!("filename", filename);
        assert_eq!(9000, port);
        assert_eq!(256, position);
    }
}

#[test]
fn accept_is_parsed_correctly() {
    let raw_message = "DCC ACCEPT filename 9000 256".to_string();

    let dcc_message = DccMessage::parse(raw_message).unwrap();

    matches!(dcc_message, DccMessage::Accept { .. });

    if let DccMessage::Accept {
        filename,
        port,
        position,
    } = dcc_message
    {
        assert_eq!("filename", filename);
        assert_eq!(9000, port);
        assert_eq!(256, position);
    }
}
