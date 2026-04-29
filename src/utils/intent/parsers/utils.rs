use crate::utils::intent::{Capabilities, Unit, UnitCategory};

/// Parse a bare numeric string, allowing embedded commas ("1,234.5").
/// Uses a small stack buffer to strip commas without heap allocation.
#[inline]
pub(super) fn parse_f64(s: &str) -> Option<f64> {
    if !s.contains(',') {
        return s.parse().ok();
    }
    // At most ~26 chars for any sane number; fall back to heap only if absurd.
    let mut buf = arrayvec::ArrayString::<32>::new();
    for c in s.chars() {
        if c != ',' {
            buf.try_push(c).ok()?;
        }
    }
    buf.parse().ok()
}

/// Try to read (value, unit) from `slice`, consuming no heap.
/// Handles: ["3.5", "kg"] · ["$100"] · ["3.5kg"] · ["3", "fluid", "oz"]
pub(super) fn extract_value_unit<'t>(
    slice: &'t [&'t str],
    caps: &Capabilities,
) -> Option<(f64, Unit)> {
    match slice {
        [] => None,

        // ── Single combined token ── "3.5kg" / "$100"
        [s] => split_combined(s, caps),

        // ── Two tokens ── "3.5 kg" / "$ 100"
        [a, b] => {
            // number · unit
            if let Some(v) = parse_f64(a)
                && let Some(u) = Unit::parse_with_capabilities(b, caps)
            {
                return Some((v, u));
            }
            // unit · number  (symbol split by tokenizer: "$" "100")
            if let Some(u) = Unit::parse_with_capabilities(a, caps)
                && let Some(v) = parse_f64(b)
            {
                return Some((v, u));
            }
            None
        }

        // ── Three tokens ── "3 fluid oz"  (value + two-word unit)
        [a, b, c] => {
            if let Some(v) = parse_f64(a) {
                // try joining b+c as a unit name without allocation via a small buf
                let mut buf = arrayvec::ArrayString::<32>::new();
                let _ = buf.try_push_str(b);
                let _ = buf.try_push(' ');
                let _ = buf.try_push_str(c);
                if let Some(u) = Unit::parse_with_capabilities(&buf, caps) {
                    return Some((v, u));
                }
                // also try just the last token ("3 imperial oz" → "oz")
                if let Some(u) = Unit::parse_with_capabilities(c, caps) {
                    return Some((v, u));
                }
            }
            None
        }

        // ── Longer: try last-two and first-two (noise words in between)
        _ => {
            let n = slice.len();
            extract_value_unit(&slice[n - 2..], caps)
                .or_else(|| extract_value_unit(&slice[..2], caps))
        }
    }
}

/// Split a combined token like "3.5kg" or "$100" without allocating.
pub(super) fn split_combined(s: &str, caps: &Capabilities) -> Option<(f64, Unit)> {
    // Find digit↔non-digit boundary
    let boundary = s.char_indices().skip(1).find(|(i, c)| {
        let prev = s[..*i].chars().last().unwrap();
        prev.is_ascii_digit() != c.is_ascii_digit() && prev != '.' && *c != '.'
    });
    let idx = boundary.map(|(i, _)| i)?;

    if s.as_bytes()[0].is_ascii_digit() {
        // "3.5kg"
        let v = parse_f64(&s[..idx])?;
        let u = Unit::parse_with_capabilities(&s[idx..], caps)?;
        Some((v, u))
    } else {
        // "$100" — find where digits start
        let digit_start = s.find(|c: char| c.is_ascii_digit())?;
        let u = Unit::parse_with_capabilities(&s[..digit_start], caps)?;
        let v = parse_f64(&s[digit_start..])?;
        Some((v, u))
    }
}

/// Try to parse a target unit from 1–2 tokens, constraining to `category`.
pub(super) fn extract_target_unit(slice: &[&str], category: UnitCategory) -> Option<Unit> {
    match slice {
        [s] => Unit::parse_in_category(s, category),
        [a, b] => {
            let mut buf = arrayvec::ArrayString::<32>::new();
            let _ = buf.try_push_str(a);
            let _ = buf.try_push(' ');
            let _ = buf.try_push_str(b);
            Unit::parse_in_category(&buf, category)
                .or_else(|| {
                    let mut buf2 = arrayvec::ArrayString::<32>::new();
                    let _ = buf2.try_push_str(a);
                    let _ = buf2.try_push_str(b);
                    Unit::parse_in_category(&buf2, category)
                })
                .or_else(|| Unit::parse_in_category(b, category))
        }
        _ => None,
    }
}
