pub struct Node {
    url: &'static str,
}

impl Node {

    pub fn new(url: &'static str) -> Self {
        Self{url: url}
    }

    pub fn request(&self, req: &str) -> String {
        ureq::post(self.url).
        send_string(req).
        unwrap().
        into_string().
        unwrap()
    }
}
