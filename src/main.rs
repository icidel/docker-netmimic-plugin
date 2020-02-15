use warp::Filter;

#[tokio::main]
async fn main() {
    // POST /Plugin.Activate => 200 OK with body 
    let activate = warp::path!("/Plugin.Activate").map(|| {
	// manifest
	r#"{"Implements":["NetworkDriver"]}"#
    });
	// POST /NetworkDriver.GetCapabilities -> 
    let getCapabilities = warp::path!("/NetworkDriver.GetCapabilities").map(|| { "" });;
	// POST /NetworkDriver.AllocateNetwork ->
	// POST /NetworkDriver.FreeNetwork ->
	// POST /NetworkDriver.CreateNetwork ->
	// POST /NetworkDriver.DeleteNetwork ->
	// POST /NetworkDriver.CreateEndpoint ->
	// POST /NetworkDriver.EndpointOperInfo ->
	// POST /NetworkDriver.DeleteEndpoint ->
	// POST /NetworkDriver.Join ->
	// POST /NetworkDriver.Leave ->
	// POST /NetworkDriver.DiscoverNew ->
	// POST /NetworkDriver.DiscoverDelete -> 
	// POST /NetworkDriver.ProgramExternalConnectivity ->
	// POST /NetworkDriver.RevokeExternalConnectivity -> 
    let routes = warp::post().and(activate.or(getCapabilities));
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
