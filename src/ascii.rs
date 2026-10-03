/// One of the 128 Unicode characters from U+0000 through U+007F,
/// often known as the [ASCII] subset.
///
/// Officially, this is the first [block] in Unicode, _Basic Latin_.
/// For details, see the [*C0 Controls and Basic Latin*][chart] code chart.
///
/// This block was based on older 7-bit character code standards such as
/// ANSI X3.4-1977, ISO 646-1973, and [NIST FIPS 1-2].
///
/// Based on the Standard Library [`ascii::Char`][std ascii].
///
/// # Layout
///
/// This type is guaranteed to have a size and alignment of 1 byte.
///
/// # Names
///
/// The variants on this type are [Unicode names][NamesList] of the characters
/// in upper camel case, with a few tweaks:
/// - For `<control>` characters, the primary alias name is used.
/// - `LATIN` is dropped, as this block has no non-latin letters.
/// - `LETTER` is dropped, as `CAPITAL`/`SMALL` suffices in this block.
/// - `DIGIT`s use a single digit rather than writing out `ZERO`, `ONE`, etc.
///
/// [ASCII]: https://www.unicode.org/glossary/index.html#ASCII
/// [block]: https://www.unicode.org/glossary/index.html#block
/// [chart]: https://www.unicode.org/charts/PDF/U0000.pdf
/// [NIST FIPS 1-2]: https://nvlpubs.nist.gov/nistpubs/Legacy/FIPS/fipspub1-2-1977.pdf
/// [NamesList]: https://www.unicode.org/Public/15.0.0/ucd/NamesList.txt
/// [std ascii]: https://doc.rust-lang.org/core/ascii/enum.Char.html
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Ascii {
    /// Null character (control code) `0x00`.
    #[default]
    Null = 0,
    /// Start of Heading (control code) `0x01`.
    StartOfHeading = 1,
    /// Start of Text (control code) `0x02`.
    StartOfText = 2,
    /// End of Text (control code) `0x03`.
    EndOfText = 3,
    /// End of Transmission (control code) `0x04`.
    EndOfTransmission = 4,
    /// Enquiry (control code) `0x05`.
    Enquiry = 5,
    /// Acknowledge (control code) `0x06`.
    Acknowledge = 6,
    /// Bell (control code) `0x06`.
    Bell = 7,
    /// Backspace (control code) `0x08`.
    Backspace = 8,
    /// Horizontal Tab (control code) `0x09`.
    CharacterTabulation = 9,
    /// Line Feed (control code) `0x0A`.
    LineFeed = 10,
    /// Vertical Tabulation (control code) `0x0B`.
    LineTabulation = 11,
    /// Form Feed (control code) `0x0C`.
    FormFeed = 12,
    /// Carriage Return (control code) `0x0D`.
    CarriageReturn = 13,
    /// Shift Out (control code) `0x0E`.
    ShiftOut = 14,
    /// Shift In (control code) `0x0F`.
    ShiftIn = 15,
    /// Data Link Escape (control code) `0x10`.
    DataLinkEscape = 16,
    /// Device Control One (control code) `0x11`.
    DeviceControlOne = 17,
    /// Device Control Two (control code) `0x12`.
    DeviceControlTwo = 18,
    /// Device Control Three (control code) `0x13`.
    DeviceControlThree = 19,
    /// Device Control Four (control code) `0x14`.
    DeviceControlFour = 20,
    /// Negative Acknowledge (control code) `0x15`.
    NegativeAcknowledge = 21,
    /// Synchronous Idle (control code) `0x16`.
    SynchronousIdle = 22,
    /// End of Transmission Block (control code) `0x17`.
    EndOfTransmissionBlock = 23,
    /// Cancel (control code) `0x18`.
    Cancel = 24,
    /// End of Medium (control code) `0x19`.
    EndOfMedium = 25,
    /// Substitute (control code) `0x1A`.
    Substitute = 26,
    /// Escape (control code) `0x1B`.
    Escape = 27,
    /// Information Separator Four or File Separator (control code) `0x1C`.
    InformationSeparatorFour = 28,
    /// Information Separator Three or Group Separator (control code) `0x1D`.
    InformationSeparatorThree = 29,
    /// Information Separator Two or Record Separator (control code) `0x1E`.
    InformationSeparatorTwo = 30,
    /// Information Separator One or Unit Separator (control code) `0x1F`.
    InformationSeparatorOne = 31,
    /// Space (` `).
    Space = 32,
    /// Exclamation Mark (`!`).
    ExclamationMark = 33,
    /// Quotation Mark or Double quotes (`"`).
    QuotationMark = 34,
    /// Number Sign (`#`).
    NumberSign = 35,
    /// Dollar Sign (`$`).
    DollarSign = 36,
    /// Percent Sign (`%`).
    PercentSign = 37,
    /// Ampersand (`&`).
    Ampersand = 38,
    /// Apostrophe or Single quotes (`'`).
    Apostrophe = 39,
    /// Left Parenthesis (`(`).
    LeftParenthesis = 40,
    /// Right Parenthesis (`)`).
    RightParenthesis = 41,
    /// Asterisk (`*`).
    Asterisk = 42,
    /// Plus sign (`+`).
    PlusSign = 43,
    /// Comma (`,`).
    Comma = 44,
    /// Hyphen-Minus (`-`).
    HyphenMinus = 45,
    /// Period, Dot or Full Stop (`.`).
    FullStop = 46,
    /// Slash, Devide or Solidus (`/`).
    Solidus = 47,
    /// Digit Zero (`0`).
    Digit0 = 48,
    /// Digit One (`1`).
    Digit1 = 49,
    /// Digit Two (`2`).
    Digit2 = 50,
    /// Digit Three (`3`).
    Digit3 = 51,
    /// Digit Four (`4`).
    Digit4 = 52,
    /// Digit Five (`5`).
    Digit5 = 53,
    /// Digit Six (`6`).
    Digit6 = 54,
    /// Digit Seven (`7`).
    Digit7 = 55,
    /// Digit Eight (`8`).
    Digit8 = 56,
    /// Digit Nine (`9`).
    Digit9 = 57,
    /// Colon (`:`).
    Colon = 58,
    /// Semicolon (`;`).
    Semicolon = 59,
    /// Less Than Sign or Open Angle Bracket (`<`).
    LessThanSign = 60,
    /// Equals Sign (`=`).
    EqualsSign = 61,
    /// Greater Than Sign or Close Angle Bracket (`>`).
    GreaterThanSign = 62,
    /// Question Mark (`?`).
    QuestionMark = 63,
    /// Commercial At (`@`).
    CommercialAt = 64,
    /// Uppercase A (`A`).
    CapitalA = 65,
    /// Uppercase B (`B`).
    CapitalB = 66,
    /// Uppercase C (`C`).
    CapitalC = 67,
    /// Uppercase D (`D`).
    CapitalD = 68,
    /// Uppercase E (`E`).
    CapitalE = 69,
    /// Uppercase F (`F`).
    CapitalF = 70,
    /// Uppercase G (`G`).
    CapitalG = 71,
    /// Uppercase H (`H`).
    CapitalH = 72,
    /// Uppercase I (`I`).
    CapitalI = 73,
    /// Uppercase J (`J`).
    CapitalJ = 74,
    /// Uppercase K (`K`).
    CapitalK = 75,
    /// Uppercase L (`L`).
    CapitalL = 76,
    /// Uppercase M (`M`).
    CapitalM = 77,
    /// Uppercase N (`N`).
    CapitalN = 78,
    /// Uppercase O (`O`).
    CapitalO = 79,
    /// Uppercase P (`P`).
    CapitalP = 80,
    /// Uppercase Q (`Q`).
    CapitalQ = 81,
    /// Uppercase R (`R`).
    CapitalR = 82,
    /// Uppercase S (`S`).
    CapitalS = 83,
    /// Uppercase T (`T`).
    CapitalT = 84,
    /// Uppercase U (`U`).
    CapitalU = 85,
    /// Uppercase V (`V`).
    CapitalV = 86,
    /// Uppercase W (`W`).
    CapitalW = 87,
    /// Uppercase X (`X`).
    CapitalX = 88,
    /// Uppercase Y (`Y`).
    CapitalY = 89,
    /// Uppercase Z (`Z`).
    CapitalZ = 90,
    /// Left Square Bracket or Opening Bracket (`[`).
    LeftSquareBracket = 91,
    /// Reverse Solidus or Backslash (`\`).
    ReverseSolidus = 92,
    /// Right Square Bracket or Closing Bracket (`]`).
    RightSquareBracket = 93,
    /// Circumflex Accent or Caret (`^`).
    CircumflexAccent = 94,
    /// Low Line or Underscore (`_`).
    LowLine = 95,
    /// Grave Accent (`).
    GraveAccent = 96,
    /// Lowercase A (`a`).
    SmallA = 97,
    /// Lowercase B (`b`).
    SmallB = 98,
    /// Lowercase C (`c`).
    SmallC = 99,
    /// Lowercase D (`d`).
    SmallD = 100,
    /// Lowercase E (`e`).
    SmallE = 101,
    /// Lowercase F (`f`).
    SmallF = 102,
    /// Lowercase G (`g`).
    SmallG = 103,
    /// Lowercase H (`h`).
    SmallH = 104,
    /// Lowercase I (`i`).
    SmallI = 105,
    /// Lowercase J (`j`).
    SmallJ = 106,
    /// Lowercase K (`k`).
    SmallK = 107,
    /// Lowercase L (`l`).
    SmallL = 108,
    /// Lowercase M (`m`).
    SmallM = 109,
    /// Lowercase N (`n`).
    SmallN = 110,
    /// Lowercase O (`o`).
    SmallO = 111,
    /// Lowercase P (`p`).
    SmallP = 112,
    /// Lowercase Q (`q`).
    SmallQ = 113,
    /// Lowercase R (`r`).
    SmallR = 114,
    /// Lowercase S (`s`).
    SmallS = 115,
    /// Lowercase T (`t`).
    SmallT = 116,
    /// Lowercase U (`u`).
    SmallU = 117,
    /// Lowercase V (`v`).
    SmallV = 118,
    /// Lowercase W (`w`).
    SmallW = 119,
    /// Lowercase X (`x`).
    SmallX = 120,
    /// Lowercase Y (`y`).
    SmallY = 121,
    /// Lowercase Z (`z`).
    SmallZ = 122,
    /// Left Curly Bracket or Opening brace (`{`).
    LeftCurlyBracket = 123,
    /// Vertical Line (`|`).
    VerticalLine = 124,
    /// Right Curly Bracket or Closing brace (`}`).
    RightCurlyBracket = 125,
    /// Tilde or Equivalency Sign (`~`).
    Tilde = 126,
    /// Delete.
    Delete = 127,
}

