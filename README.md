# FrandHome
Actix 와 Yew 를 이용하여 Rust 로 개발하는 풀스택 웹 프로젝트


## 목적
- 서버와 클라이언트 사이의 소켓 통신을 이용한 웹앱을 개발하다 보면 
실수하기 쉽고 가독성 떨어지는 보일러 플레이트 코드들이 양산됩니다.
이는 여러 가지 잠재적 보안 위험과 디버깅하기 어려운 버그들을 발생시킬 수 있습니다.
이 프로젝트는 Rust Macro 를 활용한 __Node__ 로 위와 같은 문제를 줄이고
높은 생산성과 가독성을 달성하는 프로젝트 기반을 만드는 것을 목적으로 합니다.


## 구조
- proc macro __node_state__ 를 derive 한 구조체를 기술하면
그에 맞는 구조의 __Node__ 와 __Message__ 가 생성됩니다.
__Node__ 는 클라이언트에서 Yew Component 로 전달되어 이벤트를 처리하는 View Model 로 사용되거나
서버에서 서버와 클라이언트의 상태를 보관하고 클라이언트로 보낼 __Message__ 를 생성하는 역할을 합니다.
- Yew Component 에 __Node__ 를 전달하고 value() 로 값을 꺼내고 emit() 으로 값 변경 메시지를 보낼 수 있습니다.
- handle_message() 에서 __Message__ 를 받아 match 하여 서버의 상태를 변경하고 클라이언트에 메시지를 보낼 수 있습니다.


## 구성
* backend: _image/source/actix_
    - OAuth, TLS, SSL, 세션 연결
* frontend: _image/source/yew_
    - HTTP 페이지, 웹소켓 연결
* 웹앱 컨텐츠: _image/source/app_
    - 상태 관리, 페이지 렌더링, 메시지 핸들링
    - MySQL, Youtube Data Api 이용
* 유틸리티: _image/source/node_
    - Node 를 생성하는 매크로와 그 매크로에 관련된 Trait
* 샘플: _sample_
    - docker container 를 구동하기 위한 샘플


## 기능
- proc macro 를 이용한 wss 통신용 State/Node/Message 생성
- wss 프로토콜을 통한 server-client 간 상태 관리 통신
- Google OAuth 로그인
- Docker Image 생성
- docker-compose MySQL 접속
- Youtube data api 를 이용한 플레이리스트 보기
- MySQL을 이용한 플레이리스트 관리


## 예시
_image/source/app/music_ 폴더는 템플릿 샘플로 사용될 수 있습니다. 
_src/backend/component.rs_ 의 __Music__ 구조체에는 Music 과 관련하여 backend에서 처리해야 할 일들이 통합되어 있습니다.
_image/source/app/src/backend/component.rs_ 의 __ActixApp__ 구조체에서 각 작업을 __Music__ 에 분배합니다.

State: _image/source/app/music/src/state/client/musiclist.rs_
```rust
// Musiclist::{State, Node, Message} 자동 생성
#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Musiclist { // 음악 리스트
    pub page: PlaylistPage::State, // 음악 리스트의 현재 페이지
    pub pages: Vec<PlaylistPage::State>, // 현재 페이지의 추가 페이지들
    pub items: Vec<MusiclistItem::State>, // 현재 페이지의 음악들
}

// MusiclistItem::{State, Node, Message} 자동 생성
#[node_state]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MusiclistItem {
    pub music_id: String,
    pub datetime: String,
    pub youtube_title: String,
    pub title: String,
    pub artist: String,
    pub upload_by: String,
    pub lyrics_by: String,
    pub info: String,
    pub tag: String,
    pub volume: i32,
}
```

Layout: _image/source/app/music/src/view/mod.rs_
```rust
pub fn view(
    server: &MusicServer::Node,
    client: &MusicClient::Node,
) -> Html {
    // View들에 필요한 Node 들을 배분
    html! {
        <div class="horizontal_div">
            <PlaylistView 
                visible = { client.playlist_visible.clone() }
                playlist = { server.playlist.clone() }
                playlist_page = { client.musiclist.page.clone() }
            />
            <div>
                <YoutubePlayerView
                    music_id = { client.youtube_player.music_id.clone() }
                />
                <LyricsView/>
            </div>
            <div>
                <ServerPlayerView/>        
                <MusicQueueView/>    
                <MusiclistView
                    musiclist = { client.musiclist.clone() }
                    youtube_player_video_id = { client.youtube_player.music_id.clone() }
                />          
            </div>
        </div>
    }
}
```
View: _image/source/app/music/src/view/musiclist.rs_
```rust
#[derive(Properties, PartialEq)]
pub struct MusiclistProperty {
    pub musiclist: Musiclist::Node, // 음악 리스트
    pub youtube_player_music_id: ValueNode<String>, // 유튜브 플레이어에 표시할 음악 id 
}

#[function_component]
pub fn MusiclistView(prop: &MusiclistProperty) -> Html {    
    // 현재 페이지의 추가 페이지들
    let pages: Vec<_> = prop.musiclist.pages.clone_state().into_iter().enumerate()
    .map(|(index, page)| {     
        let musiclist_page = prop.musiclist.page.clone();    
        
        // {index} 버튼이 클릭되었을 때 musiclist_page Node 의 emit 을 호출하여 
        // 선택된 page 를 서버에 전송       
        let onclick_page = move |_| {
            musiclist_page.emit(page.clone())
        };
        html! {        
            <button onclick={onclick_page}>
            {index}
            </button>
        }
    })
    .collect();

    let items: Vec<_> = prop.musiclist.items.iter()
    .map(|item| {
        let youtube_player_music_id = prop.youtube_player_music_id.clone();
        let youtube_title = item.youtube_title.clone_state();
        let music_id = item.music_id.clone_state();
        
        // {title} 버튼이 클릭되었을 때 youtube_player_music_id Node 의 emit 을 호출하여 서버에 music_id 를 전송        
        let onclick_music = move |_| {
            youtube_player_music_id.emit(music_id.clone())
        };
        html! {
            <button onclick={onclick_music}>
            {youtube_title}
            </button>
        }
    }).collect(); 

    html! {
        <div style="display:flex; flex-direction: column;">
            <p>{"Musiclist"}</p>
            <div>{pages}</div>
            {items}
        </div>
    }
}
```

