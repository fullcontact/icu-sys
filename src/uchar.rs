#![allow(unused_imports)]
/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
use libc;
use libc::*;
use utypes::*;
use ustring::*;
use utf8::*;
use utf16::*;
use uscript::*;
use uset::*;
use ucnv::*;
use uloc::*;
use ures::*;
use unorm2::*;
use ucal::*;
use udat::*;
use unum::*;
use utrans::*;
use ubidi::*;
use ushape::*;
use ucol::*;
use usearch::*;
use ubrk::*;
use uregex::*;
use usprep::*;
use uidna::*;
use uspoof::*;
use utmscale::*;
use umachine::*;
use parseerr::*;
use utext::*;
use uversion::*;
use uiter::*;
use uenum::*;
use urep::*;
use udisplaycontext::*;
use uformattable::*;
use umisc::*;
use ufieldpositer::*;
use ucnv_err::*;
use ucasemap::*;
use udata::*;

pub const UCHAR_BINARY_START: UProperty = UProperty::UCHAR_ALPHABETIC;
pub const UCHAR_INT_START: UProperty = UProperty::UCHAR_BIDI_CLASS;
pub const UCHAR_MASK_START: UProperty =
    UProperty::UCHAR_GENERAL_CATEGORY_MASK;
pub const UCHAR_DOUBLE_START: UProperty = UProperty::UCHAR_NUMERIC_VALUE;
pub const UCHAR_STRING_START: UProperty = UProperty::UCHAR_AGE;
pub const UCHAR_OTHER_PROPERTY_START: UProperty =
    UProperty::UCHAR_SCRIPT_EXTENSIONS;
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UProperty {
    UCHAR_ALPHABETIC = 0,
    UCHAR_ASCII_HEX_DIGIT = 1,
    UCHAR_BIDI_CONTROL = 2,
    UCHAR_BIDI_MIRRORED = 3,
    UCHAR_DASH = 4,
    UCHAR_DEFAULT_IGNORABLE_CODE_POINT = 5,
    UCHAR_DEPRECATED = 6,
    UCHAR_DIACRITIC = 7,
    UCHAR_EXTENDER = 8,
    UCHAR_FULL_COMPOSITION_EXCLUSION = 9,
    UCHAR_GRAPHEME_BASE = 10,
    UCHAR_GRAPHEME_EXTEND = 11,
    UCHAR_GRAPHEME_LINK = 12,
    UCHAR_HEX_DIGIT = 13,
    UCHAR_HYPHEN = 14,
    UCHAR_ID_CONTINUE = 15,
    UCHAR_ID_START = 16,
    UCHAR_IDEOGRAPHIC = 17,
    UCHAR_IDS_BINARY_OPERATOR = 18,
    UCHAR_IDS_TRINARY_OPERATOR = 19,
    UCHAR_JOIN_CONTROL = 20,
    UCHAR_LOGICAL_ORDER_EXCEPTION = 21,
    UCHAR_LOWERCASE = 22,
    UCHAR_MATH = 23,
    UCHAR_NONCHARACTER_CODE_POINT = 24,
    UCHAR_QUOTATION_MARK = 25,
    UCHAR_RADICAL = 26,
    UCHAR_SOFT_DOTTED = 27,
    UCHAR_TERMINAL_PUNCTUATION = 28,
    UCHAR_UNIFIED_IDEOGRAPH = 29,
    UCHAR_UPPERCASE = 30,
    UCHAR_WHITE_SPACE = 31,
    UCHAR_XID_CONTINUE = 32,
    UCHAR_XID_START = 33,
    UCHAR_CASE_SENSITIVE = 34,
    UCHAR_S_TERM = 35,
    UCHAR_VARIATION_SELECTOR = 36,
    UCHAR_NFD_INERT = 37,
    UCHAR_NFKD_INERT = 38,
    UCHAR_NFC_INERT = 39,
    UCHAR_NFKC_INERT = 40,
    UCHAR_SEGMENT_STARTER = 41,
    UCHAR_PATTERN_SYNTAX = 42,
    UCHAR_PATTERN_WHITE_SPACE = 43,
    UCHAR_POSIX_ALNUM = 44,
    UCHAR_POSIX_BLANK = 45,
    UCHAR_POSIX_GRAPH = 46,
    UCHAR_POSIX_PRINT = 47,
    UCHAR_POSIX_XDIGIT = 48,
    UCHAR_CASED = 49,
    UCHAR_CASE_IGNORABLE = 50,
    UCHAR_CHANGES_WHEN_LOWERCASED = 51,
    UCHAR_CHANGES_WHEN_UPPERCASED = 52,
    UCHAR_CHANGES_WHEN_TITLECASED = 53,
    UCHAR_CHANGES_WHEN_CASEFOLDED = 54,
    UCHAR_CHANGES_WHEN_CASEMAPPED = 55,
    UCHAR_CHANGES_WHEN_NFKC_CASEFOLDED = 56,
    UCHAR_BINARY_LIMIT = 57,
    UCHAR_BIDI_CLASS = 4096,
    UCHAR_BLOCK = 4097,
    UCHAR_CANONICAL_COMBINING_CLASS = 4098,
    UCHAR_DECOMPOSITION_TYPE = 4099,
    UCHAR_EAST_ASIAN_WIDTH = 4100,
    UCHAR_GENERAL_CATEGORY = 4101,
    UCHAR_JOINING_GROUP = 4102,
    UCHAR_JOINING_TYPE = 4103,
    UCHAR_LINE_BREAK = 4104,
    UCHAR_NUMERIC_TYPE = 4105,
    UCHAR_SCRIPT = 4106,
    UCHAR_HANGUL_SYLLABLE_TYPE = 4107,
    UCHAR_NFD_QUICK_CHECK = 4108,
    UCHAR_NFKD_QUICK_CHECK = 4109,
    UCHAR_NFC_QUICK_CHECK = 4110,
    UCHAR_NFKC_QUICK_CHECK = 4111,
    UCHAR_LEAD_CANONICAL_COMBINING_CLASS = 4112,
    UCHAR_TRAIL_CANONICAL_COMBINING_CLASS = 4113,
    UCHAR_GRAPHEME_CLUSTER_BREAK = 4114,
    UCHAR_SENTENCE_BREAK = 4115,
    UCHAR_WORD_BREAK = 4116,
    UCHAR_BIDI_PAIRED_BRACKET_TYPE = 4117,
    UCHAR_INT_LIMIT = 4118,
    UCHAR_GENERAL_CATEGORY_MASK = 8192,
    UCHAR_MASK_LIMIT = 8193,
    UCHAR_NUMERIC_VALUE = 12288,
    UCHAR_DOUBLE_LIMIT = 12289,
    UCHAR_AGE = 16384,
    UCHAR_BIDI_MIRRORING_GLYPH = 16385,
    UCHAR_CASE_FOLDING = 16386,
    UCHAR_ISO_COMMENT = 16387,
    UCHAR_LOWERCASE_MAPPING = 16388,
    UCHAR_NAME = 16389,
    UCHAR_SIMPLE_CASE_FOLDING = 16390,
    UCHAR_SIMPLE_LOWERCASE_MAPPING = 16391,
    UCHAR_SIMPLE_TITLECASE_MAPPING = 16392,
    UCHAR_SIMPLE_UPPERCASE_MAPPING = 16393,
    UCHAR_TITLECASE_MAPPING = 16394,
    UCHAR_UNICODE_1_NAME = 16395,
    UCHAR_UPPERCASE_MAPPING = 16396,
    UCHAR_BIDI_PAIRED_BRACKET = 16397,
    UCHAR_STRING_LIMIT = 16398,
    UCHAR_SCRIPT_EXTENSIONS = 28672,
    UCHAR_OTHER_PROPERTY_LIMIT = 28673,
    UCHAR_INVALID_CODE = -1,
}
pub const U_GENERAL_OTHER_TYPES: UCharCategory = UCharCategory::U_UNASSIGNED;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UCharCategory {
    U_UNASSIGNED = 0,
    U_UPPERCASE_LETTER = 1,
    U_LOWERCASE_LETTER = 2,
    U_TITLECASE_LETTER = 3,
    U_MODIFIER_LETTER = 4,
    U_OTHER_LETTER = 5,
    U_NON_SPACING_MARK = 6,
    U_ENCLOSING_MARK = 7,
    U_COMBINING_SPACING_MARK = 8,
    U_DECIMAL_DIGIT_NUMBER = 9,
    U_LETTER_NUMBER = 10,
    U_OTHER_NUMBER = 11,
    U_SPACE_SEPARATOR = 12,
    U_LINE_SEPARATOR = 13,
    U_PARAGRAPH_SEPARATOR = 14,
    U_CONTROL_CHAR = 15,
    U_FORMAT_CHAR = 16,
    U_PRIVATE_USE_CHAR = 17,
    U_SURROGATE = 18,
    U_DASH_PUNCTUATION = 19,
    U_START_PUNCTUATION = 20,
    U_END_PUNCTUATION = 21,
    U_CONNECTOR_PUNCTUATION = 22,
    U_OTHER_PUNCTUATION = 23,
    U_MATH_SYMBOL = 24,
    U_CURRENCY_SYMBOL = 25,
    U_MODIFIER_SYMBOL = 26,
    U_OTHER_SYMBOL = 27,
    U_INITIAL_PUNCTUATION = 28,
    U_FINAL_PUNCTUATION = 29,
    U_CHAR_CATEGORY_COUNT = 30,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UCharDirection {
    U_LEFT_TO_RIGHT = 0,
    U_RIGHT_TO_LEFT = 1,
    U_EUROPEAN_NUMBER = 2,
    U_EUROPEAN_NUMBER_SEPARATOR = 3,
    U_EUROPEAN_NUMBER_TERMINATOR = 4,
    U_ARABIC_NUMBER = 5,
    U_COMMON_NUMBER_SEPARATOR = 6,
    U_BLOCK_SEPARATOR = 7,
    U_SEGMENT_SEPARATOR = 8,
    U_WHITE_SPACE_NEUTRAL = 9,
    U_OTHER_NEUTRAL = 10,
    U_LEFT_TO_RIGHT_EMBEDDING = 11,
    U_LEFT_TO_RIGHT_OVERRIDE = 12,
    U_RIGHT_TO_LEFT_ARABIC = 13,
    U_RIGHT_TO_LEFT_EMBEDDING = 14,
    U_RIGHT_TO_LEFT_OVERRIDE = 15,
    U_POP_DIRECTIONAL_FORMAT = 16,
    U_DIR_NON_SPACING_MARK = 17,
    U_BOUNDARY_NEUTRAL = 18,
    U_FIRST_STRONG_ISOLATE = 19,
    U_LEFT_TO_RIGHT_ISOLATE = 20,
    U_RIGHT_TO_LEFT_ISOLATE = 21,
    U_POP_DIRECTIONAL_ISOLATE = 22,
    U_CHAR_DIRECTION_COUNT = 23,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UBidiPairedBracketType {
    U_BPT_NONE = 0,
    U_BPT_OPEN = 1,
    U_BPT_CLOSE = 2,
    U_BPT_COUNT = 3,
}
pub const UBLOCK_PRIVATE_USE: UBlockCode =
    UBlockCode::UBLOCK_PRIVATE_USE_AREA;
pub const UBLOCK_CYRILLIC_SUPPLEMENTARY: UBlockCode =
    UBlockCode::UBLOCK_CYRILLIC_SUPPLEMENT;
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UBlockCode {
    UBLOCK_NO_BLOCK = 0,
    UBLOCK_BASIC_LATIN = 1,
    UBLOCK_LATIN_1_SUPPLEMENT = 2,
    UBLOCK_LATIN_EXTENDED_A = 3,
    UBLOCK_LATIN_EXTENDED_B = 4,
    UBLOCK_IPA_EXTENSIONS = 5,
    UBLOCK_SPACING_MODIFIER_LETTERS = 6,
    UBLOCK_COMBINING_DIACRITICAL_MARKS = 7,
    UBLOCK_GREEK = 8,
    UBLOCK_CYRILLIC = 9,
    UBLOCK_ARMENIAN = 10,
    UBLOCK_HEBREW = 11,
    UBLOCK_ARABIC = 12,
    UBLOCK_SYRIAC = 13,
    UBLOCK_THAANA = 14,
    UBLOCK_DEVANAGARI = 15,
    UBLOCK_BENGALI = 16,
    UBLOCK_GURMUKHI = 17,
    UBLOCK_GUJARATI = 18,
    UBLOCK_ORIYA = 19,
    UBLOCK_TAMIL = 20,
    UBLOCK_TELUGU = 21,
    UBLOCK_KANNADA = 22,
    UBLOCK_MALAYALAM = 23,
    UBLOCK_SINHALA = 24,
    UBLOCK_THAI = 25,
    UBLOCK_LAO = 26,
    UBLOCK_TIBETAN = 27,
    UBLOCK_MYANMAR = 28,
    UBLOCK_GEORGIAN = 29,
    UBLOCK_HANGUL_JAMO = 30,
    UBLOCK_ETHIOPIC = 31,
    UBLOCK_CHEROKEE = 32,
    UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS = 33,
    UBLOCK_OGHAM = 34,
    UBLOCK_RUNIC = 35,
    UBLOCK_KHMER = 36,
    UBLOCK_MONGOLIAN = 37,
    UBLOCK_LATIN_EXTENDED_ADDITIONAL = 38,
    UBLOCK_GREEK_EXTENDED = 39,
    UBLOCK_GENERAL_PUNCTUATION = 40,
    UBLOCK_SUPERSCRIPTS_AND_SUBSCRIPTS = 41,
    UBLOCK_CURRENCY_SYMBOLS = 42,
    UBLOCK_COMBINING_MARKS_FOR_SYMBOLS = 43,
    UBLOCK_LETTERLIKE_SYMBOLS = 44,
    UBLOCK_NUMBER_FORMS = 45,
    UBLOCK_ARROWS = 46,
    UBLOCK_MATHEMATICAL_OPERATORS = 47,
    UBLOCK_MISCELLANEOUS_TECHNICAL = 48,
    UBLOCK_CONTROL_PICTURES = 49,
    UBLOCK_OPTICAL_CHARACTER_RECOGNITION = 50,
    UBLOCK_ENCLOSED_ALPHANUMERICS = 51,
    UBLOCK_BOX_DRAWING = 52,
    UBLOCK_BLOCK_ELEMENTS = 53,
    UBLOCK_GEOMETRIC_SHAPES = 54,
    UBLOCK_MISCELLANEOUS_SYMBOLS = 55,
    UBLOCK_DINGBATS = 56,
    UBLOCK_BRAILLE_PATTERNS = 57,
    UBLOCK_CJK_RADICALS_SUPPLEMENT = 58,
    UBLOCK_KANGXI_RADICALS = 59,
    UBLOCK_IDEOGRAPHIC_DESCRIPTION_CHARACTERS = 60,
    UBLOCK_CJK_SYMBOLS_AND_PUNCTUATION = 61,
    UBLOCK_HIRAGANA = 62,
    UBLOCK_KATAKANA = 63,
    UBLOCK_BOPOMOFO = 64,
    UBLOCK_HANGUL_COMPATIBILITY_JAMO = 65,
    UBLOCK_KANBUN = 66,
    UBLOCK_BOPOMOFO_EXTENDED = 67,
    UBLOCK_ENCLOSED_CJK_LETTERS_AND_MONTHS = 68,
    UBLOCK_CJK_COMPATIBILITY = 69,
    UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_A = 70,
    UBLOCK_CJK_UNIFIED_IDEOGRAPHS = 71,
    UBLOCK_YI_SYLLABLES = 72,
    UBLOCK_YI_RADICALS = 73,
    UBLOCK_HANGUL_SYLLABLES = 74,
    UBLOCK_HIGH_SURROGATES = 75,
    UBLOCK_HIGH_PRIVATE_USE_SURROGATES = 76,
    UBLOCK_LOW_SURROGATES = 77,
    UBLOCK_PRIVATE_USE_AREA = 78,
    UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS = 79,
    UBLOCK_ALPHABETIC_PRESENTATION_FORMS = 80,
    UBLOCK_ARABIC_PRESENTATION_FORMS_A = 81,
    UBLOCK_COMBINING_HALF_MARKS = 82,
    UBLOCK_CJK_COMPATIBILITY_FORMS = 83,
    UBLOCK_SMALL_FORM_VARIANTS = 84,
    UBLOCK_ARABIC_PRESENTATION_FORMS_B = 85,
    UBLOCK_SPECIALS = 86,
    UBLOCK_HALFWIDTH_AND_FULLWIDTH_FORMS = 87,
    UBLOCK_OLD_ITALIC = 88,
    UBLOCK_GOTHIC = 89,
    UBLOCK_DESERET = 90,
    UBLOCK_BYZANTINE_MUSICAL_SYMBOLS = 91,
    UBLOCK_MUSICAL_SYMBOLS = 92,
    UBLOCK_MATHEMATICAL_ALPHANUMERIC_SYMBOLS = 93,
    UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_B = 94,
    UBLOCK_CJK_COMPATIBILITY_IDEOGRAPHS_SUPPLEMENT = 95,
    UBLOCK_TAGS = 96,
    UBLOCK_CYRILLIC_SUPPLEMENT = 97,
    UBLOCK_TAGALOG = 98,
    UBLOCK_HANUNOO = 99,
    UBLOCK_BUHID = 100,
    UBLOCK_TAGBANWA = 101,
    UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_A = 102,
    UBLOCK_SUPPLEMENTAL_ARROWS_A = 103,
    UBLOCK_SUPPLEMENTAL_ARROWS_B = 104,
    UBLOCK_MISCELLANEOUS_MATHEMATICAL_SYMBOLS_B = 105,
    UBLOCK_SUPPLEMENTAL_MATHEMATICAL_OPERATORS = 106,
    UBLOCK_KATAKANA_PHONETIC_EXTENSIONS = 107,
    UBLOCK_VARIATION_SELECTORS = 108,
    UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_A = 109,
    UBLOCK_SUPPLEMENTARY_PRIVATE_USE_AREA_B = 110,
    UBLOCK_LIMBU = 111,
    UBLOCK_TAI_LE = 112,
    UBLOCK_KHMER_SYMBOLS = 113,
    UBLOCK_PHONETIC_EXTENSIONS = 114,
    UBLOCK_MISCELLANEOUS_SYMBOLS_AND_ARROWS = 115,
    UBLOCK_YIJING_HEXAGRAM_SYMBOLS = 116,
    UBLOCK_LINEAR_B_SYLLABARY = 117,
    UBLOCK_LINEAR_B_IDEOGRAMS = 118,
    UBLOCK_AEGEAN_NUMBERS = 119,
    UBLOCK_UGARITIC = 120,
    UBLOCK_SHAVIAN = 121,
    UBLOCK_OSMANYA = 122,
    UBLOCK_CYPRIOT_SYLLABARY = 123,
    UBLOCK_TAI_XUAN_JING_SYMBOLS = 124,
    UBLOCK_VARIATION_SELECTORS_SUPPLEMENT = 125,
    UBLOCK_ANCIENT_GREEK_MUSICAL_NOTATION = 126,
    UBLOCK_ANCIENT_GREEK_NUMBERS = 127,
    UBLOCK_ARABIC_SUPPLEMENT = 128,
    UBLOCK_BUGINESE = 129,
    UBLOCK_CJK_STROKES = 130,
    UBLOCK_COMBINING_DIACRITICAL_MARKS_SUPPLEMENT = 131,
    UBLOCK_COPTIC = 132,
    UBLOCK_ETHIOPIC_EXTENDED = 133,
    UBLOCK_ETHIOPIC_SUPPLEMENT = 134,
    UBLOCK_GEORGIAN_SUPPLEMENT = 135,
    UBLOCK_GLAGOLITIC = 136,
    UBLOCK_KHAROSHTHI = 137,
    UBLOCK_MODIFIER_TONE_LETTERS = 138,
    UBLOCK_NEW_TAI_LUE = 139,
    UBLOCK_OLD_PERSIAN = 140,
    UBLOCK_PHONETIC_EXTENSIONS_SUPPLEMENT = 141,
    UBLOCK_SUPPLEMENTAL_PUNCTUATION = 142,
    UBLOCK_SYLOTI_NAGRI = 143,
    UBLOCK_TIFINAGH = 144,
    UBLOCK_VERTICAL_FORMS = 145,
    UBLOCK_NKO = 146,
    UBLOCK_BALINESE = 147,
    UBLOCK_LATIN_EXTENDED_C = 148,
    UBLOCK_LATIN_EXTENDED_D = 149,
    UBLOCK_PHAGS_PA = 150,
    UBLOCK_PHOENICIAN = 151,
    UBLOCK_CUNEIFORM = 152,
    UBLOCK_CUNEIFORM_NUMBERS_AND_PUNCTUATION = 153,
    UBLOCK_COUNTING_ROD_NUMERALS = 154,
    UBLOCK_SUNDANESE = 155,
    UBLOCK_LEPCHA = 156,
    UBLOCK_OL_CHIKI = 157,
    UBLOCK_CYRILLIC_EXTENDED_A = 158,
    UBLOCK_VAI = 159,
    UBLOCK_CYRILLIC_EXTENDED_B = 160,
    UBLOCK_SAURASHTRA = 161,
    UBLOCK_KAYAH_LI = 162,
    UBLOCK_REJANG = 163,
    UBLOCK_CHAM = 164,
    UBLOCK_ANCIENT_SYMBOLS = 165,
    UBLOCK_PHAISTOS_DISC = 166,
    UBLOCK_LYCIAN = 167,
    UBLOCK_CARIAN = 168,
    UBLOCK_LYDIAN = 169,
    UBLOCK_MAHJONG_TILES = 170,
    UBLOCK_DOMINO_TILES = 171,
    UBLOCK_SAMARITAN = 172,
    UBLOCK_UNIFIED_CANADIAN_ABORIGINAL_SYLLABICS_EXTENDED = 173,
    UBLOCK_TAI_THAM = 174,
    UBLOCK_VEDIC_EXTENSIONS = 175,
    UBLOCK_LISU = 176,
    UBLOCK_BAMUM = 177,
    UBLOCK_COMMON_INDIC_NUMBER_FORMS = 178,
    UBLOCK_DEVANAGARI_EXTENDED = 179,
    UBLOCK_HANGUL_JAMO_EXTENDED_A = 180,
    UBLOCK_JAVANESE = 181,
    UBLOCK_MYANMAR_EXTENDED_A = 182,
    UBLOCK_TAI_VIET = 183,
    UBLOCK_MEETEI_MAYEK = 184,
    UBLOCK_HANGUL_JAMO_EXTENDED_B = 185,
    UBLOCK_IMPERIAL_ARAMAIC = 186,
    UBLOCK_OLD_SOUTH_ARABIAN = 187,
    UBLOCK_AVESTAN = 188,
    UBLOCK_INSCRIPTIONAL_PARTHIAN = 189,
    UBLOCK_INSCRIPTIONAL_PAHLAVI = 190,
    UBLOCK_OLD_TURKIC = 191,
    UBLOCK_RUMI_NUMERAL_SYMBOLS = 192,
    UBLOCK_KAITHI = 193,
    UBLOCK_EGYPTIAN_HIEROGLYPHS = 194,
    UBLOCK_ENCLOSED_ALPHANUMERIC_SUPPLEMENT = 195,
    UBLOCK_ENCLOSED_IDEOGRAPHIC_SUPPLEMENT = 196,
    UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_C = 197,
    UBLOCK_MANDAIC = 198,
    UBLOCK_BATAK = 199,
    UBLOCK_ETHIOPIC_EXTENDED_A = 200,
    UBLOCK_BRAHMI = 201,
    UBLOCK_BAMUM_SUPPLEMENT = 202,
    UBLOCK_KANA_SUPPLEMENT = 203,
    UBLOCK_PLAYING_CARDS = 204,
    UBLOCK_MISCELLANEOUS_SYMBOLS_AND_PICTOGRAPHS = 205,
    UBLOCK_EMOTICONS = 206,
    UBLOCK_TRANSPORT_AND_MAP_SYMBOLS = 207,
    UBLOCK_ALCHEMICAL_SYMBOLS = 208,
    UBLOCK_CJK_UNIFIED_IDEOGRAPHS_EXTENSION_D = 209,
    UBLOCK_ARABIC_EXTENDED_A = 210,
    UBLOCK_ARABIC_MATHEMATICAL_ALPHABETIC_SYMBOLS = 211,
    UBLOCK_CHAKMA = 212,
    UBLOCK_MEETEI_MAYEK_EXTENSIONS = 213,
    UBLOCK_MEROITIC_CURSIVE = 214,
    UBLOCK_MEROITIC_HIEROGLYPHS = 215,
    UBLOCK_MIAO = 216,
    UBLOCK_SHARADA = 217,
    UBLOCK_SORA_SOMPENG = 218,
    UBLOCK_SUNDANESE_SUPPLEMENT = 219,
    UBLOCK_TAKRI = 220,
    UBLOCK_BASSA_VAH = 221,
    UBLOCK_CAUCASIAN_ALBANIAN = 222,
    UBLOCK_COPTIC_EPACT_NUMBERS = 223,
    UBLOCK_COMBINING_DIACRITICAL_MARKS_EXTENDED = 224,
    UBLOCK_DUPLOYAN = 225,
    UBLOCK_ELBASAN = 226,
    UBLOCK_GEOMETRIC_SHAPES_EXTENDED = 227,
    UBLOCK_GRANTHA = 228,
    UBLOCK_KHOJKI = 229,
    UBLOCK_KHUDAWADI = 230,
    UBLOCK_LATIN_EXTENDED_E = 231,
    UBLOCK_LINEAR_A = 232,
    UBLOCK_MAHAJANI = 233,
    UBLOCK_MANICHAEAN = 234,
    UBLOCK_MENDE_KIKAKUI = 235,
    UBLOCK_MODI = 236,
    UBLOCK_MRO = 237,
    UBLOCK_MYANMAR_EXTENDED_B = 238,
    UBLOCK_NABATAEAN = 239,
    UBLOCK_OLD_NORTH_ARABIAN = 240,
    UBLOCK_OLD_PERMIC = 241,
    UBLOCK_ORNAMENTAL_DINGBATS = 242,
    UBLOCK_PAHAWH_HMONG = 243,
    UBLOCK_PALMYRENE = 244,
    UBLOCK_PAU_CIN_HAU = 245,
    UBLOCK_PSALTER_PAHLAVI = 246,
    UBLOCK_SHORTHAND_FORMAT_CONTROLS = 247,
    UBLOCK_SIDDHAM = 248,
    UBLOCK_SINHALA_ARCHAIC_NUMBERS = 249,
    UBLOCK_SUPPLEMENTAL_ARROWS_C = 250,
    UBLOCK_TIRHUTA = 251,
    UBLOCK_WARANG_CITI = 252,
    UBLOCK_COUNT = 253,
    UBLOCK_INVALID_CODE = -1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UEastAsianWidth {
    U_EA_NEUTRAL = 0,
    U_EA_AMBIGUOUS = 1,
    U_EA_HALFWIDTH = 2,
    U_EA_FULLWIDTH = 3,
    U_EA_NARROW = 4,
    U_EA_WIDE = 5,
    U_EA_COUNT = 6,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UCharNameChoice {
    U_UNICODE_CHAR_NAME = 0,
    U_UNICODE_10_CHAR_NAME = 1,
    U_EXTENDED_CHAR_NAME = 2,
    U_CHAR_NAME_ALIAS = 3,
    U_CHAR_NAME_CHOICE_COUNT = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UPropertyNameChoice {
    U_SHORT_PROPERTY_NAME = 0,
    U_LONG_PROPERTY_NAME = 1,
    U_PROPERTY_NAME_CHOICE_COUNT = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UDecompositionType {
    U_DT_NONE = 0,
    U_DT_CANONICAL = 1,
    U_DT_COMPAT = 2,
    U_DT_CIRCLE = 3,
    U_DT_FINAL = 4,
    U_DT_FONT = 5,
    U_DT_FRACTION = 6,
    U_DT_INITIAL = 7,
    U_DT_ISOLATED = 8,
    U_DT_MEDIAL = 9,
    U_DT_NARROW = 10,
    U_DT_NOBREAK = 11,
    U_DT_SMALL = 12,
    U_DT_SQUARE = 13,
    U_DT_SUB = 14,
    U_DT_SUPER = 15,
    U_DT_VERTICAL = 16,
    U_DT_WIDE = 17,
    U_DT_COUNT = 18,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UJoiningType {
    U_JT_NON_JOINING = 0,
    U_JT_JOIN_CAUSING = 1,
    U_JT_DUAL_JOINING = 2,
    U_JT_LEFT_JOINING = 3,
    U_JT_RIGHT_JOINING = 4,
    U_JT_TRANSPARENT = 5,
    U_JT_COUNT = 6,
}
pub const U_JG_HAMZA_ON_HEH_GOAL: UJoiningGroup =
    UJoiningGroup::U_JG_TEH_MARBUTA_GOAL;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UJoiningGroup {
    U_JG_NO_JOINING_GROUP = 0,
    U_JG_AIN = 1,
    U_JG_ALAPH = 2,
    U_JG_ALEF = 3,
    U_JG_BEH = 4,
    U_JG_BETH = 5,
    U_JG_DAL = 6,
    U_JG_DALATH_RISH = 7,
    U_JG_E = 8,
    U_JG_FEH = 9,
    U_JG_FINAL_SEMKATH = 10,
    U_JG_GAF = 11,
    U_JG_GAMAL = 12,
    U_JG_HAH = 13,
    U_JG_TEH_MARBUTA_GOAL = 14,
    U_JG_HE = 15,
    U_JG_HEH = 16,
    U_JG_HEH_GOAL = 17,
    U_JG_HETH = 18,
    U_JG_KAF = 19,
    U_JG_KAPH = 20,
    U_JG_KNOTTED_HEH = 21,
    U_JG_LAM = 22,
    U_JG_LAMADH = 23,
    U_JG_MEEM = 24,
    U_JG_MIM = 25,
    U_JG_NOON = 26,
    U_JG_NUN = 27,
    U_JG_PE = 28,
    U_JG_QAF = 29,
    U_JG_QAPH = 30,
    U_JG_REH = 31,
    U_JG_REVERSED_PE = 32,
    U_JG_SAD = 33,
    U_JG_SADHE = 34,
    U_JG_SEEN = 35,
    U_JG_SEMKATH = 36,
    U_JG_SHIN = 37,
    U_JG_SWASH_KAF = 38,
    U_JG_SYRIAC_WAW = 39,
    U_JG_TAH = 40,
    U_JG_TAW = 41,
    U_JG_TEH_MARBUTA = 42,
    U_JG_TETH = 43,
    U_JG_WAW = 44,
    U_JG_YEH = 45,
    U_JG_YEH_BARREE = 46,
    U_JG_YEH_WITH_TAIL = 47,
    U_JG_YUDH = 48,
    U_JG_YUDH_HE = 49,
    U_JG_ZAIN = 50,
    U_JG_FE = 51,
    U_JG_KHAPH = 52,
    U_JG_ZHAIN = 53,
    U_JG_BURUSHASKI_YEH_BARREE = 54,
    U_JG_FARSI_YEH = 55,
    U_JG_NYA = 56,
    U_JG_ROHINGYA_YEH = 57,
    U_JG_MANICHAEAN_ALEPH = 58,
    U_JG_MANICHAEAN_AYIN = 59,
    U_JG_MANICHAEAN_BETH = 60,
    U_JG_MANICHAEAN_DALETH = 61,
    U_JG_MANICHAEAN_DHAMEDH = 62,
    U_JG_MANICHAEAN_FIVE = 63,
    U_JG_MANICHAEAN_GIMEL = 64,
    U_JG_MANICHAEAN_HETH = 65,
    U_JG_MANICHAEAN_HUNDRED = 66,
    U_JG_MANICHAEAN_KAPH = 67,
    U_JG_MANICHAEAN_LAMEDH = 68,
    U_JG_MANICHAEAN_MEM = 69,
    U_JG_MANICHAEAN_NUN = 70,
    U_JG_MANICHAEAN_ONE = 71,
    U_JG_MANICHAEAN_PE = 72,
    U_JG_MANICHAEAN_QOPH = 73,
    U_JG_MANICHAEAN_RESH = 74,
    U_JG_MANICHAEAN_SADHE = 75,
    U_JG_MANICHAEAN_SAMEKH = 76,
    U_JG_MANICHAEAN_TAW = 77,
    U_JG_MANICHAEAN_TEN = 78,
    U_JG_MANICHAEAN_TETH = 79,
    U_JG_MANICHAEAN_THAMEDH = 80,
    U_JG_MANICHAEAN_TWENTY = 81,
    U_JG_MANICHAEAN_WAW = 82,
    U_JG_MANICHAEAN_YODH = 83,
    U_JG_MANICHAEAN_ZAYIN = 84,
    U_JG_STRAIGHT_WAW = 85,
    U_JG_COUNT = 86,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UGraphemeClusterBreak {
    U_GCB_OTHER = 0,
    U_GCB_CONTROL = 1,
    U_GCB_CR = 2,
    U_GCB_EXTEND = 3,
    U_GCB_L = 4,
    U_GCB_LF = 5,
    U_GCB_LV = 6,
    U_GCB_LVT = 7,
    U_GCB_T = 8,
    U_GCB_V = 9,
    U_GCB_SPACING_MARK = 10,
    U_GCB_PREPEND = 11,
    U_GCB_REGIONAL_INDICATOR = 12,
    U_GCB_COUNT = 13,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UWordBreakValues {
    U_WB_OTHER = 0,
    U_WB_ALETTER = 1,
    U_WB_FORMAT = 2,
    U_WB_KATAKANA = 3,
    U_WB_MIDLETTER = 4,
    U_WB_MIDNUM = 5,
    U_WB_NUMERIC = 6,
    U_WB_EXTENDNUMLET = 7,
    U_WB_CR = 8,
    U_WB_EXTEND = 9,
    U_WB_LF = 10,
    U_WB_MIDNUMLET = 11,
    U_WB_NEWLINE = 12,
    U_WB_REGIONAL_INDICATOR = 13,
    U_WB_HEBREW_LETTER = 14,
    U_WB_SINGLE_QUOTE = 15,
    U_WB_DOUBLE_QUOTE = 16,
    U_WB_COUNT = 17,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum USentenceBreak {
    U_SB_OTHER = 0,
    U_SB_ATERM = 1,
    U_SB_CLOSE = 2,
    U_SB_FORMAT = 3,
    U_SB_LOWER = 4,
    U_SB_NUMERIC = 5,
    U_SB_OLETTER = 6,
    U_SB_SEP = 7,
    U_SB_SP = 8,
    U_SB_STERM = 9,
    U_SB_UPPER = 10,
    U_SB_CR = 11,
    U_SB_EXTEND = 12,
    U_SB_LF = 13,
    U_SB_SCONTINUE = 14,
    U_SB_COUNT = 15,
}
pub const U_LB_INSEPERABLE: ULineBreak = ULineBreak::U_LB_INSEPARABLE;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum ULineBreak {
    U_LB_UNKNOWN = 0,
    U_LB_AMBIGUOUS = 1,
    U_LB_ALPHABETIC = 2,
    U_LB_BREAK_BOTH = 3,
    U_LB_BREAK_AFTER = 4,
    U_LB_BREAK_BEFORE = 5,
    U_LB_MANDATORY_BREAK = 6,
    U_LB_CONTINGENT_BREAK = 7,
    U_LB_CLOSE_PUNCTUATION = 8,
    U_LB_COMBINING_MARK = 9,
    U_LB_CARRIAGE_RETURN = 10,
    U_LB_EXCLAMATION = 11,
    U_LB_GLUE = 12,
    U_LB_HYPHEN = 13,
    U_LB_IDEOGRAPHIC = 14,
    U_LB_INSEPARABLE = 15,
    U_LB_INFIX_NUMERIC = 16,
    U_LB_LINE_FEED = 17,
    U_LB_NONSTARTER = 18,
    U_LB_NUMERIC = 19,
    U_LB_OPEN_PUNCTUATION = 20,
    U_LB_POSTFIX_NUMERIC = 21,
    U_LB_PREFIX_NUMERIC = 22,
    U_LB_QUOTATION = 23,
    U_LB_COMPLEX_CONTEXT = 24,
    U_LB_SURROGATE = 25,
    U_LB_SPACE = 26,
    U_LB_BREAK_SYMBOLS = 27,
    U_LB_ZWSPACE = 28,
    U_LB_NEXT_LINE = 29,
    U_LB_WORD_JOINER = 30,
    U_LB_H2 = 31,
    U_LB_H3 = 32,
    U_LB_JL = 33,
    U_LB_JT = 34,
    U_LB_JV = 35,
    U_LB_CLOSE_PARENTHESIS = 36,
    U_LB_CONDITIONAL_JAPANESE_STARTER = 37,
    U_LB_HEBREW_LETTER = 38,
    U_LB_REGIONAL_INDICATOR = 39,
    U_LB_COUNT = 40,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UNumericType {
    U_NT_NONE = 0,
    U_NT_DECIMAL = 1,
    U_NT_DIGIT = 2,
    U_NT_NUMERIC = 3,
    U_NT_COUNT = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum UHangulSyllableType {
    U_HST_NOT_APPLICABLE = 0,
    U_HST_LEADING_JAMO = 1,
    U_HST_VOWEL_JAMO = 2,
    U_HST_TRAILING_JAMO = 3,
    U_HST_LV_SYLLABLE = 4,
    U_HST_LVT_SYLLABLE = 5,
    U_HST_COUNT = 6,
}
pub type UCharEnumTypeRange =
    ::std::option::Option<unsafe extern "C" fn(context:
                                                   *const ::std::os::raw::c_void,
                                               start: UChar32, limit: UChar32,
                                               type_: UCharCategory)
                              -> UBool>;
pub type UEnumCharNamesFn =
    ::std::option::Option<unsafe extern "C" fn(context:
                                                   *mut ::std::os::raw::c_void,
                                               code: UChar32,
                                               nameChoice: UCharNameChoice,
                                               name:
                                                   *const ::std::os::raw::c_char,
                                               length: int32_t) -> UBool>;
#[link(name = "icuuc", kind = "static")]
#[link(name = "icudata", kind = "static")] 
#[link(name = "icui18n", kind = "static")] 
#[link(name = "stdc++", kind = "dylib")]
extern "C" {
    pub fn u_hasBinaryProperty(c: UChar32, which: UProperty) -> UBool;
    pub fn u_isUAlphabetic(c: UChar32) -> UBool;
    pub fn u_isULowercase(c: UChar32) -> UBool;
    pub fn u_isUUppercase(c: UChar32) -> UBool;
    pub fn u_isUWhiteSpace(c: UChar32) -> UBool;
    pub fn u_getIntPropertyValue(c: UChar32, which: UProperty) -> int32_t;
    pub fn u_getIntPropertyMinValue(which: UProperty) -> int32_t;
    pub fn u_getIntPropertyMaxValue(which: UProperty) -> int32_t;
    pub fn u_getNumericValue(c: UChar32) -> f64;
    pub fn u_islower(c: UChar32) -> UBool;
    pub fn u_isupper(c: UChar32) -> UBool;
    pub fn u_istitle(c: UChar32) -> UBool;
    pub fn u_isdigit(c: UChar32) -> UBool;
    pub fn u_isalpha(c: UChar32) -> UBool;
    pub fn u_isalnum(c: UChar32) -> UBool;
    pub fn u_isxdigit(c: UChar32) -> UBool;
    pub fn u_ispunct(c: UChar32) -> UBool;
    pub fn u_isgraph(c: UChar32) -> UBool;
    pub fn u_isblank(c: UChar32) -> UBool;
    pub fn u_isdefined(c: UChar32) -> UBool;
    pub fn u_isspace(c: UChar32) -> UBool;
    pub fn u_isJavaSpaceChar(c: UChar32) -> UBool;
    pub fn u_isWhitespace(c: UChar32) -> UBool;
    pub fn u_iscntrl(c: UChar32) -> UBool;
    pub fn u_isISOControl(c: UChar32) -> UBool;
    pub fn u_isprint(c: UChar32) -> UBool;
    pub fn u_isbase(c: UChar32) -> UBool;
    pub fn u_charDirection(c: UChar32) -> UCharDirection;
    pub fn u_isMirrored(c: UChar32) -> UBool;
    pub fn u_charMirror(c: UChar32) -> UChar32;
    pub fn u_getBidiPairedBracket(c: UChar32) -> UChar32;
    pub fn u_charType(c: UChar32) -> int8_t;
    pub fn u_enumCharTypes(enumRange: UCharEnumTypeRange,
                           context: *const ::std::os::raw::c_void);
    pub fn u_getCombiningClass(c: UChar32) -> uint8_t;
    pub fn u_charDigitValue(c: UChar32) -> int32_t;
    pub fn ublock_getCode(c: UChar32) -> UBlockCode;
    pub fn u_charName(code: UChar32, nameChoice: UCharNameChoice,
                      buffer: *mut ::std::os::raw::c_char,
                      bufferLength: int32_t, pErrorCode: *mut UErrorCode)
     -> int32_t;
    pub fn u_getISOComment(c: UChar32, dest: *mut ::std::os::raw::c_char,
                           destCapacity: int32_t, pErrorCode: *mut UErrorCode)
     -> int32_t;
    pub fn u_charFromName(nameChoice: UCharNameChoice,
                          name: *const ::std::os::raw::c_char,
                          pErrorCode: *mut UErrorCode) -> UChar32;
    pub fn u_enumCharNames(start: UChar32, limit: UChar32,
                           fn_: UEnumCharNamesFn,
                           context: *mut ::std::os::raw::c_void,
                           nameChoice: UCharNameChoice,
                           pErrorCode: *mut UErrorCode);
    pub fn u_getPropertyName(property: UProperty,
                             nameChoice: UPropertyNameChoice)
     -> *const ::std::os::raw::c_char;
    pub fn u_getPropertyEnum(alias: *const ::std::os::raw::c_char)
     -> UProperty;
    pub fn u_getPropertyValueName(property: UProperty, value: int32_t,
                                  nameChoice: UPropertyNameChoice)
     -> *const ::std::os::raw::c_char;
    pub fn u_getPropertyValueEnum(property: UProperty,
                                  alias: *const ::std::os::raw::c_char)
     -> int32_t;
    pub fn u_isIDStart(c: UChar32) -> UBool;
    pub fn u_isIDPart(c: UChar32) -> UBool;
    pub fn u_isIDIgnorable(c: UChar32) -> UBool;
    pub fn u_isJavaIDStart(c: UChar32) -> UBool;
    pub fn u_isJavaIDPart(c: UChar32) -> UBool;
    pub fn u_tolower(c: UChar32) -> UChar32;
    pub fn u_toupper(c: UChar32) -> UChar32;
    pub fn u_totitle(c: UChar32) -> UChar32;
    pub fn u_foldCase(c: UChar32, options: uint32_t) -> UChar32;
    pub fn u_digit(ch: UChar32, radix: int8_t) -> int32_t;
    pub fn u_forDigit(digit: int32_t, radix: int8_t) -> UChar32;
    pub fn u_charAge(c: UChar32, versionArray: UVersionInfo);
    pub fn u_getUnicodeVersion(versionArray: UVersionInfo);
    pub fn u_getFC_NFKC_Closure(c: UChar32, dest: *mut UChar,
                                destCapacity: int32_t,
                                pErrorCode: *mut UErrorCode) -> int32_t;
}
