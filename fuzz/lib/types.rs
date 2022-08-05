#[derive(Debug)]
pub struct ConstrainedUSize {
    pub num: usize
}

#[derive(Debug)]
pub struct ConstrainedN {
    pub num: usize
}


use libfuzzer_sys::arbitrary::{Arbitrary,Result,Unstructured};

impl<'a> Arbitrary<'a> for ConstrainedUSize {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.int_in_range(0..=20000000).map(|i| ConstrainedUSize { num: i })
    }
}

impl<'a> Arbitrary<'a> for ConstrainedN {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.int_in_range(0..=1270607).map(|i| ConstrainedN { num: i })
    }
}
