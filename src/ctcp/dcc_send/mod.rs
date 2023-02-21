mod dcc_resume_sender;
pub mod dcc_send_receiver;
pub mod dcc_send_sender;
pub mod file_transfer;

/*
::: REPECIÖN DE MENSAJES :::

for every server message:
    if not privmsg:
        continue;

    let ctcp = get_ctcp();

    if ctcp is none:
        continue;

    let dcc_message = parse_dcc_message(ctcp);

    if dcc_message = dcc send

        preguntar al usuario si quiere aceptar el archivo

        new dccSendReceiver

        si quiere:
            dccSendReceiver.accept()
        si no quiere:
            dccSendReceiver.decline()

    if dcc_message = dcc send accept:
        acceder al dccSendSender perteneciente al cliente que lo envio

        dccSendSender.accept()


    if dcc_message = dcc send decline:
        acceder al dccSendSender perteneciente al cliente que lo envio

        dccSendSender.close()
*/

/*
::: EMISION DE MENSAJES :::

si el cliente aprieta el boton de enviar un archivo:

abrir widget de elección de archivo

DccSendSender::send(tcpStream del servidor, cliente, nombre del archivo);

guardar la estructura para posteriormente comenzar la transferencia
*/
