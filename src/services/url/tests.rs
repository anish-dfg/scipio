use anyhow::Result;

use crate::services::url::UrlBuilder;

#[test]
pub fn test_build_url() -> Result<()> {
    let url = UrlBuilder::default()
        .host("anishsinha.io")
        .path(vec!["blog".to_string(), "posts".to_string()])
        .query(vec![
            ("key1".to_owned(), "value1".to_owned()),
            ("key2".to_owned(), "value2".to_owned()),
        ])
        .fragment("frag".to_owned())
        .build()?;

    dbg!(&url);
    assert_eq!(url.to_string(), "https://anishsinha.io/blog/posts?key1=value1&key2=value2#frag");
    Ok(())
}

#[test]
pub fn test_parse_url() -> Result<()> {
    let url = UrlBuilder::parse("https://anishsinha.io/blog/posts?key1=value1&key2=value2#frag")?
        .build()?;
    dbg!(&url);

    assert_eq!(url.to_string(), "https://anishsinha.io/blog/posts?key1=value1&key2=value2#frag");
    Ok(())
}
