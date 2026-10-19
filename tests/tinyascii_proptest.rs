#![allow(clippy::unwrap_used)]
#![allow(clippy::as_conversions)]
#![allow(clippy::undocumented_unsafe_blocks)]

use proptest::prelude::*;

use tinyascii::error::{
    CapacityLimitError, InsertAsciiError, InsertRangeError, RemoveRangeError, WriteAsciiError,
};
use tinyascii::{
    NonNullAscii, TinyAscii4, TinyAscii8, TinyAscii16, TinyAscii32, TinyAscii64, TinyAscii128,
};

fn non_null_ascii_strategy() -> impl Strategy<Value = NonNullAscii> {
    (1u8..128).prop_map(|b| NonNullAscii::from_u8(b).unwrap())
}

proptest! {
    #[test]
    fn try_new_parse_invalid_ascii(
        mut s in prop::collection::vec(1u8..=127u8, 1..=128).prop_map(|b| String::from_utf8(b).unwrap()),
        invalid_idx in 0usize..128,
        invalid_byte in 128u8..=255,
    ) {
        unsafe {
            let idx = invalid_idx.min(s.len() - 1);
            let bytes = s.as_bytes_mut();
            bytes[idx] = invalid_byte;
        }

        let result = TinyAscii128::try_new(&s);

        prop_assert_eq!(result, Err(WriteAsciiError::InvalidAscii));
    }
}

proptest! {
    #[test]
    fn try_new_parse_invalid_capacity(
        s in prop::collection::vec(1u8..=127u8, 129..=192).prop_map(|b| String::from_utf8(b).unwrap()),
    ) {
        let result = TinyAscii128::try_new(&s);

        prop_assert_eq!(result, Err(WriteAsciiError::Capacity(CapacityLimitError)));
    }
}

proptest! {
    #[test]
    fn try_from_bytes_valid_eq_try_new(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
    ) {
        let from_bytes = TinyAscii128::try_from_bytes(s.as_bytes()).unwrap();
        let from_str = TinyAscii128::try_new(&s).unwrap();

        prop_assert_eq!(from_bytes, from_str);
    }
}

proptest! {
    #[test]
    fn from_ascii_lossy_parse_valid(
        bytes in prop::collection::vec(0u8..=255u8, 0..=128),
        replacement in non_null_ascii_strategy(),
    ) {
        let ta = TinyAscii128::from_ascii_lossy(&bytes, replacement).unwrap();

        prop_assert_eq!(ta.len(), bytes.len());

        let ta_bytes = ta.as_bytes();
        for (idx, byte) in bytes.iter().copied().enumerate() {
            match NonNullAscii::from_u8(byte) {
                Some(_) => prop_assert_eq!(ta_bytes[idx], byte),
                None => prop_assert_eq!(ta_bytes[idx], replacement.to_u8()),
            }
        }
    }
}

proptest! {
    #[test]
    fn from_ascii_lossy_parse_invalid_capacity(
        bytes in prop::collection::vec(0u8..=255u8, 129..=192),
        replacement in non_null_ascii_strategy(),
    ) {
        let result = TinyAscii128::from_ascii_lossy(&bytes, replacement);

        prop_assert_eq!(result, Err(CapacityLimitError));
    }
}

proptest! {
    #[test]
    fn from_ascii_array_valid(
        arr in prop::array::uniform::<_, 128>(1u8..=127u8),
    ) {
        let arr = arr.map(|byte| NonNullAscii::from_u8(byte).unwrap());

        let ta = TinyAscii128::from_ascii_array(arr);

        prop_assert_eq!(ta.len(), arr.len());
        prop_assert_eq!(ta.as_ascii(), arr.as_slice());
    }
}

proptest! {
    #[test]
    fn try_push_available_capacity(
        mut s in prop::collection::vec(1u8..=127u8, 0..=127).prop_map(|b| String::from_utf8(b).unwrap()),
        ascii in non_null_ascii_strategy(),
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        let result = ta.try_push(ascii);

        prop_assert_eq!(result, Ok(()));
        prop_assert_eq!(ta.len(), s.len() + 1);
        prop_assert_eq!(ta.len() + ta.available_capacity(), ta.capacity());

        s.push(ascii.to_char());
        prop_assert_eq!(&ta, s.as_str());
    }
}