/// One of the 127 Unicode characters from U+0001 through U+007F,
/// often known as the [ASCII] subset.
///
/// This is a non-null version of the [`Ascii`] type. See the type documentation for details.
///
/// [ASCII]: https://www.unicode.org/glossary/index.html#ASCII
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum NonNullAscii {
    /// Start of Heading (control code) `0x01`.
    StartOfHeading = 1,
    /// Start of Text (control code) `0x02`.
    StartOfText = 2,
    /// End of Text (control code) `0x03`.
    EndOfText = 3,
    /// End of Transmission (control code) `0x04`.
    EndOfTransmission = 4,
    /// Enquiry (control code) `0x05`.
    Enquiry = 5,
    /// Acknowledge (control code) `0x06`.
    Acknowledge = 6,
    /// Bell (control code) `0x06`.
    Bell = 7,
    /// Backspace (control code) `0x08`.
    Backspace = 8,
    /// Horizontal Tab (control code) `0x09`.
    CharacterTabulation = 9,
    /// Line Feed (control code) `0x0A`.
    LineFeed = 10,
    /// Vertical Tabulation (control code) `0x0B`.
    LineTabulation = 11,
    /// Form Feed (control code) `0x0C`.
    FormFeed = 12,
    /// Carriage Return (control code) `0x0D`.
    CarriageReturn = 13,
    /// Shift Out (control code) `0x0E`.
    ShiftOut = 14,
    /// Shift In (control code) `0x0F`.
    ShiftIn = 15,
    /// Data Link Escape (control code) `0x10`.
    DataLinkEscape = 16,
    /// Device Control One (control code) `0x11`.
    DeviceControlOne = 17,
    /// Device Control Two (control code) `0x12`.
    DeviceControlTwo = 18,
    /// Device Control Three (control code) `0x13`.
    DeviceControlThree = 19,
    /// Device Control Four (control code) `0x14`.
    DeviceControlFour = 20,
    /// Negative Acknowledge (control code) `0x15`.
    NegativeAcknowledge = 21,
    /// Synchronous Idle (control code) `0x16`.
    SynchronousIdle = 22,
    /// End of Transmission Block (control code) `0x17`.
    EndOfTransmissionBlock = 23,
    /// Cancel (control code) `0x18`.
    Cancel = 24,
    /// End of Medium (control code) `0x19`.
    EndOfMedium = 25,
    /// Substitute (control code) `0x1A`.
    Substitute = 26,
    /// Escape (control code) `0x1B`.
    Escape = 27,
    /// Information Separator Four or File Separator (control code) `0x1C`.
    InformationSeparatorFour = 28,
    /// Information Separator Three or Group Separator (control code) `0x1D`.
    InformationSeparatorThree = 29,
    /// Information Separator Two or Record Separator (control code) `0x1E`.
    InformationSeparatorTwo = 30,
    /// Information Separator One or Unit Separator (control code) `0x1F`.
    InformationSeparatorOne = 31,
    /// Space (` `).
    Space = 32,
    /// Exclamation Mark (`!`).
    ExclamationMark = 33,
    /// Quotation Mark or Double quotes (`"`).
    QuotationMark = 34,
    /// Number Sign (`#`).
    NumberSign = 35,
    /// Dollar Sign (`$`).
    DollarSign = 36,
    /// Percent Sign (`%`).
    PercentSign = 37,
    /// Ampersand (`&`).
    Ampersand = 38,
    /// Apostrophe or Single quotes (`'`).
    Apostrophe = 39,
    /// Left Parenthesis (`(`).
    LeftParenthesis = 40,
    /// Right Parenthesis (`)`).
    RightParenthesis = 41,
    /// Asterisk (`*`).
    Asterisk = 42,
    /// Plus sign (`+`).
    PlusSign = 43,
    /// Comma (`,`).
    Comma = 44,
    /// Hyphen-Minus (`-`).
    HyphenMinus = 45,
    /// Period, Dot or Full Stop (`.`).
    FullStop = 46,
    /// Slash, Devide or Solidus (`/`).
    Solidus = 47,
    /// Digit Zero (`0`).
    Digit0 = 48,
    /// Digit One (`1`).
    Digit1 = 49,
    /// Digit Two (`2`).
    Digit2 = 50,
    /// Digit Three (`3`).
    Digit3 = 51,
    /// Digit Four (`4`).
    Digit4 = 52,
    /// Digit Five (`5`).
    Digit5 = 53,
    /// Digit Six (`6`).
    Digit6 = 54,
    /// Digit Seven (`7`).
    Digit7 = 55,
    /// Digit Eight (`8`).
    Digit8 = 56,
    /// Digit Nine (`9`).
    Digit9 = 57,
    /// Colon (`:`).
    Colon = 58,
    /// Semicolon (`;`).
    Semicolon = 59,
    /// Less Than Sign or Open Angle Bracket (`<`).
    LessThanSign = 60,
    /// Equals Sign (`=`).
    EqualsSign = 61,
    /// Greater Than Sign or Close Angle Bracket (`>`).
    GreaterThanSign = 62,
    /// Question Mark (`?`).
    QuestionMark = 63,
    /// Commercial At (`@`).
    CommercialAt = 64,
    /// Uppercase A (`A`).
    CapitalA = 65,
    /// Uppercase B (`B`).
    CapitalB = 66,
    /// Uppercase C (`C`).
    CapitalC = 67,
    /// Uppercase D (`D`).
    CapitalD = 68,
    /// Uppercase E (`E`).
    CapitalE = 69,
    /// Uppercase F (`F`).
    CapitalF = 70,
    /// Uppercase G (`G`).
    CapitalG = 71,
    /// Uppercase H (`H`).
    CapitalH = 72,
    /// Uppercase I (`I`).
    CapitalI = 73,
    /// Uppercase J (`J`).
    CapitalJ = 74,
    /// Uppercase K (`K`).
    CapitalK = 75,
    /// Uppercase L (`L`).
    CapitalL = 76,
    /// Uppercase M (`M`).
    CapitalM = 77,
    /// Uppercase N (`N`).
    CapitalN = 78,
    /// Uppercase O (`O`).
    CapitalO = 79,
    /// Uppercase P (`P`).
    CapitalP = 80,
    /// Uppercase Q (`Q`).
    CapitalQ = 81,
    /// Uppercase R (`R`).
    CapitalR = 82,
    /// Uppercase S (`S`).
    CapitalS = 83,
    /// Uppercase T (`T`).
    CapitalT = 84,
    /// Uppercase U (`U`).
    CapitalU = 85,
    /// Uppercase V (`V`).
    CapitalV = 86,
    /// Uppercase W (`W`).
    CapitalW = 87,
    /// Uppercase X (`X`).
    CapitalX = 88,
    /// Uppercase Y (`Y`).
    CapitalY = 89,
    /// Uppercase Z (`Z`).
    CapitalZ = 90,
    /// Left Square Bracket or Opening Bracket (`[`).
    LeftSquareBracket = 91,
    /// Reverse Solidus or Backslash (`\`).
    ReverseSolidus = 92,
    /// Right Square Bracket or Closing Bracket (`]`).
    RightSquareBracket = 93,
    /// Circumflex Accent or Caret (`^`).
    CircumflexAccent = 94,
    /// Low Line or Underscore (`_`).
    LowLine = 95,
    /// Grave Accent (`).
    GraveAccent = 96,
    /// Lowercase A (`a`).
    SmallA = 97,
    /// Lowercase B (`b`).
    SmallB = 98,
    /// Lowercase C (`c`).
    SmallC = 99,
    /// Lowercase D (`d`).
    SmallD = 100,
    /// Lowercase E (`e`).
    SmallE = 101,
    /// Lowercase F (`f`).
    SmallF = 102,
    /// Lowercase G (`g`).
    SmallG = 103,
    /// Lowercase H (`h`).
    SmallH = 104,
    /// Lowercase I (`i`).
    SmallI = 105,
    /// Lowercase J (`j`).
    SmallJ = 106,
    /// Lowercase K (`k`).
    SmallK = 107,
    /// Lowercase L (`l`).
    SmallL = 108,
    /// Lowercase M (`m`).
    SmallM = 109,
    /// Lowercase N (`n`).
    SmallN = 110,
    /// Lowercase O (`o`).
    SmallO = 111,
    /// Lowercase P (`p`).
    SmallP = 112,
    /// Lowercase Q (`q`).
    SmallQ = 113,
    /// Lowercase R (`r`).
    SmallR = 114,
    /// Lowercase S (`s`).
    SmallS = 115,
    /// Lowercase T (`t`).
    SmallT = 116,
    /// Lowercase U (`u`).
    SmallU = 117,
    /// Lowercase V (`v`).
    SmallV = 118,
    /// Lowercase W (`w`).
    SmallW = 119,
    /// Lowercase X (`x`).
    SmallX = 120,
    /// Lowercase Y (`y`).
    SmallY = 121,
    /// Lowercase Z (`z`).
    SmallZ = 122,
    /// Left Curly Bracket or Opening brace (`{`).
    LeftCurlyBracket = 123,
    /// Vertical Line (`|`).
    VerticalLine = 124,
    /// Right Curly Bracket or Closing brace (`}`).
    RightCurlyBracket = 125,
    /// Tilde or Equivalency Sign (`~`).
    Tilde = 126,
    /// Delete.
    Delete = 127,
}

