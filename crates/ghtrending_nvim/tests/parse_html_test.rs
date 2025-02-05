use scraper::{Html, Selector};

#[test]
fn test_parse_input_attr() {
    let fragment = Html::parse_fragment(r#"<input name="foo" value="bar">"#);
    let selector = Selector::parse(r#"input[name="foo"]"#).unwrap();

    let input = fragment.select(&selector).next().unwrap();
    assert_eq!(input.value().attr("name"), Some("foo"));
    assert_eq!(input.value().attr("value"), Some("bar"));
}

#[test]
fn test_parse_article() {
    let fragement = Html::parse_fragment(r#"<article class="Box-row">...</article>"#);
    let selector = Selector::parse(r#"article[class="Box-row"]"#).unwrap();

    let article = fragement.select(&selector).next().unwrap();
    assert_eq!(article.value().attr("class"), Some("Box-row"));
    assert_eq!(article.text().collect::<Vec<_>>(), vec!["...".to_string()]);
}
