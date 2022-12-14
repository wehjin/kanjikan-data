use serde_json::json;
use worker::*;

mod utils;

fn log_request(req: &Request) {
	console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

mod data;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
	log_request(&req);

	// Optionally, get more helpful error messages written to the console in the case of a panic.
	utils::set_panic_hook();

	// Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
	// catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
	// provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
	let router = Router::new();

	// Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
	// functionality and a `RouteContext` which you can use to  and get route parameters and
	// Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
	router
		.get("/", |_, _| Response::ok("Hello from Workers!"))
		.post_async("/form/:field", |mut req, ctx| async move {
			if let Some(name) = ctx.param("field") {
				let form = req.form_data().await?;
				return match form.get(name) {
					Some(FormEntry::Field(value)) => {
						Response::from_json(&json!({ name: value }))
					}
					Some(FormEntry::File(_)) => {
						Response::error("`field` param in form shouldn't be a File", 422)
					}
					None => Response::error("Bad Request", 400),
				};
			}

			Response::error("Bad Request", 400)
		})
		.get("/cards", |_, _| {
			let cards = data::cards();
			let response = Response::from_json(&cards)
				.map(|mut response| {
					response.headers_mut().append("Access-Control-Allow-Origin", "*").expect("Set allow-origin");
					response
				});
			response
		})
		.get("/font", |_, _| {
			let font = data::font();
			Response::from_json(&font)
		})
		.get("/worker-version", |_, ctx| {
			let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
			Response::ok(version)
		})
		.run(req, env)
		.await
}
