#[inline]
pub const fn is_line_feed(c: char) -> bool {
    c == '\n'
}

#[inline]
pub const fn is_caride_retrun(c: char) -> bool {
    c == '\r'
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
    use rstest::*;

    #[rstest]
    #[case('\n', true)]
    #[case('\r', false)]
    #[case('a', false)]
    #[case('あ', false)]
    fn is_line_feed_works(#[case] c: char, #[case] expected: bool) {
        assert_that!(is_line_feed(c), eq(expected))
    }

    #[gtest]
    #[rstest]
    #[case('\n', false)]
    #[case('\r', true)]
    #[case('a', false)]
    #[case('あ', false)]
    fn is_caride_return_works(#[case] c: char, #[case] expected: bool) {
        assert_that!(is_caride_retrun(c), eq(expected))
    }
}
