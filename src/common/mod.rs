use actix::{Actor,Addr,Handler, prelude::{Message}};
use actix_web::{dev::HttpServiceFactory, web,  HttpResponse,HttpRequest, Error};
use actix_web_actors::ws;
use std::sync::RwLock;

pub mod geom;

pub struct Game<PlayerType> {
    players: Vec<PlayerType>,
}

impl<P> Game<P>{
    pub fn new() -> Self {
        Self{
            players: Vec::new(),
        }
    }
}

pub struct WebsocketHandler <ServerActor: Actor> {
   addr: Addr<ServerActor>,
    playerCount: RwLock<u32>,
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

impl<ServerActor> WebsocketHandler<ServerActor> where ServerActor: Actor + Handler<Connect> {
    pub fn new(a: Addr<ServerActor>) -> Self {
        Self {
            addr: a,
            playerCount: RwLock::new(0),
        }
    }

    async fn ws_route(
        &self,
        req: HttpRequest,
        stream: web::Payload,
        srv: web::Data<Addr<ServerActor>>,
    ) -> Result<HttpResponse, Error> {
        ws::start(
            WsChatSession {
                id: 0,
                hb: Instant::now(),
                room: "Main".to_owned(),
                name: None,
                addr: srv.get_ref().clone(),
            },
            &req,
            stream,
        )
    }

    pub fn actix_scope(&self) -> impl HttpServiceFactory {
        web::scope("")
            .service(web::resource("").to(&self.ws_route))
            .service(web::resource("/n").to(|| HttpResponse::Ok().body(self.playerCount.read().map_or(0, |v| *v).to_string())))
    }
}
