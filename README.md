# FrandHome
Actix 와 Yew 를 이용하여 Rust 로 개발하는 풀스택 웹 프로젝트


## 목적
- 간결하고 쉽고 직관적인 기능 코드는 유지보수와 추가 기능 개발을 쉽게 합니다.
이 프로젝트는 Rust Macro 를 활용한 Property 로 그러한 환경을 구현하는 것을 목적으로 합니다.


## 구조
- proc macro __PropertyState__ 를 derive 한 구조체를 기술하면
그에 맞는 구조의 __StateProperty__ 와 __StateMessage__ 가 생성됩니다.
__StateProperty__ 는 클라이언트에서 Yew Component 로 전달되어 이벤트를 처리하는 View Model 로 사용되거나
서버에서 서버와 클라이언트의 상태를 보관하고 클라이언트로 보낼 __StateMessage__ 를 생성하는 역할을 합니다.
- Yew Component 에 __StateProperty__ 를 전달하고 value() 로 값을 꺼내고 emit() 으로 값 변경 메시지를 보낼 수 있습니다.
- handle_message() 에서 __StateMessage__ 를 받아 서버의 상태를 변경하고 클라이언트에 메시지를 보낼 수 있습니다.


## 기능
- derive macro 를 이용한 wss 통신용 State/Property/Message 생성
- wss 프로토콜을 통한 server-client 간 상태 관리 통신
- Google OAuth 로그인
- Docker Image 생성
- docker-compose MySQL 접속
- Youtube data api 를 이용한 플레이리스트 보기


## 예시
State: 
```rust
// MusiclistStateProperty, MusiclistStateMessage 자동 생성
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct MusiclistState {
    pub playlist_page: PlaylistPageState,
    pub list_items: MusiclistItemsState,
}

// MusiclistItemsStateProperty, MusiclistItemsStateMessage 자동 생성
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct MusiclistItemsState {
    pub next_page_token: Option<String>,
    pub prev_page_token: Option<String>,
    pub total_results: usize,
    pub results_per_page: usize,
    pub items: Vec<MusiclistItemState>,    
}

// MusiclistItemStateProperty, MusiclistItemStateMessage 자동 생성
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, PropertyState)]
pub struct MusiclistItemState {
    pub video_id: String,
    pub title: String,
}
```
Layout:
```rust
pub fn view(prop: &AppProperty) -> Html {     
    html! {
        <div>
            <TaskBar
                user = { prop.socket.client.user.clone() }
                task_bar = { prop.socket.client.task_bar.clone() }
            />     
            <div style="display:flex; flex-direction: row;">
                <Playlist 
                    visible = { prop.socket.client.task_bar.playlist_visible.clone() }
                    list_items = { prop.socket.server.music.playlist.list_items.clone() }
                    musiclist_playlist_id = { prop.socket.client.music.musiclist.playlist_page.playlist_id.clone() }
                />
                <div>
                    <YoutubePlayer
                        video_id = { prop.socket.client.music.youtube_player.video_id.clone() }
                    />
                    <Lyrics/>
                </div>
                <div>
                    <ServerPlayer/>        
                    <MusicQueue/>    
                    <Musiclist
                        musiclist = { prop.socket.client.music.musiclist.clone() }
                        youtube_player_video_id = { prop.socket.client.music.youtube_player.video_id.clone() }
                    />          
                </div>
            </div>
        </div>
    }
}
```
View: 
```rust
#[derive(Properties, PartialEq)]
pub struct MusiclistProperty {
    pub musiclist: <MusiclistState as State>::Property,
    pub youtube_player_video_id: Node<String>,
}

#[function_component]
pub fn Musiclist(prop: &MusiclistProperty) -> Html {    
    let pages = {
        let page_token = prop.musiclist.playlist_page.page_token.clone();
        let prev_page_token = prop.musiclist.list_items.prev_page_token.value().clone();
        let prev_page_disabled = prev_page_token.is_none();

        // {"prev"} 버튼이 클릭되었을 때 page_token Property 의 emit 을 호출하여 서버에 prev_page_token 을 전송
        let onclick_prev_page = move |_| {
            page_token.emit(prev_page_token.clone());
        };

        let page_token = prop.musiclist.playlist_page.page_token.clone();
        let next_page_token = prop.musiclist.list_items.next_page_token.value().clone();
        let next_page_disabled = next_page_token.is_none();

        // {"next"} 버튼이 클릭되었을 때 page_token Property 의 emit 을 호출하여 서버에 next_page_token 을 전송
        let onclick_next_page = move |_| {
            page_token.emit(next_page_token.clone());
        };

        html! {
            <div>
                <button disabled={prev_page_disabled} onclick={onclick_prev_page}>
                {"prev"}
                </button>
                <button disabled={next_page_disabled} onclick={onclick_next_page}>
                {"next"}
                </button>
            </div>
        }
    };

    let items: Vec<_> = prop.musiclist.list_items.items.items().clone().into_iter()
    .map(|item| {
        let youtube_player_video_id = prop.youtube_player_video_id.clone();
        let title = item.title.clone();
        let video_id = item.video_id.clone();

        // {title} 버튼이 클릭되었을 때 youtube_player_video_id Property 의 emit 을 호출하여 서버에 video_id 를 전송
        let onclick_music = move |_| {
            youtube_player_video_id.emit(video_id.clone())
        };

        html! {
            <button onclick={onclick_music}>
            {title}
            </button>
        }
    }).collect(); 

    html! {
        <div style="display:flex; flex-direction: column;">
            <p>{"Musiclist"}</p>
            {pages}
            {items}
        </div>
    }
}
```
Control:
```rust
pub async fn handle_client_message(
    client: &Client,
    sender: &UnboundedSender<SocketStateMessage>,
    client_state: &mut ClientStateProperty,
    message: ClientStateMessage,
) -> anyhow::Result<()> {        
    match &message {
        // Client Music Musiclist PlaylistPage 패턴의 메시지가 서버에 수신되었을 때 추가 처리:
        // client 로부터 playlistPage의 변경을 요청하는 메시지가 수신되었을 때
        // playlistPage 에 해당하는 playlist_items 를 youtube api 로 얻어서
        // playlist_items 를 넣는 동작에 해당하는 메시지를 클라이언트에 전송
        ClientStateMessage::Music(
            ClientMusicStateMessage::Musiclist(
                MusiclistStateMessage::PlaylistPage(_)
            )
        ) => {
            // client 로부터 수신된 메시지를 client 로 전송
            // client 의 view 에서 emit 된 메시지를 서버가 되돌려 주어야 client의 state 가 변경됨
            client_state.apply_message(message.clone());
            sender.send(SocketStateMessage::Client(message))?;  

            // youtube data api 를 이용하여 playlist 데이터를 얻음
            let playlist_items = PlaylistItems::youtube_get(
                client, 
                &client_state.music.musiclist.playlist_page.clone_state(),
            ).await?;

            // client_state.music.musiclist.list_items.state 에 playlist_items 값을 넣고
            // 위 동작에 해당하는 message 를 생성
            let message = client_state.music.musiclist.list_items.apply_export(
                playlist_items.into(),
            );          
                         
            // 클라이언트에 playlist_items 값을 넣는 메시지를 전송
            sender.send(message)?;  
        },
        _ => {
            // client 로부터 수신된 메시지를 client 로 전송
            // client 의 view 에서 emit 된 메시지를 서버가 되돌려 주어야 client의 state 가 변경됨
            client_state.apply_message(message.clone());
            sender.send(SocketStateMessage::Client(message))?;  
        },
    }

    Ok(())
}
```