proptest! {
    #[test]
    fn try_push_no_capacity(
        s in prop::collection::vec(1u8..=127u8, 128).prop_map(|b| String::from_utf8(b).unwrap()),
        ascii in non_null_ascii_strategy(),
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        let result = ta.try_push(ascii);

        prop_assert_eq!(result, Err(CapacityLimitError));
        prop_assert_eq!(ta.len(), s.len());
        prop_assert_eq!(&ta, s.as_str());
    }
}

proptest! {
    #[test]
    fn try_push_u8_all_cases(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
        byte in 0u8..=255u8,
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        let result = ta.try_push_u8(byte);

        if NonNullAscii::from_u8(byte).is_none() {
            prop_assert_eq!(result, Err(WriteAsciiError::InvalidAscii));
            prop_assert_eq!(ta.len(), s.len());
        } else if s.len() == 128 {
            prop_assert_eq!(result, Err(WriteAsciiError::Capacity(CapacityLimitError)));
            prop_assert_eq!(ta.len(), s.len());
        } else {
            prop_assert_eq!(result, Ok(()));
            prop_assert_eq!(ta.len(), s.len() + 1);
        }
    }
}

proptest! {
    #[test]
    fn try_push_str_valid(
        s in prop::collection::vec(1u8..=127u8, 0..=96).prop_map(|b| String::from_utf8(b).unwrap()),
        append in prop::collection::vec(1u8..=127u8, 0..=32).prop_map(|b| String::from_utf8(b).unwrap()),
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        let result = ta.try_push_str(&append);

        prop_assert_eq!(result, Ok(()));
        prop_assert_eq!(ta.len(), s.len() + append.len());
        prop_assert_eq!(&ta.as_str()[s.len()..], append.as_str());
    }
}

proptest! {
    #[test]
    fn try_push_str_invalid_ascii(
        s in prop::collection::vec(1u8..=127u8, 0..=96).prop_map(|b| String::from_utf8(b).unwrap()),
        mut append in ".{0,7}"
    ) {
        append.push('😀');
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        let result = ta.try_push_str(&append);

        prop_assert_eq!(result, Err(WriteAsciiError::InvalidAscii));
        prop_assert_eq!(ta.len(), s.len());
        prop_assert_eq!(&ta, s.as_str());
    }
}

proptest! {
    #[test]
    fn try_push_bytes_valid_ascii(
        s in prop::collection::vec(1u8..=127u8, 0..=64).prop_map(|b| String::from_utf8(b).unwrap()),
        bytes in prop::collection::vec(1u8..=127u8, 0..=64),
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        let result = ta.try_push_bytes(&bytes);

        prop_assert_eq!(result, Ok(()));
        prop_assert_eq!(ta.len(), s.len() + bytes.len());
        prop_assert_eq!(&ta.as_bytes()[s.len()..], bytes);
    }
}

proptest! {
    #[test]
    fn try_push_bytes_invalid(
        s in prop::collection::vec(1u8..=127u8, 0..=64).prop_map(|b| String::from_utf8(b).unwrap()),
        mut bytes in prop::collection::vec(0u8..=255u8, 1..=64),
        invalid_idx in 0usize..64
    ) {
        let idx = invalid_idx.min(bytes.len() - 1);
        bytes[idx] = 0;
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        let result = ta.try_push_bytes(&bytes);

        prop_assert_eq!(result, Err(WriteAsciiError::InvalidAscii));
        prop_assert_eq!(ta.len(), s.len());
        prop_assert_eq!(&ta, s.as_str());
    }
}

