pub(crate) const fn is_space(c: char) -> bool {
    c == ' '
}

pub(crate) const fn is_zenkaku_space(c: char) -> bool {
    c == '　'
}
pub(crate) const fn is_tab(c: char) -> bool {
    c == '\t'
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
    use rstest::*;

    #[gtest]
    #[rstest]
    #[case(' ', true)]
    #[case('　', false)]
    #[case('\t', false)]
    #[case('a', false)]
    #[case('あ', false)]
    fn is_space_works(#[case] c: char, #[case] expected: bool) {
        assert_that!(is_space(c), eq(expected))
    }

    #[gtest]
    #[rstest]
    #[case(' ', false)]
    #[case('　', true)]
    #[case('\t', false)]
    #[case('a', false)]
    #[case('あ', false)]
    fn is_zenkaku_space_works(#[case] c: char, #[case] expected: bool) {
        assert_that!(is_zenkaku_space(c), eq(expected))
    }

    #[gtest]
    #[rstest]
    #[case(' ', false)]
    #[case('　', false)]
    #[case('\t', true)]
    #[case('a', false)]
    #[case('あ', false)]
    fn is_tab_works(#[case] c: char, #[case] expected: bool) {
        assert_that!(is_tab(c), eq(expected))
    }
}
