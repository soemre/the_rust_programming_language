pub struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}

impl Post {
    pub fn new() -> Post {
        Self {
            content: String::new(),
            state: Some(Box::new(Draft)),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        let text = self.state.as_mut().unwrap().add_text(text);
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

#[allow(unused)]
trait State {
    fn add_text<'a>(&mut self, text: &'a str) -> &'a str {
        ""
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn obj(self: Box<Self>) -> Box<dyn State>;

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self.obj()
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self.obj()
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self.obj()
    }
}

struct Draft;
impl State for Draft {
    fn obj(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text<'a>(&mut self, text: &'a str) -> &'a str {
        text
    }

    fn request_review(self: Box<Self>) -> Box<dyn State>
    where
        Self: Sized + 'static,
    {
        Box::new(PendingReview {
            requires_verification: true,
        })
    }
}

struct PendingReview {
    requires_verification: bool,
}
impl State for PendingReview {
    fn obj(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State>
    where
        Self: Sized + 'static,
    {
        if self.requires_verification {
            self.requires_verification = false;
            self
        } else {
            Box::new(Published)
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State>
    where
        Self: Sized + 'static,
    {
        Box::new(Draft)
    }
}

struct Published;
impl State for Published {
    fn obj(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
