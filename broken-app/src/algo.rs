use std::collections::HashSet;

/// Было: O(n²) — линейный поиск в `out` на каждой вставке — плюс O(n log n)
/// сортировка на КАЖДОЙ вставке
///
/// Стало: O(1) в среднем на проверку "уже видели ли v" через `HashSet`,
/// плюс одна финальная сортировка
pub fn fast_dedup(values: &[u64]) -> Vec<u64> {
    let mut seen = HashSet::with_capacity(values.len());
    let mut out = Vec::with_capacity(values.len());
    for &v in values {
        if seen.insert(v) {
            out.push(v);
        }
    }
    out.sort_unstable();
    out
}

/// Было: экспоненциальная рекурсия без мемоизации
///
/// Стало: итеративный проход, O(n) времени и O(1) памяти.
pub fn fast_fib(n: u64) -> u64 {
    let (mut a, mut b) = (0_u64, 1_u64);
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a
}