Control:

__handle_server_message__ 에서는 __ServerState__ 를 편집할 수 있고
모든 클라이언트에 결과 메시지를 보낼 수 있습니다.
```rust
pub async fn handle_server_message<Msg: RootMessage>(
    &self,
    senders: &HashMap<Uuid, UnboundedSender<Msg>>,
    server: &mut MusicServer::Node,
    message: MusicServer::Message,
) -> anyhow::Result<()> {
    Ok(match message {
        // Playlist Items Item Update 패턴의 메시지가 서버에 수신되었을 때 추가 처리:
        // 수신된 index 에 해당하는 PlaylistItem 의 playlist_id를 insert_update_musics 함수에 전달하여
        // Youtube Data Api 를 이용하여 음악들의 정보를 DB에 업데이트
        MusicServer::Message::Playlist(
            Playlist::Message::Items(
                VecMessage::Item(
                    (index, PlaylistItem::Message::Update(update))
                )
            )
        ) if update => {
            let playlist = server.playlist.items
            .get_mut(index)
            .ok_or(anyhow!("server.playlist.list_items.items has no index {index}"))?;

            // 모든 클라이언트에 update 시작 메시지 전송
            let message: Msg = playlist.update.apply_export(true);
            for sender in senders.values() {
                sender.send(message.clone())?;
            }
                                   
            let playlist_page = playlist.page.clone_state();
            let mut conn = self.pool.get_conn()?;
            
            // Youtube Data Api 로 playlist_page.id 에 해당하는 모든 음악 가져오기
            let playlist_items = get_playlist_items_all(
                &self.client, 
                &self.config, 
                &playlist_page.id,
            ).await?;

            // 가져온 음악들을 DB에 업데이트
            insert_update_musics(
                &mut conn, 
                &playlist_page.id,
                playlist_items,
            ).await?;               

            // playlist_page.id 에 해당하는 페이지들을 쿼리
            let pages = select_playlist_pages(
                &self.config, 
                &mut conn, 
                &playlist_page.id,
            )?;

            // 첫 페이지가 존재한다면 playlist에 적용
            // 모든 클라이언트에 변경 메시지 전송
            if let Some(page) = pages.first() {
                let message: Msg = playlist.page.apply_export(page.to_owned());
                for sender in senders.values() {
                    sender.send(message.clone())?;
                }
            }           

            // 모든 클라이언트에 update 완료 메시지 전송
            let message: Msg = playlist.update.apply_export(false);
            for sender in senders.values() {
                sender.send(message.clone())?;
            }
        },
        _ => {},
    })
}
```

__handle_client_message__ 에서는 __ServerState__ 를 읽을 수 있으며
메시지를 보낸 클라이언트의 __ClientState__ 를 편집할 수 있고
해당 클라이언트에 결과 메시지를 보낼 수 있습니다.
```rust    
pub async fn handle_client_message<Msg: RootMessage>(
    &self,
    sender: &UnboundedSender<Msg>,
    _server: &MusicServer::Node,
    client: &mut MusicClient::Node,
    message: MusicClient::Message,
) -> anyhow::Result<()> {
    Ok(match message {
        // Musiclist Page 패턴의 메시지가 서버에 수신되었을 때 추가 처리:
        // 수신된 page 정보를 select_musics 함수에 전달하여 
        // DB 로부터 Musiclist Item 들을 쿼리하여 Node에 Apply 하고 
        // 그 동작에 해당하는 Message 를 Export
        MusicClient::Message::Musiclist(
            Musiclist::Message::Page(_)        
        ) => {    
            let page = client.musiclist.page.clone_state();
            let message: Msg = client.musiclist.items.apply_export(
                select_musics(
                    &mut self.pool.get_conn()?, 
                    &page,
                )?,
            );       

            // 클라이언트에 musiclist.items 값을 넣는 메시지를 전송
            sender.send(message)?;  
        },
        _ => {},
    })
}   
```
