pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
///
/// See [the issue](https://github.com/noxlovette/yp-module-5/issues/3) for the initial state
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().filter(|&&e| e % 2 == 0).sum()
}

/// Fixed due to miri warnings
///
/// Микрооптимизация: раньше здесь был `input.to_vec().into_boxed_slice()` —
/// лишняя аллокация и полное копирование буфера только ради подсчёта байт.
/// Считаем прямо по срезу, без копий.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|&&b| b != 0_u8).count()
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

/// Logical errors fixed; Closes [#5](https://github.com/noxlovette/yp-module-5/issues/5)
pub fn average_positive(values: &[i64]) -> f64 {
    let (sum, count) = values
        .iter()
        .filter_map(|&x| (x > 0).then_some(x))
        .fold((0_i64, 0_usize), |(sum, count), x| (sum + x, count + 1));

    if count == 0 {
        0.0
    } else {
        sum as f64 / count as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn averages_only_positive() {
        assert!((average_positive(&[-5, 5, 15]) - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn averages_ignores_zero() {
        assert!((average_positive(&[-4, 2, 0, 6]) - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn averages_empty_slice() {
        assert_eq!(average_positive(&[]), 0.0)
    }
    #[test]
    fn averages_negatives() {
        assert_eq!(average_positive(&[-1, -2, -3]), 0.0)
    }
}
