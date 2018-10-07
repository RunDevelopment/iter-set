use super::*;

const BITS: u32 = 8;

fn unpack(i: u32) -> Vec<u32> {
    (0..BITS).filter(|bit| i & (1u32 << bit) != 0).collect()
}

#[test]
fn test_cmp() {
    use std::cmp::Ordering::*;

    assert_eq!(cmp(&[1, 2], &[1, 2]), Some(Equal));
    assert_eq!(cmp(&[1], &[1, 2]), Some(Less));
    assert_eq!(cmp(&[1, 2, 3], &[1, 2]), Some(Greater));
    assert_eq!(cmp(&[1, 2, 3], &[1, 2, 4]), None);

    for i in 0..(1u32 << BITS) {
        for j in 0..(1u32 << BITS) {
            let lhs = unpack(i);
            let rhs = unpack(j);

            let expected = if i == j {
                Some(Equal)
            } else if (i & j) == i {
                Some(Less)
            } else if (i & j) == j {
                Some(Greater)
            } else {
                None
            };

            assert_eq!(cmp(&lhs, &rhs), expected);
        }
    }
}

#[test]
fn test_union() {
    fn slice_union(a: &[u32], b: &[u32]) -> Vec<u32> {
        union(a, b).cloned().collect()
    }

    assert_eq!(slice_union(&[1, 2], &[1, 2]), vec![1, 2]);
    assert_eq!(slice_union(&[1], &[1, 2]), vec![1, 2]);
    assert_eq!(slice_union(&[1, 2, 3], &[1, 2]), vec![1, 2, 3]);
    assert_eq!(slice_union(&[1, 2, 3], &[1, 2, 4]), vec![1, 2, 3, 4]);

    for i in 0..(1u32 << BITS) {
        for j in 0..(1u32 << BITS) {
            let lhs = unpack(i);
            let rhs = unpack(j);

            let expected = unpack(i | j);
            assert_eq!(slice_union(&lhs, &rhs), expected);
        }
    }
}

#[test]
fn test_intersection() {
    fn slice_intersection(a: &[u32], b: &[u32]) -> Vec<u32> {
        intersection(a, b).cloned().collect()
    }

    assert_eq!(slice_intersection(&[1, 2], &[1, 2]), vec![1, 2]);
    assert_eq!(slice_intersection(&[1], &[1, 2]), vec![1]);
    assert_eq!(slice_intersection(&[1, 2, 3], &[1, 2]), vec![1, 2]);
    assert_eq!(slice_intersection(&[1, 2, 3], &[1, 2, 4]), vec![1, 2]);

    for i in 0..(1u32 << BITS) {
        for j in 0..(1u32 << BITS) {
            let lhs = unpack(i);
            let rhs = unpack(j);

            let expected = unpack(i & j);
            assert_eq!(slice_intersection(&lhs, &rhs), expected);
        }
    }
}

#[test]
fn test_difference() {
    fn slice_difference(a: &[u32], b: &[u32]) -> Vec<u32> {
        difference(a, b).cloned().collect()
    }

    assert_eq!(slice_difference(&[1, 2], &[1, 2]), vec![]);
    assert_eq!(slice_difference(&[1, 2], &[1]), vec![2]);
    assert_eq!(slice_difference(&[1, 2, 3], &[1, 2]), vec![3]);
    assert_eq!(slice_difference(&[1, 2, 3], &[1, 2, 4]), vec![3]);

    for i in 0..(1u32 << BITS) {
        for j in 0..(1u32 << BITS) {
            let lhs = unpack(i);
            let rhs = unpack(j);

            let expected = unpack(i & !j);
            assert_eq!(slice_difference(&lhs, &rhs), expected);
        }
    }
}

#[test]
fn test_symmetric_difference() {
    fn slice_symmetric_difference(a: &[u32], b: &[u32]) -> Vec<u32> {
        symmetric_difference(a, b).cloned().collect()
    }

    assert_eq!(slice_symmetric_difference(&[1, 2], &[1, 2]), vec![]);
    assert_eq!(slice_symmetric_difference(&[1, 2], &[1]), vec![2]);
    assert_eq!(slice_symmetric_difference(&[1, 2, 3], &[1, 2]), vec![3]);
    assert_eq!(
        slice_symmetric_difference(&[1, 2, 3], &[1, 2, 4]),
        vec![3, 4]
    );

    for i in 0..(1u32 << BITS) {
        for j in 0..(1u32 << BITS) {
            let lhs = unpack(i);
            let rhs = unpack(j);

            let expected = unpack(i ^ j);
            assert_eq!(slice_symmetric_difference(&lhs, &rhs), expected);
        }
    }
}