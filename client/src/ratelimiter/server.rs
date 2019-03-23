use futures::future::Future;
use hyper::{Body, Request, Server};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};

use super::RatelimitOptions;

pub fn start_server(cfg: RatelimitOptions) {
    let address = cfg.address.clone();
    let make_svc = make_service_fn(move |socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        service_fn({
            let mut url = cfg.url.clone();
            move |req: Request<Body>| hyper_reverse_proxy::call(remote_addr.ip(), &mut url, req)
            /*.and_then(|response| {

            })*/
        })
    });

    let server = Server::bind(&address)
        .serve(make_svc)
        .map_err(|e| error!("Failed to start server: {:?}", e));

    info!("Begin: Running HTTP ratelimit proxy on {:?}", address);

    hyper::rt::run(server);
}

