use self::dcc_type::DccType;

pub mod dcc_type;

const DCC: &str = "DCC";

pub struct DccMessage {
    pub type_: DccType,
    // pub argument: String,
    pub address: String,
    pub metadata: Option<String>,
}

impl DccMessage {
    pub fn parse(message: String) -> Result<Self, String> {
        let mut arguments: Vec<String> = message.split(' ').map(|s| s.to_string()).collect();

        // if arguments.len() < 5 {
        //     return Err("Not enough arguments".to_string());
        // }

        let first = arguments.remove(0);

        // if first != DCC {
        //     return Err("Must start with DCC".to_string());
        // }

        // esto se va a romper cuando sea DCC SEND porque tiene como último parámetro el tamaño del archivo

        let type_ = arguments.remove(0);
        // let argument = arguments.remove(0);

        let ip = arguments.remove(0);
        let port = arguments.remove(0);

        let metadata = arguments.pop();

        let address = format!("{ip}:{port}");

        let type_ = DccType::from(&*type_);

        Ok(Self {
            type_,
            // argument,
            address,
            metadata,
        })
    }
}
