use std::collections::HashMap;

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

type BoxedCallback = Box<Fn(&Request) -> Response>;

struct BasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl BasicRouter {
    fn new() -> Self {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    fn add_route<C>(&mut self, url: &str, callback: C) where C: Fn(&Request) -> Response + 'static {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    fn handle_request(&self,request:&Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => calback(request)
        }
    }
}