impl Ascii {
    /// Minimum [Ascii] value (`\0`).
    pub const MIN: Ascii = Ascii::Null;

    /// Maximum [Ascii] value.
    pub const MAX: Ascii = Ascii::Delete;

    /// Creates a [`Ascii`] character from a byte,
    /// or returns `None` if it's not ASCII.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::Ascii;
    /// assert_eq!(Ascii::from_u8(99).map(Ascii::to_char), Some('c'));
    /// assert_eq!(Ascii::from_u8(221).map(Ascii::to_char), None);
    /// ```
    #[inline]
    pub const fn from_u8(b: u8) -> Option<Self> {
        if b <= Self::MAX.to_u8() {
            // SAFETY: Just checked that `b` is in-range
            Some(unsafe { Self::from_u8_unchecked(b) })
        } else {
            None
        }
    }

    /// Creates a [`Ascii`] character from a [char],
    /// or returns `None` if it's not ASCII.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::Ascii;
    /// assert_eq!(Ascii::from_char('a'), Some(Ascii::SmallA));
    /// assert_eq!(Ascii::from_char('\0'), Some(Ascii::Null));
    /// assert_eq!(Ascii::from_char('😀'), None);
    /// ```
    #[inline]
    pub const fn from_char(ch: char) -> Option<Self> {
        if !ch.is_ascii() {
            return None;
        }

        // SAFETY: `ch` is a valid ASCII character as checked above.
        Some(unsafe { Self::from_u8_unchecked(ch as u8) })
    }

