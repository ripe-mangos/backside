//! # `backside_parser`
//! Parser implementation for `backside`

use core::str::from_utf8;

//mod styles;

use backside_types::*;

pub fn parse_bool(r: u8) -> bool {
    match from_utf8(&[r]).unwrap().parse::<u8>().unwrap() {
        0 => false,
        1 => true,
        _ => panic!()
    }
}

pub fn parse_override(r: &[u8]) -> OverrideCode {
    let mut i: usize = 0;
    let len = r.len();

    i += 1;

    use OverrideCode::*;

    if len == 3 {
        match &r[1] {
            b'b' => Bold(parse_bool(r[2])),
            b'i' => Italic(parse_bool(r[2])),
            b'u' => Underline(parse_bool(r[2])),
            b's' => Strikeout(parse_bool(r[2])),
            &_ => unimplemented!()
        }
    } else if len == 4 {
        match &r[1..3] {
            b"be" => BlurEdges(parse_bool(r[3])),
            &_ => unimplemented!()
        }
    } else {
        unimplemented!()
    }
}

pub fn parse_dialogue(r: &[u8]) -> String {
    let mut i: usize = 0;
    loop {
        if r[i] == b'{' {
            i += 1;
            if r[i] == b'\\' {
                // It's an override

                let o1 = i;
                let o2;
                loop {
                    if r[i+1] == b'\\' || r[i+1] == b'}' {
                        o2 = i;
                        break;
                    }
                    i += 1;
                }
                let ovrd = &r[o1..o2];

                parse_override(ovrd);
            } else {
                // It's a comment
            }
        }
    }
}

pub fn parse_format(r: &[u8]) -> Vec<&[u8]> {
    let mut i: usize = 8;
    let len = r.len();
    let mut p: Vec<&[u8]> = Vec::new();

    assert_eq!(&r[0..6], b"Format");

    // Loop until EOL
    loop {
        let st = i;

        // Parse fields
        loop {
            // If there's a comma, end of field
            if r[i] == b',' {
                // Push field to vec
                p.push(&r[st..i]);

                i += 1;

                // If there's a space after the comma, skip over it
                // (ASS uses ", " rather than SSA's ",")
                //
                // Otherwise, skip just the comma
                if r[i] == b' ' {
                    i += 1;
                }
                break;
            }
            // If end of line, break
            else if i == (len-1) { break; }
            // Otherwise, next char
            else { i += 1; }
        }

        if i == (len-1) {
            break;
        }
    }

    p
}

pub fn parse_sections(tr: &str) -> String {
    let mut r = tr.as_bytes();

    let mut mode: Section = Section::None;

    let mut style_init = false;
    let mut fonts_init = false;
    let mut events_init = false;
    let mut style_fmt: Vec<&str> = Vec::new();
    let mut events_fmt: Vec<&str> = Vec::new();
    let mut ret = String::new();

    let mut i: usize = 0;
    loop {
        if i >= r.len() - 1 {
            break;
        }
        if r[i] == b'[' {
            i += 1;

            let h = {
                let st = i;
                let ed;
                loop {
                    if r[i] == b']' {
                        ed = i;
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                &r[st..ed]
            };

            i += 1;

            let body = {
                let st = i;
                let ed;
                loop {
                    if i+2 >= r.len() {
                        ed = r.len() - 1;
                        break;
                    }
                    if r[i] == b'\n' && r[i+1] == b'\n' {
                        ed = i;
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                &r[st..ed]
            };

            match h {
                b"Script Info" => {},
                b"V4+ Styles" => {
                    let mut i: usize = 0;
                    let e;
                    loop {
                        if body[i] == b'\n' {
                            e = i;
                            break;
                        }
                        i += 1;
                    }
                    for f in parse_format(&body[0..e]) {
                        ret.push_str(from_utf8(f).unwrap());
                        ret.push('\n');
                    }
                },
                b"Fonts" => {},
                b"Events" => {
                    let mut i: usize = 0;
                    let e;
                    loop {
                        if body[i] == b'\n' {
                            e = i;
                            break;
                        }
                        i += 1;
                    }
                    for f in parse_format(&body[0..e]) {
                        ret.push_str(from_utf8(f).unwrap());
                        ret.push('\n');
                    }
                },
                &_ => {}
            }
        } else {
            i += 1;
        }
    }

    ret
}