proptest! {
    #[test]
    fn try_push_lossy_replacement_usage(
        s in prop::collection::vec(1u8..=127u8, 0..=96).prop_map(|b| String::from_utf8(b).unwrap()),
        bytes in prop::collection::vec(0u8..=255u8, 0..=32),
        replacement in non_null_ascii_strategy(),
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        let result = ta.try_push_lossy(&bytes, replacement);

        prop_assert_eq!(result, Ok(()));
        prop_assert_eq!(ta.len(), s.len() + bytes.len());
        prop_assert_eq!(&ta[..s.len()], s.as_str());

        for (i, &byte) in bytes.iter().enumerate() {
            let idx = s.len() + i;
            match NonNullAscii::from_u8(byte) {
                Some(_) => prop_assert_eq!(ta.as_bytes()[idx], byte),
                None => prop_assert_eq!(ta.as_bytes()[idx], replacement.to_u8()),
            }
        }
    }
}

proptest! {
    #[test]
    fn try_push_slice_all_cases(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
        slice in prop::collection::vec(non_null_ascii_strategy(), 0..=128),
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        let result = ta.try_push_slice(&slice);

        if s.len() + slice.len() > 128 {
            prop_assert_eq!(result, Err(CapacityLimitError));
            prop_assert_eq!(ta.len(), s.len());
            prop_assert_eq!(&ta, s.as_str());
        } else {
            prop_assert_eq!(result, Ok(()));
            prop_assert_eq!(ta.len(), s.len() + slice.len());
            prop_assert_eq!(&ta.as_ascii()[s.len()..], &slice);
        }
    }
}

proptest! {
    #[test]
    fn try_insert_valid_case(
        s in prop::collection::vec(1u8..=127u8, 0..=127).prop_map(|b| String::from_utf8(b).unwrap()),
        idx in 0usize..=128,
        ascii in non_null_ascii_strategy()
    ) {
        let idx = idx.min(s.len());
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        let result = ta.try_insert(idx, ascii);

        prop_assert_eq!(result, Ok(()));
        prop_assert_eq!(ta.len(), s.len() + 1);
        prop_assert_eq!(ta.as_bytes()[idx], ascii.to_u8());

        for i in idx..s.len() {
            prop_assert_eq!(ta.as_bytes()[i + 1], s.as_bytes()[i]);
        }
        for i in 0..idx {
            prop_assert_eq!(ta.as_bytes()[i], s.as_bytes()[i]);
        }
    }
}

proptest! {
    #[test]
    fn try_insert_slice_all_cases(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
        insert in prop::collection::vec(non_null_ascii_strategy(), 0..=128),
        idx in 0usize..=128,
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        let result = ta.try_insert_slice(idx, &insert);

        if s.len() + insert.len() > 128 {
            prop_assert_eq!(result, Err(InsertAsciiError::Capacity(CapacityLimitError)));
        } else if idx > s.len() {
            prop_assert_eq!(result, Err(InsertAsciiError::IndexOutOfBounds));
        } else {
            prop_assert_eq!(result, Ok(()));
            prop_assert_eq!(ta.len(), s.len() + insert.len());

            for i in 0..idx {
                prop_assert_eq!(ta.as_bytes()[i], s.as_bytes()[i]);
            }
            for (i, &byte) in insert.iter().enumerate() {
                prop_assert_eq!(ta.as_bytes()[idx + i], byte.to_u8());
            }
            for i in idx..s.len() {
                prop_assert_eq!(
                    ta.as_bytes()[idx + insert.len() + (i - idx)],
                    s.as_bytes()[i]
                );
            }
        }
    }
}

proptest! {
    #[test]
    fn insert_then_remove_equivalence(
        s in prop::collection::vec(1u8..=127u8, 0..=64).prop_map(|b| String::from_utf8(b).unwrap()),
        idx in 0usize..=128,
        ascii in non_null_ascii_strategy(),
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();
        let idx = idx.min(ta.len());

        ta.try_insert(idx, ascii).unwrap();
        let result = ta.remove(idx);

        prop_assert_eq!(result, Some(ascii));
        prop_assert_eq!(&ta, s.as_str());
    }
}

proptest! {
    #[test]
    fn extend_from_within_all_cases(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
        start in 0usize..=128,
        end in 0usize..=128,
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();
        let available = ta.available_capacity();

        let result = ta.extend_from_within(start..end);

        if start > end || end > s.len() {
            prop_assert_eq!(result, Err(InsertRangeError::RangeOutOfBounds));
        } else if end - start > available {
            prop_assert_eq!(result, Err(InsertRangeError::Capacity(CapacityLimitError)));
        } else {
            prop_assert_eq!(result, Ok(()));
            prop_assert_eq!(&ta.as_str()[..s.len()], s.as_str());
            prop_assert_eq!(ta.len(), s.len() + (end - start));
        }
    }
}

