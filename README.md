# FrandHome
Actix 와 Yew 를 이용하여 Rust 로 개발한 개인 홈페이지
- Actix (https://actix.rs/)
- Yew (https://yew.rs/)


## 구성
- Google OAuth 로그인
- Youtube data api 를 이용한 플레이리스트 보기
- wss 프로토콜을 통한 server-client 간 상태 관리 통신
- derive macro 를 이용한 wss 통신용 State/Property/Message 생성


## 예시
State: 
```rust
use frand_home_base::PropertyState;
use serde::{Deserialize, Serialize};

use super::playlist_state::PlaylistPageState;

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
View: 
```rust
use frand_home_common::{state::client::music::musiclist_state::MusiclistState, Node, State};
use yew::{function_component, html, Html, Properties};

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
        client_state.apply_message(message.clone());
        send(&self.senders, &id, SocketStateMessage::Client(message))?;  

        // youtube data api 를 이용하여 playlist 데이터를 얻음
        let playlist_items = PlaylistItems::youtube_get(
            &self.client, 
            &client_state.music.musiclist.playlist_page.clone_state(),
        ).await?;

        // client_state.music.musiclist.list_items.state 에 playlist_items 값을 넣고
        // 위 동작에 해당하는 message 를 생성
        let message = client_state.music.musiclist.list_items.apply_export(
            playlist_items.into(),
        );          
                        
        // 클라이언트에 playlist_items 값을 넣는 메시지를 전송
        self.send(&id, message)?;
    },
    _ => {
        client_state.apply_message(message.clone());
        send(&self.senders, &id, SocketStateMessage::Client(message))?;  
    },
}
```
