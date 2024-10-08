use frand_home_common::{
    state::{app_state::{AppState, AppStateMessage, AppStateProperty}, socket_state::SocketStateMessage}, 
    State, StateProperty,
};
use yew::{html, Component, Context, Html};
use crate::{
    socket::client_socket::ClientSocket, 
    view::music::{
        lyrics::Lyrics, music_queue::MusicQueue, musiclist::Musiclist, playlist::Playlist, 
        server_player::ServerPlayer, task_bar::TaskBar, youtube_player::YoutubePlayer,
    },
};

pub struct App {
    socket: ClientSocket,
    state: AppState,
    prop: AppStateProperty,
}

impl Component for App {
    type Message = AppStateMessage;
    type Properties = AppStateProperty;

    fn create(context: &Context<Self>) -> Self {
        Self {
            socket: ClientSocket::new(context),
            state: AppState::default(),
            prop: <Self::Properties as StateProperty>::new(vec![], Some(context)),
        }
    }

    fn view(&self, _context: &Context<Self>) -> Html {     
        html! {
            <div>
                <TaskBar 
                state = { 
                    self.prop.send.client.music.task_bar.applied(
                        &self.state.receive.client.music.task_bar
                    )
                }
                playlist_visible = {
                    self.prop.send.client.music.playlist.visible.applied(
                        self.state.receive.client.music.playlist.visible
                    )
                }
                />
                <div style="display:flex; flex-direction: row;">
                    <Playlist 
                    state = { 
                        self.prop.send.client.music.playlist.applied(
                            &self.state.receive.client.music.playlist
                        )
                    }
                    musiclist_playlist_id = {                    
                        self.prop.send.client.music.musiclist.playlist_id.applied(
                            self.state.receive.client.music.musiclist.playlist_id.clone()
                        )
                    }
                    />
                    <div>
                        <YoutubePlayer
                        state = { 
                            self.prop.send.client.music.youtube_player.applied(
                                &self.state.receive.client.music.youtube_player
                            )
                        }
                        />
                        <Lyrics
                        state = { 
                            self.prop.send.client.music.lyrics.applied(
                                &self.state.receive.client.music.lyrics
                            )
                        }
                        />
                    </div>
                    <div>
                        <ServerPlayer
                        state = { 
                            self.prop.send.server.music.server_player.applied(
                                &self.state.receive.server.music.server_player
                            )
                        }
                        />        
                        <MusicQueue
                        state = { 
                            self.prop.send.server.music.music_queue.applied(
                                &self.state.receive.server.music.music_queue
                            )
                        }
                        />    
                        <Musiclist
                        state = { 
                            self.prop.send.client.music.musiclist.applied(
                                &self.state.receive.client.music.musiclist
                            )
                        }
                        youtube_player_video_id = {                    
                            self.prop.send.client.music.youtube_player.video_id.applied(
                                self.state.receive.client.music.youtube_player.video_id.clone()
                            )
                        }
                        />          
                    </div>
                </div>      
            </div>
        }
    }

    fn update(&mut self, _context: &Context<Self>, message: Self::Message) -> bool {   
        match message {       
            Self::Message::Error(err) => {
                log::error!("{err}");          
            },
            Self::Message::State(app_state) => {
                self.state = app_state;                         
            },
            Self::Message::Send(socket_message) => self.socket.send(socket_message),
            Self::Message::Receive(socket_message) => {
                match socket_message {
                    SocketStateMessage::State(socket_state) => {
                        self.state.receive = socket_state;                    
                    },
                    SocketStateMessage::Server(server_state_message) => {
                        self.state.receive.server.apply(server_state_message);
                    },
                    SocketStateMessage::Client(client_state_message) => {
                        self.state.receive.client.apply(client_state_message);                 
                    },
                    SocketStateMessage::Request(_) => {/*todo!()*/},
                    SocketStateMessage::Opened(_) => {/*todo!()*/},
                    SocketStateMessage::Closed(_) => {/*todo!()*/},
                    SocketStateMessage::Error(_) => {/*todo!()*/},
                }
            },
        }
        true
    }
}
