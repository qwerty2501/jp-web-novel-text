pub(crate) const fn is_space(c: char) -> bool {
    c == ' '
}

pub(crate) const fn is_zenkaku_space(c: char) -> bool {
    c == '　'
}
pub(crate) const fn is_tab(c: char) -> bool {
    c == '\t'
}

pub(crate) const fn is_start_instruction(c: char) -> bool {
    c == '|' || c == '｜'
}

pub(crate) const fn is_start_ruby(c: char) -> bool {
    c == '(' || c == '（' || c == '《' || c == '⟪'
}

pub(crate) const fn is_end_ruby(c: char) -> bool {
    c == ')' || c == '）' || c == '》' || c == '⟫'
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
    #[gtest]
    #[rstest]
    #[case(' ', false)]
    #[case('　', false)]
    #[case('\t', false)]
    #[case('a', false)]
    #[case('あ', false)]
    #[case('|', true)]
    #[case('｜', true)]
    fn is_start_instruction_works(#[case] c: char, #[case] expected: bool) {
        assert_that!(is_start_instruction(c), eq(expected))
    }

    #[gtest]
    #[rstest]
    #[case(' ', false)]
    #[case('　', false)]
    #[case('\t', false)]
    #[case('a', false)]
    #[case('あ', false)]
    #[case('|', false)]
    #[case('｜', false)]
    #[case('(', true)]
    #[case('（', true)]
    #[case('《', true)]
    #[case('⟪', true)]
    #[case(')', false)]
    #[case('）', false)]
    #[case('》', false)]
    #[case('⟫', false)]
    fn is_start_ruby_works(#[case] c: char, #[case] expected: bool) {
        assert_that!(is_start_ruby(c), eq(expected))
    }
    #[gtest]
    #[rstest]
    #[case(' ', false)]
    #[case('　', false)]
    #[case('\t', false)]
    #[case('a', false)]
    #[case('あ', false)]
    #[case('|', false)]
    #[case('｜', false)]
    #[case('(', false)]
    #[case('（', false)]
    #[case('《', false)]
    #[case('⟪', false)]
    #[case(')', true)]
    #[case('）', true)]
    #[case('》', true)]
    #[case('⟫', true)]
    fn is_end_ruby_works(#[case] c: char, #[case] expected: bool) {
        assert_that!(is_end_ruby(c), eq(expected))
    }
}
