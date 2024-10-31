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


## 구성
* backend: _image/source/actix_
    - OAuth, TLS, SSL, 세션 연결
* frontend: _image/source/yew_
    - HTTP 페이지, 웹소켓 연결
* 웹앱 컨텐츠: _image/source/app_
    - 상태 관리, 페이지 렌더링, 메시지 핸들링
* 유틸리티: _image/source/state_
    - Property 를 생성하는 매크로와 관련된 Trait
* 샘플: _sample_
    - docker container 를 구동하기 위한 샘플


## 기능
- derive macro 를 이용한 wss 통신용 State/Property/Message 생성
- wss 프로토콜을 통한 server-client 간 상태 관리 통신
- Google OAuth 로그인
- Docker Image 생성
- docker-compose MySQL 접속
- Youtube data api 를 이용한 플레이리스트 보기


## 예시
_image/source/app/music_ 폴더는 템플릿 샘플로 사용될 수 있습니다. 
_src/backend/component.rs_ 의 __Music__ 구조체에는 Music 과 관련하여 backend에서 처리해야 할 일들이 통합되어 있습니다.
_image/source/app/src/backend/component.rs_ 의 __App__ 구조체에서 각 작업을 __Music__ 에 분배합니다.

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
pub fn view(
    server_prop: &<ServerState as State>::Property,
    client_prop: &<ClientState as State>::Property,
) -> Html {
    html! {
        <div style="display:flex; flex-direction: row;">
            <Playlist 
                visible = { client_prop.playlist_visible.clone() }
                list_items = { server_prop.playlist.list_items.clone() }
                musiclist_playlist_id = { client_prop.musiclist.playlist_page.playlist_id.clone() }
            />
            <div>
                <YoutubePlayer
                    video_id = { client_prop.youtube_player.video_id.clone() }
                />
                <Lyrics/>
            </div>
            <div>
                <ServerPlayer/>        
                <MusicQueue/>    
                <Musiclist
                    musiclist = { client_prop.musiclist.clone() }
                    youtube_player_video_id = { client_prop.youtube_player.video_id.clone() }
                />          
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
__handle_client_message__ 에서는 메시지를 보낸 클라이언트의 __ClientState__ 에 접근할 수 있고 
해당 클라이언트에 결과 메시지를 보낼 수 있습니다.
```rust
pub async fn handle_client_message<Msg: frand_home_node::StateMessage>(
    &self,
    sender: &UnboundedSender<Msg>,
    prop: &mut <ClientState as State>::Property,
    message: <ClientState as State>::Message,
) -> anyhow::Result<()> {
    Ok(match message {
        // Musiclist PlaylistPage 패턴의 메시지가 서버에 수신되었을 때 추가 처리:
        // client 로부터 playlistPage의 변경을 요청하는 메시지가 수신되었을 때
        // playlistPage 에 해당하는 playlist_items 를 youtube api 로 얻어서
        // playlist_items 를 넣는 동작에 해당하는 메시지를 클라이언트에 전송
        ClientStateMessage::Musiclist(
            MusiclistStateMessage::PlaylistPage(_)        
        ) => {
            // youtube data api 를 이용하여 playlist 데이터를 얻음
            let playlist_items = PlaylistItems::youtube_get(
                self,
                &prop.musiclist.playlist_page.clone_state(),
            ).await?;

            // musiclist.list_items 에 playlist_items 값을 넣고
            // 위 동작에 해당하는 message 를 생성
            let message = prop.musiclist.list_items.apply_export(
                playlist_items.into(),
            );          
                            
            // 클라이언트에 playlist_items 값을 넣는 메시지를 전송
            sender.send(message)?;  
        },
        _ => {},
    })
}   
```

__handle_server_message__ 에서는 __ServerState__ 에 접근할 수 있고 
모든 클라이언트에 결과 메시지를 보낼 수 있습니다.