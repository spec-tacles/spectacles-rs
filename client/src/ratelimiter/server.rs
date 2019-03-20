use super::RatelimitConfig;

pub fn start_server(cfg: RatelimitConfig) {
    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        service_fn(move |req: Request<Body>| hyper_reverse_proxy::call(remote_addr.ip(), &cfg.url, req))
    });

    let server = Server::bind(&cfg.addr)
        .serve(make_svc)
        .map_err(|e| error!("Failed to start server: {:?}", e));

    info!("Begin: Running HTTP ratelimit proxy on {:?}", addr);

    hyper::rt::run(server);
}