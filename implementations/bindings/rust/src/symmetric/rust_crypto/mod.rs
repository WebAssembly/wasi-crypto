use digest::{HashMarker, Update, OutputSizeUser, FixedOutput};

impl HashMarker for super::Hash {

}

impl Update for super::Hash {
    fn update(&mut self, data: &[u8]) {
        self.absorb(data).unwrap();
    }
}

impl OutputSizeUser for super::Hash {
    type OutputSize = digest::generic_array::typenum::U64;
}

impl FixedOutput for super::Hash {
    fn finalize_into(self, out: &mut digest::Output<Self>) {
        self.squeeze_into(out).unwrap();
    }
}