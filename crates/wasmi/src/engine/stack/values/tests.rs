use super::*;
use crate::engine::DropKeep;

fn drop_keep(drop: usize, keep: usize) -> DropKeep {
    DropKeep::new(drop, keep).unwrap()
}

impl<'a> IntoIterator for &'a ValueStackRef<'a> {
    type Item = &'a UntypedValue;
    type IntoIter = core::slice::Iter<'a, UntypedValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.values[0..self.stack_ptr].iter()
    }
}

impl<'a> ValueStackRef<'a> {
    pub fn iter(&'a self) -> core::slice::Iter<'a, UntypedValue> {
        self.into_iter()
    }
}

#[test]
fn drop_keep_works() {
    fn assert_drop_keep<E>(stack: &ValueStack, drop_keep: DropKeep, expected: E)
    where
        E: IntoIterator,
        E::Item: Into<UntypedValue>,
    {
        let mut s = stack.clone();
        let mut s = ValueStackRef::new(&mut s);
        s.drop_keep(drop_keep);
        let expected = expected.into_iter().map(Into::into);
        assert!(s.iter().copied().eq(expected));
    }

    let test_inputs = [1, 2, 3, 4, 5, 6];
    let stack = test_inputs
        .into_iter()
        .map(UntypedValue::from)
        .collect::<ValueStack>();

    // Drop is always 0 but keep varies:
    for keep in 0..stack.len() {
        // Assert that nothing was changed since nothing was dropped.
        assert_drop_keep(&stack, drop_keep(0, keep), test_inputs);
    }

    // Drop is always 1 but keep varies:
    assert_drop_keep(&stack, drop_keep(1, 0), [1, 2, 3, 4, 5]);
    assert_drop_keep(&stack, drop_keep(1, 1), [1, 2, 3, 4, 6]);
    assert_drop_keep(&stack, drop_keep(1, 2), [1, 2, 3, 5, 6]);
    assert_drop_keep(&stack, drop_keep(1, 3), [1, 2, 4, 5, 6]);
    assert_drop_keep(&stack, drop_keep(1, 4), [1, 3, 4, 5, 6]);
    assert_drop_keep(&stack, drop_keep(1, 5), [2, 3, 4, 5, 6]);

    // Drop is always 2 but keep varies:
    assert_drop_keep(&stack, drop_keep(2, 0), [1, 2, 3, 4]);
    assert_drop_keep(&stack, drop_keep(2, 1), [1, 2, 3, 6]);
    assert_drop_keep(&stack, drop_keep(2, 2), [1, 2, 5, 6]);
    assert_drop_keep(&stack, drop_keep(2, 3), [1, 4, 5, 6]);
    assert_drop_keep(&stack, drop_keep(2, 4), [3, 4, 5, 6]);

    // Drop is always 3 but keep varies:
    assert_drop_keep(&stack, drop_keep(3, 0), [1, 2, 3]);
    assert_drop_keep(&stack, drop_keep(3, 1), [1, 2, 6]);
    assert_drop_keep(&stack, drop_keep(3, 2), [1, 5, 6]);
    assert_drop_keep(&stack, drop_keep(3, 3), [4, 5, 6]);

    // Drop is always 4 but keep varies:
    assert_drop_keep(&stack, drop_keep(4, 0), [1, 2]);
    assert_drop_keep(&stack, drop_keep(4, 1), [1, 6]);
    assert_drop_keep(&stack, drop_keep(4, 2), [5, 6]);

    // Drop is always 5 but keep varies:
    assert_drop_keep(&stack, drop_keep(5, 0), [1]);
    assert_drop_keep(&stack, drop_keep(5, 1), [6]);

    // Drop is always 6.
    assert_drop_keep(&stack, drop_keep(6, 0), iter::repeat(0).take(0));
}
