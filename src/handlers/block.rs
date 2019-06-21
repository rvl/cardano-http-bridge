use super::super::config::Networks;
use cardano::block;
use cardano::util::{hex, try_from_slice::TryFromSlice};
use cardano_storage::tag;
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
        router.get(":network/block/:blockid", self, "block")
    }
}

impl iron::Handler for Handler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let params = req.extensions.get::<router::Router>().unwrap();

        let (_, net) = match common::get_network(req, &self.networks) {
            None => {
                return Ok(Response::with((status::BadRequest, "Invalid network name")));
            }
            Some(x) => x,
        };

        let ref blockid = params.find("blockid").unwrap();

        if !blockid.chars().all(|c| c.is_ascii_alphanumeric()) {
            error!("invalid blockid: {}", blockid);
            return Ok(Response::with(status::BadRequest));
        }
        let hh_bytes = match tag::read(&net.storage.read().unwrap(), &blockid) {
            None => hex::decode(&blockid).unwrap(),
            Some(t) => t,
        };
        let hh = block::HeaderHash::try_from_slice(&hh_bytes).expect("blockid invalid");
        info!("querying block header: {}", hh);

        match &(net.storage)
            .read()
            .unwrap()
            .block_location(&hh.clone().into())
        {
            Err(_) => {
                warn!("block `{}' does not exist", hh);
                Ok(Response::with((status::NotFound, "Not Found")))
            }
            Ok(loc) => {
                debug!("blk location: {:?}", loc);
                match net.storage.read().unwrap().read_block_at(&loc) {
                    Err(_) => {
                        error!("error while reading block at location: {:?}", loc);
                        Ok(Response::with(status::InternalServerError))
                    }
                    Ok(rblk) => Ok(Response::with((status::Ok, rblk.as_ref()))),
                }
            }
        }
    }
}
