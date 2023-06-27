use warp::Filter;

mod service;
mod server;

#[tokio::main]
async fn main() {
    // Serve the form
    let form = server::form().with(warp::cors().allow_any_origin());

    // Handle form submission
    let submit = server::submit_form()
        .and_then(server::form_handler)
        .with(warp::cors().allow_any_origin());

    // Combine all the routes
    let routes = warp::get().and(form.or(submit));

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