proptest! {
    #[test]
    fn replace_range_all_cases(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
        start in 0usize..=128,
        end in 0usize..=128,
        repl in prop::collection::vec(non_null_ascii_strategy(), 0..=128),
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        let available_cap = ta.available_capacity();

        let result = ta.replace_range(start..end, &repl);

        if start > end || end > s.len() {
            prop_assert_eq!(result, Err(InsertRangeError::RangeOutOfBounds));
        } else if repl.len() as isize - (end - start) as isize > available_cap as isize {
            prop_assert_eq!(result, Err(InsertRangeError::Capacity(CapacityLimitError)));
        } else {
            let suffix_start = start + (end - start);
            prop_assert_eq!(result, Ok(()));
            prop_assert_eq!(ta.len(), s.len() + repl.len() - (end - start));

            prop_assert_eq!(&ta[..start], &s[..start]);
            prop_assert_eq!(&ta.as_ascii()[start..(start + repl.len())], &repl);
            prop_assert_eq!(&ta[(start + repl.len())..], &s[suffix_start..]);
        }
    }
}

proptest! {
    #[test]
    fn remove_range_all_cases(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
        start in 0usize..=128,
        end in 0usize..=128,
    ) {
        let mut ta: TinyAscii128 = TinyAscii128::try_new(&s).unwrap();

        let result = ta.remove_range(start..end);

        if start > end || end > s.len() {
            prop_assert_eq!(result, Err(RemoveRangeError));
        } else {
            let removed = end - start;
            prop_assert_eq!(result, Ok(()));
            prop_assert_eq!(ta.len(), s.len() - removed);

            prop_assert_eq!(&ta[..start], &s[..start]);
            prop_assert_eq!(&ta[start..], &s[start + removed..]);
        }
    }
}

proptest! {
    #[test]
    fn remove_all_cases(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
        idx in 0usize..=128,
    ) {
        let mut ta: TinyAscii128 = TinyAscii128::try_new(&s).unwrap();

        let removed = ta.remove(idx);

        if idx >= s.len() {
            prop_assert_eq!(removed, None);
            prop_assert_eq!(ta.len(), s.len());
            prop_assert_eq!(&ta, s.as_str());
        } else {
            prop_assert_eq!(removed.map(NonNullAscii::to_u8), Some(s.as_bytes()[idx]));
            prop_assert_eq!(ta.len(), s.len() - 1);

            prop_assert_eq!(&ta[..idx], &s[..idx]);
            prop_assert_eq!(&ta[idx..], &s[idx + 1..]);
        }
    }
}

proptest! {
    #[test]
    fn pop_values(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
        pop_count in 1usize..=128,
    ) {
        let mut ta: TinyAscii128 = TinyAscii128::try_new(&s).unwrap();
        let pop_count = pop_count.min(ta.len());

        for i in 1..pop_count + 1 {
            let poped = ta.pop();
            let expected = s.as_bytes()[s.len() - i];
            prop_assert_eq!(poped, Some(NonNullAscii::from_u8(expected).unwrap()));
        }

        prop_assert_eq!(ta.len(), s.len() - pop_count);
        prop_assert_eq!(&ta, &s[..s.len() - pop_count]);
    }
}

proptest! {
    #[test]
    fn pop_and_push_reversibility(
        s in prop::collection::vec(1u8..=127u8, 1..=128).prop_map(|b| String::from_utf8(b).unwrap()),
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        let mut stack = Vec::with_capacity(s.len());
        while let Some(c) = ta.pop() {
            stack.push(c);
        }

        for c in stack {
            ta.try_push(c).unwrap();
        }

        let expected: String = s.chars().rev().collect();
        prop_assert_eq!(&ta, expected.as_str());
    }
}

