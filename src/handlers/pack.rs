use super::super::config::Networks;
use cardano::util::hex;
use cardano_storage::tag;
use cardano_storage::types::HASH_SIZE;
use std::sync::Arc;

use iron;
use iron::status;
use iron::{IronResult, Request, Response};

use router;
use router::Router;

use super::common;

pub struct Handler {
    networks: Arc<Networks>,
}
impl Handler {
    pub fn new(networks: Arc<Networks>) -> Self {
        Handler { networks: networks }
    }
    pub fn route(self, router: &mut Router) -> &mut Router {
        router.get(":network/pack/:packid", self, "pack")
    }
}

impl iron::Handler for Handler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let (_, net) = match common::get_network(req, &self.networks) {
            None => {
                return Ok(Response::with((status::BadRequest, "Invalid network name")));
            }
            Some(x) => x,
        };
        let ref packid = req
            .extensions
            .get::<router::Router>()
            .unwrap()
            .find("packid")
            .unwrap();
        if !packid
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            error!("invalid packid: {}", packid);
            return Ok(Response::with(status::BadRequest));
        }
        info!("query pack: {}", packid);
        let packhash_vec = match tag::read(&net.storage.read().unwrap(), &packid) {
            None => hex::decode(&packid).unwrap(),
            Some(t) => t,
        };

        let mut packhash = [0; HASH_SIZE];
        packhash[..].clone_from_slice(packhash_vec.as_slice());
        let path = net
            .storage
            .read()
            .unwrap()
            .config
            .get_pack_filepath(&packhash);

        Ok(Response::with((status::Ok, path)))
    }
}
