use self::dcc_type::DccType;

pub mod dcc_type;

const DCC: &str = "DCC";

pub struct DccMessage {
    pub type_: DccType,
    pub argument: String,
    pub ip: String,
    pub port: String,
}

impl DccMessage {
    pub fn parse(message: String) -> Result<Self, String> {
        let mut arguments: Vec<&str> = message.split(' ').collect();

        if arguments.len() < 5 {
            return Err("Not enough arguments".to_string());
        }

        let first = arguments.remove(0);

        if first != DCC {
            return Err("Must start with DCC".to_string());
        }

        let port = arguments.pop().unwrap().to_string();
        let ip = arguments.pop().unwrap().to_string();
        let argument = arguments.pop().unwrap().to_string();
        let type_ = arguments.pop().unwrap();

        let type_ = DccType::from(type_);

        Ok(Self {
            type_,
            argument,
            ip,
            port,
        })
    }
}