    /// Creates a [`Ascii`] character from a byte, without checking whether it's valid.
    ///
    /// # Safety
    ///
    /// Byte must be in `0..=127`, or else this is UB.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::Ascii;
    /// let ascii = unsafe { Ascii::from_u8_unchecked(99) };
    /// assert_eq!(ascii.to_char(), 'c');
    /// ```
    pub const unsafe fn from_u8_unchecked(b: u8) -> Self {
        // SAFETY: Caller enforced
        unsafe { core::mem::transmute::<u8, Self>(b) }
    }

    /// Views this [`Ascii`] character as a one-code-unit UTF-8 `str`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::Ascii;
    /// let ascii = unsafe { Ascii::from_u8_unchecked(65) };
    /// assert_eq!(ascii.as_str(), "A");
    /// ```
    pub const fn as_str(&self) -> &str {
        let slice = core::slice::from_ref(self);
        // SAFETY: a `&[Ascii]` has the same
        // representation of a `&str`
        unsafe { core::mem::transmute::<&[Self], &str>(slice) }
    }

    /// Gets this [`Ascii`] character as a byte.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::Ascii;
    /// assert_eq!(Ascii::CapitalA.to_u8(), 65);
    /// ```
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Gets this [`Ascii`] character as a `char` Unicode Scalar Value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::Ascii;
    /// assert_eq!(Ascii::CapitalA.to_char(), 'A');
    /// ```
    pub const fn to_char(self) -> char {
        self as u8 as char
    }
}

