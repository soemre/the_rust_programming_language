#[test]
fn general() {
    use state_pattern::Post;
    const TEST_TEXT: &str = "Cool Text";

    let mut post = Post::new();
    assert_eq!("", post.content());

    post.add_text(TEST_TEXT);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.approve();
    assert_ne!(TEST_TEXT, post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(TEST_TEXT, post.content());

    post.add_text("this should not change anything");
    assert_eq!(TEST_TEXT, post.content());
}
