pub struct Client<
    S: Debug + Send + Sized + 'static,
    A: Algorithm<S, SocketAddr> + Sized + 'static,
    D: Adapter + Send + 'static,
> {
    adapter: D,
    algorithm: A,
    // ...
}
