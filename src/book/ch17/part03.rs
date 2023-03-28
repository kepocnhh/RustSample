pub fn run() {
    const CHAPTER: u8 = 17;
    const PART: u8 = 3;
    const TITLE: &str = "Implementing an Object-Oriented Design Pattern";
    println!("\n\t{:02}/{:02}\t\"{TITLE}\"", CHAPTER, PART);

    _01();
    _02();
}

mod blog {
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String
    }

    impl Post {
        pub fn new() -> Post {
            return Post {
                state: Some(Box::new(Draft {})),
                content: String::new()
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text)
        }

        pub fn content(&self) -> &str {
            return self.state.as_ref().unwrap().content(self);
        }

        pub fn request_review(&mut self) {
            if let Some(it) = self.state.take() {
                self.state = Some(it.request_review())
            }
        }

        pub fn reject(&mut self) {
            if let Some(it) = self.state.take() {
                self.state = Some(it.reject())
            }
        }

        pub fn approve(&mut self) {
            if let Some(it) = self.state.take() {
                self.state = Some(it.approve())
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            return "";
        }
        fn add_text(&self, _post: &mut Post, text: &str);
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            return Box::new(PendingReview {});
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            panic!("Still a draft!");
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            panic!("Still a draft!");
        }

        fn add_text(&self, post: &mut Post, text: &str) {
            post.content.push_str(text)
        }
    }

    struct PendingReview {}

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            panic!("Pending review already!");
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            return Box::new(Draft {});
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            return Box::new(Published {});
        }

        fn add_text(&self, _post: &mut Post, _text: &str) {
            panic!("Pending review already!");
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            panic!("Published already!");
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            panic!("Published already!");
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            panic!("Published already!");
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            return &post.content;
        }

        fn add_text(&self, _post: &mut Post, _text: &str) {
            panic!("Published already!");
        }
    }
}

fn _01() {
    println!("\nDefining Post and Creating a New Instance in the Draft State");

    let mut post = blog::Post::new();

    let expected = "I ate a salad for lunch today";
    post.add_text(expected);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    let actual = post.content();
    assert_eq!(expected, actual);
    println!("expected: '{expected}'");
    println!("actual: '{actual}'");
}

mod _02 {
    pub struct DraftPost {
        content: String
    }

    impl DraftPost {
        pub fn new() -> DraftPost {
            return DraftPost {
                content: String::from("")
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            return PendingReviewPost { content: self.content };
        }
    }

    pub struct PendingReviewPost {
        content: String
    }

    impl PendingReviewPost {
        pub fn reject(self) -> DraftPost {
            return DraftPost { content: self.content }
        }

        pub fn approve(self) -> PublishedPost {
            return PublishedPost { content: self.content }
        }
    }

    pub struct PublishedPost {
        content: String
    }

    impl PublishedPost {
        pub fn content(&self) -> &str {
            return &self.content;
        }
    }
}

fn _02() {
    println!("\nDefining Post and Creating a New Instance in the Draft State");

    let mut post = _02::DraftPost::new();

    post.add_text("foo");
    let post = post.request_review();
    let mut post = post.reject();
    post.add_text("bar");
    let post = post.request_review();
    let post = post.approve();
    let expected = "foobar";
    let actual = post.content();
    assert_eq!(expected, actual);
    println!("expected: '{expected}'");
    println!("actual: '{actual}'");
}