proptest! {
    #[test]
    fn retain_valid_output(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
        greater in non_null_ascii_strategy(),
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        ta.retain(|c| c > greater);

        let expected: Vec<_> = s.as_bytes().iter().copied()
            .filter(|&c| c > greater.to_u8())
            .collect();

        prop_assert!(ta.len() <= s.len());
        prop_assert_eq!(ta.as_bytes(), &expected);
    }
}

proptest! {
    #[test]
    fn retain_all_false(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
    ) {
        let mut ta: TinyAscii128 = TinyAscii128::try_new(&s).unwrap();

        ta.retain(|_| false);

        prop_assert!(ta.is_empty());
    }
}

proptest! {
    #[test]
    fn retain_all_true(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
    ) {
        let mut ta = TinyAscii128::try_new(&s).unwrap();

        ta.retain(|_| true);

        prop_assert_eq!(ta.len(), s.len());
        prop_assert_eq!(ta.as_str(), s.as_str());
    }
}

proptest! {
    #[test]
    fn truncate_all_cases(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
        new_len in 0usize..256,
    ) {
        let true_len = new_len.min(s.len());
        let mut ta: TinyAscii128 = TinyAscii128::try_new(&s).unwrap();

        ta.truncate(new_len);

        prop_assert!(ta.len() <= s.len());
        prop_assert!(ta.len() <= new_len);
        prop_assert_eq!(ta.len(), true_len);

        prop_assert_eq!(&ta, &s[..true_len]);
    }
}

proptest! {
    #[test]
    fn clear_then_fill_equivalence(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
    ) {
        let ta1 = TinyAscii128::try_new(&s).unwrap();

        let mut ta2 = TinyAscii128::try_new(&s).unwrap();
        ta2.clear();

        for c in s.bytes() {
            let _ = ta2.try_push_u8(c);
        }

        prop_assert_eq!(&ta1, &ta2);
    }
}

proptest! {
    #[test]
    fn resize_preserves_input(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
    ) {
        let ta: TinyAscii128 = TinyAscii128::try_new(&s).unwrap();

        let r4: TinyAscii4 = ta.clone().resize();
        prop_assert_eq!(r4.capacity(), 4);
        prop_assert_eq!(&r4, &s[..r4.len().min(4)]);

        let r8: TinyAscii8 = ta.clone().resize();
        prop_assert_eq!(r8.capacity(), 8);
        prop_assert_eq!(&r8, &s[..r8.len().min(8)]);

        let r16: TinyAscii16 = ta.clone().resize();
        prop_assert_eq!(r16.capacity(), 16);
        prop_assert_eq!(&r16, &s[..r16.len().min(16)]);

        let r32: TinyAscii32 = ta.clone().resize();
        prop_assert_eq!(r32.capacity(), 32);
        prop_assert_eq!(&r32, &s[..r32.len().min(32)]);

        let r64: TinyAscii64 = ta.clone().resize();
        prop_assert_eq!(r64.capacity(), 64);
        prop_assert_eq!(&r64, &s[..r64.len().min(64)]);

        let r128: TinyAscii128 = ta.clone().resize();
        prop_assert_eq!(r128.capacity(), 128);
        prop_assert_eq!(&r128, &s[..r128.len().min(128)]);
    }
}

proptest! {
    #[test]
    fn into_parts_from_parts_roundtrip(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
    ) {
        let ta: TinyAscii128 = TinyAscii128::try_new(&s).unwrap();
        let (arr, len) = ta.into_parts();

        let rebuilt = unsafe { TinyAscii128::from_parts(arr, len) };
        prop_assert_eq!(&rebuilt, s.as_str());
    }
}

proptest! {
    #[test]
    fn from_parts_into_parts_roundtrip(
        s in prop::collection::vec(1u8..=127u8, 0..=128).prop_map(|b| String::from_utf8(b).unwrap()),
    ) {
        let mut arr = [0u8; 128];
        arr[..s.len()].copy_from_slice(s.as_bytes());

        let ta = unsafe { TinyAscii128::from_parts(arr, s.len()) };
        let (arr2, len2) = ta.into_parts();

        prop_assert_eq!(len2, s.len());
        prop_assert_eq!(&arr2[..s.len()], s.as_bytes());
    }
}
