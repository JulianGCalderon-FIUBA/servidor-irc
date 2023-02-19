use std::{
    io::{self, Write},
    net::{TcpListener, TcpStream},
};

use crate::message::CRLF;

use super::DccChat;

struct DccChatSender {
    server: TcpStream,
    client: String,
    listener: TcpListener,
}

impl DccChatSender {
    pub fn send(mut server: TcpStream, client: String) -> io::Result<Self> {
        let listener = TcpListener::bind("0.0.0.0:0")?;

        let address = listener.local_addr()?;

        let ip = address.ip();
        let port = address.port();

        write!(server, "CTCP {client} :DCC CHAT chat {ip} {port}")?;
        server.write_all(CRLF)?;

        Ok(Self {
            server,
            client,
            listener,
        })
    }

    pub fn accept(self) -> io::Result<DccChat> {
        let stream = self.listener.accept()?.0;
        DccChat::new(stream)
    }

    pub fn close(self) {}
}

/*
** RECEPCION DE MENSAJES **
for every server message:
    if not privmsg:
        continue;

    let ctcp = get_ctcp();

    if ctcp is none:
        continue;

    let dcc_message = parse_dcc_message(ctcp);

    if dcc_chat
        let dcr = new dccChatReceiver
        preguntar al usuario si quiere establecer conexion

        si quiere:
            dcr.accept()
        si no quiere:
            dcr.decline()

    if dcc chat accept:
        obtener el cliente que lo envio
        acceder al dccChatSender perteneciente al cliente que lo envio
            (sugerencia, usar un hashmap)
        dccChatSender.receive_accept()

    if dcc chat decline:
        obtener el cliente que lo envio
        acceder al dccChatSender perteneciente al cliente que lo envio
            (sugerencia, usar un hashmap)
        dccChatSender.receive_decline()
*/

/*
** EMISION DE MENSAJES
si el cliente aprieta el boton de iniciar un dcc chat:
    let dcs = DccChatSender::issue(tcpStream del servidor, cliente);
    guardar el dcs en la estructura, con el cliente asociado (hashmap)
*/
