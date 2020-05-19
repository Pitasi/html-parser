use html_parser::prelude::*;
use indoc::indoc;
use insta::assert_debug_snapshot;

#[test]
fn it_can_parse_single_div_as_fragment() -> Result<()> {
    let markup = "<div/>";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_single_text_as_fragment() -> Result<()> {
    let markup = "hello";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_can_parse_text_comment_element_as_fragment() -> Result<()> {
    let markup = "hello<!--world?--><div/>";
    let ast = Ast::parse(markup)?;
    assert_debug_snapshot!(ast);
    Ok(())
}
#[test]
fn it_error_when_body_is_used_in_fragment_root() {
    let markup = "<div></div><body></body>";
    assert!(Ast::parse(markup).is_err());
}
#[test]
fn it_error_when_head_is_used_in_fragment_root() {
    let markup = "<div></div><head></head>";
    assert!(Ast::parse(markup).is_err());
}
#[test]
fn it_error_when_html_is_used_in_fragment_root() {
    let markup = "<div></div><html></html>";
    assert!(Ast::parse(markup).is_err());
}
