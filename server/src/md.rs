pub trait MdToHtml {
  fn md_to_html(&self) -> String;
}

fn default_md_options() -> pulldown_cmark::Options {
  use pulldown_cmark::*;
  Options::all()
}

impl MdToHtml for &str {
  fn md_to_html(&self) -> String {
    use pulldown_cmark::*;
    let options = default_md_options();
    let parser = Parser::new_ext(self, options);
    let mut html = String::new();

    // Remove HTML
    let parser = parser.map(|e| match e {
      Event::Html(text) => Event::Text(text),
      _ => e,
    });

    html::push_html(&mut html, parser);
    html
  }
}

impl MdToHtml for String {
  fn md_to_html(&self) -> String {
    self.as_str().md_to_html()
  }
}

impl MdToHtml for std::borrow::Cow<'static, str> {
  fn md_to_html(&self) -> String {
    self.as_ref().md_to_html()
  }
}
