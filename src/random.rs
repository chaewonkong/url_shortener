use rand::{RngExt, distr::Alphanumeric};

/// 길이 n의 무작위 Base62 문자열을 반환한다
pub fn random_string(n: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}