impl NonNullAscii {
    /// Minimum [NonNullAscii] value.
    pub const MIN: NonNullAscii = NonNullAscii::StartOfHeading;

    /// Maximum [NonNullAscii] value.
    pub const MAX: NonNullAscii = NonNullAscii::Delete;

    /// Creates [`NonNullAscii`] character from a byte,
    /// or returns `None` if it's too large.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::NonNullAscii;
    /// assert_eq!(NonNullAscii::from_u8(97).map(NonNullAscii::to_char), Some('a'));
    /// assert_eq!(NonNullAscii::from_u8(0).map(NonNullAscii::to_char), None);
    /// ```
    #[inline]
    pub const fn from_u8(b: u8) -> Option<Self> {
        if b != 0 && b <= Self::MAX.to_u8() {
            // SAFETY: Just checked that `b` is in-range
            Some(unsafe { Self::from_u8_unchecked(b) })
        } else {
            None
        }
    }

    /// Creates a [`NonNullAscii`] character from a [char],
    /// or returns `None` if it's null (`\0`) or not ASCII.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::NonNullAscii;
    /// assert_eq!(NonNullAscii::from_char('a'), Some(NonNullAscii::SmallA));
    /// assert_eq!(NonNullAscii::from_char('\0'), None);
    /// assert_eq!(NonNullAscii::from_char('😀'), None);
    /// ```
    #[inline]
    pub const fn from_char(ch: char) -> Option<Self> {
        if ch == '\0' || !ch.is_ascii() {
            return None;
        }

        // SAFETY: `ch` is a valid non-null ASCII character as checked above.
        Some(unsafe { Self::from_u8_unchecked(ch as u8) })
    }

