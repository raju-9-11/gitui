use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

///
pub fn trim_length_left(s: &str, width: usize) -> &str {
	let len = s.len();
	if len > width {
		for i in len - width..len {
			if s.is_char_boundary(i) {
				return &s[i..];
			}
		}
	}

	s
}

///
pub fn tabs_to_spaces(input: String, tab_size: usize) -> String {
	if input.contains('\t') {
		let spaces = " ".repeat(tab_size);
		input.replace('\t', &spaces)
	} else {
		input
	}
}

/// This function will return a str slice which start at specified offset.
/// As src is a unicode str, start offset has to be calculated with each character.
pub fn trim_offset(src: &str, mut offset: usize) -> &str {
	let mut start = 0;
	for c in UnicodeSegmentation::graphemes(src, true) {
		let w = c.width();
		if w <= offset {
			offset -= w;
			start += c.len();
		} else {
			break;
		}
	}
	&src[start..]
}

#[cfg(test)]
mod test {
	use pretty_assertions::assert_eq;

	use crate::string_utils::{tabs_to_spaces, trim_length_left};

	#[test]
	fn test_trim() {
		assert_eq!(trim_length_left("👍foo", 3), "foo");
		assert_eq!(trim_length_left("👍foo", 4), "foo");
	}

	#[test]
	fn test_tabs_to_spaces() {
		assert_eq!(tabs_to_spaces("foo\tbar".into(), 2), "foo  bar");
		assert_eq!(tabs_to_spaces("foo\tbar".into(), 4), "foo    bar");
		assert_eq!(tabs_to_spaces("foobar".into(), 4), "foobar");
	}
}
