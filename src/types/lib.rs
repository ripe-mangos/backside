#[derive(PartialEq)]
pub enum Section {
    None,
    ScriptInfo,
    V4PlusStyles,
    Fonts,
    Events,
}

pub enum XOrYOrZ {
    X,
    Y,
    Z,
}

/// ASS/SSA override codes
///
/// - [**`\b`** *`"0" / "1"`*](#variant.Bold)
/// - [**`\i`** *`"0" / "1"`*](#variant.Italic)
/// - [**`\u`** *`"0" / "1"`*](#variant.Underline)
/// - [**`\s`** *`"0" / "1"`*](#variant.Strikeout)
/// - [**`\bord`** *`width`*](#variant.Border)
/// - [**`\shad`** *`depth`*](#variant.Shadow)
/// - [**`\be`** *`"0" / "1"`*](#variant.BlurEdges)
/// - [**`\fn`** *`font name`*](#variant.FontName)
/// - [**`\fs`** *`font size`*](#variant.FontSize)
/// - [**`\fsc`** *`"x" / "y"`* *`percent`*](#variant.Scale)
/// - [**`\fsp`** *`pixels`*](#variant.Spacing)
/// - [**`\fr`** *`"x" / "y" / "z"`* *`degrees`*](#variant.Rotation)
/// - [**`\fe`** *`charset`*](#variant.FontEncoding)
/// - [**`\c&H`** *`bbggrr`* **`&`**](#variant.Color)
/// - [**`\a`** *`alignment`*](#variant.Alignment)
/// - [**`\an`** *`alignment`*](#variant.AlignmentNumpad) **(ASS)**
/// - [**`\k`** *`duration`*](#variant.Kaoroke)
pub enum OverrideCode {
    /// # **`\b`** *`"0" / "1"`*
    ///
    /// `\b1` makes the text bold. `\b0` forces non-bold text.
    /// > eg. There is a {\b1}bold {\b0}word here
    ///
    /// ## ASS
    ///
    /// When this parameter is greater than 1, it will be used as the weight of the font.
    /// (400 = Normal, 700 = Bold, note: most fonts will quantize to 2 or 3 levels of thickness)
    Bold(bool),
    /// # **`\i`** *`"0" / "1"`*
    ///
    /// `\i1` makes the text italic. `\i0` forces non-italic text.
    /// > eg. `There is an {\i1}italicised {\i0}word here`
    Italic(bool),
    /// # **`\u`** *`"0" / "1"`*
    ///
    /// underline
    Underline(bool),
    /// # **`\s`** *`"0" / "1"`*
    ///
    /// strikeout
    Strikeout(bool),
    /// # **`\bord`** *`width`*
    Border(),
    /// # **`\shad`** *`depth`*
    Shadow(),
    /// # **`\be`** *`"0" / "1"`*
    ///
    /// blur edges
    BlurEdges(bool),
    /// # **`\fn`** *`font name`*
    ///
    /// specifies a font which you have installed in Windows. This is case sensitive.
    /// > e.g. `Here is some {\fnCourier New}fixed space text`
    ///
    /// If you use a font name that doesn't exist, then Arial will be used instead.
    FontName(String),
    /// # **`\fs`** *`font size`*
    ///
    /// A number specifying a font point size.
    /// > e.g. `{\fs16}This is small text. {\fs28}This is large text`
    FontSize(u16),
    /// # **`\fsc`** *`"x" / "y"`* *`percent`*
    Scale(XOrYOrZ, u16),
    /// # **`\fsp`** *`pixels`*
    Spacing(u16),
    /// # **`\fr`** *`"x" / "y" / "z"`* *`degrees`*
    ///
    /// *`degrees`* sets the rotation angle around the x/y/z axis.
    ///
    /// `\fr` defaults to `\frz`.
    Rotation(XOrYOrZ, u16),
    /// # **`\fe`** *`charset`*
    ///
    /// A number specifying the character set (font encoding)
    FontEncoding(),
    /// # **`\c&H`** *`bbggrr`* **`&`**
    ///
    /// *`bbggrr`* is a hexadecimal RGB value, but in reverse order. Leading zeroes are not required.
    /// > e.g. `{\c&HFF&}This is pure, full intensity red`
    /// >
    /// > `{\c&HFF00&}This is pure, full intensity Green`
    /// >
    /// > `{\c&HFF0000&}This is pure, full intensity Blue`
    /// >
    /// > `{\c&HFFFFFF&}This is White`
    /// >
    /// > `{\c&HA0A0A&}This is dark grey`
    ///
    /// `\1c&Hbbggrr&`, `\2c&Hbbggrr&`, `\3c&Hbbggrr&`, `\4c&Hbbggrr&` to set specific colors.
    ///
    /// `\1a&Haa&`, `\2a&Haa&`, `\3a&Haa&`, `\4a&Haa&` to set specific alpha channels.
    ///
    /// `\alpha` defaults to `\1a`
    Color(),
    /// # **`\a`** *`alignment`*
    ///
    /// *`alignment`* is a number specifying the onscreen alignment/positioning of a subtitle.
    ///
    /// - A value of 1 specifies a left-justified subtitle
    /// - A value of 2 specifies a centered subtitle
    /// - A value of 3 specifies a right-justified subtitle
    ///
    /// - Adding 4 to the value specifies a "Toptitle"
    /// - Adding 8 to the value specifies a "Midtitle"
    ///
    /// 0 or nothing resets to the style default (which is usually 2)
    /// > eg. `{\a1}This is a left-justified subtitle`
    /// >
    /// > `{\a2}This is a centered subtitle`
    /// >
    /// > `{\a3}This is a right-justified subtitle`
    /// >
    /// > `{\a5}This is a left-justified toptitle`
    /// >
    /// > `{\a11}This is a right-justified midtitle`
    ///
    /// ## ASS
    ///
    /// Only the first appearance counts.
    Alignment(),
    /// # **`\an`** *`alignment`* **(ASS)**
    ///
    /// numpad layout
    ///
    /// Only the first appearance counts.
    AlignmentNumpad(),
    /// # **`\k`** *`duration`*
    ///
    /// *`duration`* is the amount of time that each section of text is highlighted for in a dialogue event with the Karaoke effect. The durations are in hundredths of seconds.
    /// > eg. `{\k94}This {\k48}is {\k24}a {\k150}karaoke {\k94}line`
    ///
    /// `\k<duration>` highlight by words
    ///
    /// `\kf` or `\K<duration>` fill up from left to right
    ///
    /// `\ko<duration>` outline highlighting from left to right
    Karaoke(),
    /// # **`\q`** *`num`*
    ///
    /// *`num`*-- wrapping style
    WrappingStyle(),
    /// # **`\r`** *`style`*
    ///
    /// This cancels all previous style overrides in a line
    ///
    /// ## ASS
    ///
    /// *`style`* Restores to *`style`* instead of the dialogue line default.
    /// Any style modifier followed by no recognizable parameter resets to the default.
    Reset(),
}

