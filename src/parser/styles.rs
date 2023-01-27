use backside::*;

pub fn parse_style(r: &[u8]) -> Result<Style> {
  let mut i: usize = 7;
  assert_eq!(&r[0..5], b"Style");

  let mut style = Style::default();

  for f in fields.enumerate() {
    let st = i;
    let ed;

    comma_sep!(r[i], ed);

    let v = &r[st..ed]
    match f {
      b"Name" => style.name = v,
      b"Fontname" => style.font_name = v,
      b"Fontsize" => style.font_size = v,
      &_ => {
        return Error::StyleInvalid;
      }
    }
  }

  Ok(style)
}