    /// Creates a [`NonNullAscii`] character from a byte, without checking whether it's valid.
    ///
    /// # Safety
    ///
    /// Byte must be in `1..=127`, or else this is UB.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::NonNullAscii;
    /// let ascii = unsafe { NonNullAscii::from_u8_unchecked(97) };
    /// assert_eq!(ascii.to_char(), 'a');
    /// ```
    pub const unsafe fn from_u8_unchecked(b: u8) -> Self {
        // SAFETY: Caller enforced
        unsafe { core::mem::transmute::<u8, Self>(b) }
    }

    /// Views this [`NonNullAscii`] character as a one-code-unit UTF-8 `str`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::NonNullAscii;
    /// let ascii = unsafe { NonNullAscii::from_u8_unchecked(65) };
    /// assert_eq!(ascii.as_str(), "A");
    /// ```
    pub const fn as_str(&self) -> &str {
        let slice = core::slice::from_ref(self);
        // SAFETY: a `&[NonNullAscii]` has the same
        // representation of a `&str`
        unsafe { core::mem::transmute::<&[Self], &str>(slice) }
    }

    /// Gets this [`NonNullAscii`] character as a byte.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::NonNullAscii;
    /// assert_eq!(NonNullAscii::CapitalA.to_u8(), 65);
    /// ```
    pub const fn to_u8(self) -> u8 {
        self as u8
    }

    /// Gets this [`NonNullAscii`] character as a `char` Unicode Scalar Value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::NonNullAscii;
    /// assert_eq!(NonNullAscii::CapitalA.to_char(), 'A');
    /// ```
    pub const fn to_char(self) -> char {
        self as u8 as char
    }

    /// Gets this [`NonNullAscii`] character as an [`Ascii`] character.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{NonNullAscii, Ascii};
    /// assert_eq!(NonNullAscii::QuestionMark.to_ascii(), Ascii::QuestionMark);
    /// ```
    pub const fn to_ascii(self) -> Ascii {
        // SAFETY: `Ascii` is a super set of `NonNullAscii` with the
        // same representation.
        unsafe { core::mem::transmute::<Self, Ascii>(self) }
    }
}
