use super::super::config::Networks;
use cardano_storage::{tag, Error};
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
        router.get(":network/tip", self, "tip")
    }
}

impl iron::Handler for Handler {
    // XXX
    //
    // The current implementation of the TIP handler is to look for the HEAD tag
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let (_, net) = match common::get_network(req, &self.networks) {
            None => {
                return Ok(Response::with((status::BadRequest, "Invalid network name")));
            }
            Some(x) => x,
        };

        match net.storage.read().unwrap().get_block_from_tag(&tag::HEAD) {
            Err(Error::NoSuchTag) => Ok(Response::with((status::NotFound, "No Tip To Serve"))),
            Err(err) => {
                error!("error while reading block: {:?}", err);
                Ok(Response::with((status::InternalServerError, "Couldn't find Tip")))
            }
            Ok(block) => Ok(Response::with((
                status::Ok,
                block.header().to_raw().as_ref(),
            ))),
        }
    }
}
