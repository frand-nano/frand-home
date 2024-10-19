use actix_session::SessionExt;
use actix_web::{get, web::{Data, Payload}, HttpRequest, HttpResponse};
use actix_ws::{handle, Message, MessageStream, Session};
use awc::Client;
use frand_home_common::state::socket_state::SocketStateMessage;
use futures_util::StreamExt;
use tokio::{sync::mpsc::{unbounded_channel, UnboundedSender}, task::spawn_local};
use crate::{authorize::user::User, server::{ServerHandle, ServerMessage}, session::SessionUtil};

#[get("/ws/")]
pub async fn get_ws(
    request: HttpRequest, 
    stream: Payload,
    server_sender: Data<UnboundedSender<ServerMessage>>,
    client: Data<Client>,
) -> actix_web::Result<HttpResponse> {
    let session = request.get_session();
    if !session.client_whitelist() {
        return Ok(super::get_login(request).await);
    }
    
    let user = session.user();
    
    let (response, session, stream) = handle(&request, stream)?;

    spawn_message_loop(&client, user, (**server_sender).clone(), session, stream).await;

    Ok(response)
}

async fn spawn_message_loop(
    client: &Client,
    user: User, 
    server_sender: UnboundedSender<ServerMessage>,
    session: Session, 
    mut stream: MessageStream,
) {
    log::info!("{user} 🔗 {}", user.additional_info_text());

    let (client_sender, mut client_receiver) = unbounded_channel();
    let server_handle = match ServerHandle::new(client, user.clone(), server_sender, client_sender).await {
        Ok(server_handle) => server_handle,
        Err(err) => return log::error!("❗ {user} 🔗 ServerHandle::new err: {err}"),
    };

    let user_clone = user.clone();
    let mut session_clone = session.clone();
    spawn_local(async move {
        while let Some(message) = client_receiver.recv().await { 
            let message = serde_json::to_string_pretty(&message);
            match message {
                Ok(message) => {
                    if let Err(err) = session_clone.text(message).await {                        
                        log::info!("{user_clone} 🔗 Closed({err})");
                    }                    
                },
                Err(err) => log::error!("❗ {user_clone} 🔗 Message Serialize err: {err}"),
            }
        }
    });    

    let user_clone = user.clone();
    let mut session_clone = session.clone();
    spawn_local(async move {
        while let Some(message) = stream.next().await { 
            match message {
                Ok(Message::Text(json)) => {
                    match SocketStateMessage::try_from(&json) {
                        Ok(message) => {
                            if let Err(err) = server_handle.send(message) {
                                log::error!("❗ {user_clone} 🔗 Send Message err: {err}");
                            }
                        },
                        Err(err) => log::error!("❗ {user_clone} 🔗 Message Deserialize err: {err}"),
                    }             
                },
                Ok(Message::Ping(message)) => {
                    session_clone.pong(&message).await.ok();
                },
                Ok(Message::Close(None)) => {
                    log::info!("{user_clone} 🔗 Close");                    
                },
                Ok(Message::Close(Some(reason))) => {
                    let code = format!("{:#?}", reason.code);
                    let desc = reason.description.unwrap_or_default();
                    log::info!("{user_clone} 🔗 Close({code}) {desc}");
                },
                Ok(message) => log::debug!("{user_clone} 🔗 Message({:#?})", message),
                Err(err) => log::error!("❗ {user_clone} 🔗 Err({err})"),
            }
        }
    });    
}
