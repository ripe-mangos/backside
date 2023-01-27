use crate::*;

/// Style
#[derive(Default, Clone)]
pub struct Style {
    /// #1:
    ///  `Name`
    ///
    /// The name of the Style.
    /// Case sensitive. Cannot include commas.
    pub name: String,

    /// #2:
    ///  `Fontname`
    ///
    /// Case-sensitive.
    pub font_name: String,

    /// #3:
    ///  `Fontsize`
    pub font_size: i8,

    /// #4:
    ///  `PrimaryColour`
    ///
    /// A long integer BGR (blue-green-red) value. ie. the byte order in the hexadecimal equivelent of this number is BBGGRR
    /// This is the colour that a subtitle will normally appear in.
    pub primary_color: i64,

    /// #5:
    ///  `SecondaryColour`
    ///
    /// A long integer BGR (blue-green-red) value. ie. the byte order in the hexadecimal equivelent of this number is BBGGRR
    /// This colour may be used instead of the Primary colour when a subtitle is automatically shifted to prevent an onscreen collsion, to distinguish the different subtitles.
    pub secondary_color: i64,

    /// #6:
    ///  `OutlineColor`   (ASS),
    ///  `TertiaryColour` (SSA)
    ///
    /// A long integer BGR (blue-green-red) value. ie. the byte order in the hexadecimal equivelent of this number is BBGGRR.
    /// This colour may be used instead of the Primary colour when a subtitle is automatically shifted to prevent an onscreen collsion, to distinguish the different subtitles.
    pub outline_color: i64,

    /// #7:
    ///  `BackColour`
    ///
    /// This is the colour of the subtitle outline or shadow, if these are used.
    /// A long integer BGR (blue-green-red) value. ie. the byte order in the hexadecimal equivelent of this number is BBGGRR.
    pub back_color: i64,

    /// #8:
    ///  `Bold`
    ///
    /// - `-1` = On
    /// - `0`  = Off
    pub bold: bool,

    /// #9:
    ///  `Italic`
    ///
    /// - `-1` = On
    /// - `0`  = Off
    pub italic: bool,

    /// #9.1:
    ///  `Underline` (ASS)
    ///
    /// - `-1` = On
    /// - `0`  = Off
    pub underline: bool,

    /// #9.2:
    ///  `Strikeout` (ASS)
    ///
    /// - `-1` = On
    /// - `0`  = Off
    pub strikeout: bool,

    /// #9.3:
    ///  `ScaleX` (ASS)
    ///
    /// Modifies the width of the font. (percent)
    pub scale_x: i16,

    /// #9.4:
    ///  `ScaleY` (ASS)
    ///
    /// Modifies the height of the font. (percent)
    pub scale_y: i16,

    /// #9.5:
    ///  `Spacing` (ASS)
    ///
    /// Extra space between characters. (pixels)
    pub spacing: i32,

    /// #9.6:
    ///  `Angle` (ASS)
    ///
    /// The origin of the rotation is defined by the alignment.
    /// Can be a floating point number. (degrees)
    pub angle: f32,

    /// #10:
    ///  `BorderStyle`
    ///
    /// - 1 = Outline + drop shadow
    /// - 3 = Opaque box
    pub border_style: i8,

    /// #11:
    ///  `Outline`
    ///
    /// If BorderStyle is 1, then this specifies the width of the outline around the text, in pixels.
    /// Values may be 0, 1, 2, 3 or 4.
    pub outline: f32,

    /// #12:
    ///  `Shadow`
    ///
    /// If BorderStyle is 1, then this specifies the depth of the drop shadow behind the text, in pixels.
    /// Values may be 0, 1, 2, 3 or 4. Drop shadow is always used in addition to an outline - SSA will force an outline of 1 pixel if no outline width is given.
    pub shadow: f32,

    /// #13:
    ///  `Alignment`
    ///
    /// Alignment is *completely* different between SSA and ASS.
    ///
    /// ## SSA
    /// This sets how text is "justified" within the Left/Right onscreen margins, and also the vertical placing.
    ///
    /// Values may be:
    ///  - `1` = Left
    ///  - `2` = Centered
    ///  - `3` = Right
    ///
    /// Add 4 to the value for a "Toptitle". Add 8 to the value for a "Midtitle". eg. 5 = left-justified toptitle.
    /// 
    /// ## ASS
    /// After the layout of a standard keypad:
    ///  - 1-3 = Sub (Bottom)
    ///  - 4-6 = Mid (Middle)
    ///  - 7-9 = Top (Top)
    pub alignment: i8,

    /// #14:
    ///  `MarginL`
    ///
    /// This defines the Left Margin in pixels.
    /// It is the distance from the left-hand edge of the screen. The three onscreen margins (MarginL, MarginR, MarginV) define areas in which the subtitle text will be displayed.
    pub margin_l: i32,

    /// #15:
    ///  `MarginR`
    ///
    /// This defines the Right Margin in pixels.
    /// It is the distance from the right-hand edge of the screen. The three onscreen margins (MarginL, MarginR, MarginV) define areas in which the subtitle text will be displayed.
    pub margin_r: i32,

    /// #16:
    ///  `MarginV`
    ///
    /// This defines the vertical Left Margin in pixels.
    /// For a subtitle, it is the distance from the bottom of the screen.
    /// For a toptitle, it is the distance from the top of the screen.
    /// For a midtitle, the value is ignored - the text will be vertically centred.
    pub margin_v: i32,
}

pub fn parse_style(r: &str) -> Option<Style> {
    // Parse fields
    let fields: Vec<&str> = {
        let o;
        let strp = l.strip_prefix("Style: ");
        if strp != None {
            o = strp.unwrap();
        } else {
            return None;
        }
        o.split(",").collect()
    };

    let iter = format.iter().clone().zip(fields.clone());

    let mut style = Style::default();
    for f in iter {
        let k: &str = f.0;
        let v: String = f.1.to_string();

        macro_rules! parse {
            () => {
                match v.parse::<i8>().unwrap() {
                    -1 => true,
                    0  => false,
                    _  => unimplemented!(),
                }
            };
            ($type:ident) => {
                v.parse::<$type>().unwrap()
            };
        }

        match k {
            "Name"        => style.name = v,
            "Fontname"    => style.font_name = v,
            "Fontsize"    => style.font_size = parse!(i8),
            "Bold"        => style.bold = parse!(),
            "Italic"      => style.italic = parse!(),
            "Underline"   => style.underline = parse!(),
            "Strikeout"   => style.strikeout = parse!(),
            "ScaleX"      => style.scale_x = parse!(i16),
            "ScaleY"      => style.scale_y = parse!(i16),
            "Spacing"     => style.spacing = parse!(i32),
            "Angle"       => style.angle = parse!(f32),
            "BorderStyle" => style.border_style = parse!(i8),
            "Outline"     => style.outline = parse!(f32),
            "Shadow"      => style.shadow = parse!(f32),
            "Alignment"   => style.alignment = parse!(i8),
            "MarginL"     => style.margin_l = parse!(i32),
            "MarginR"     => style.margin_r = parse!(i32),
            _             => {}
        }
    }

    Some(style)
}
