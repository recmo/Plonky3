use alloc::vec::Vec;
use p3_matrix::Matrix;

/// A "Mixed Matrix Commitment Scheme" (MMCS) is a bit like a vector commitment scheme, but it
/// supports committing to matrices and then opening rows. It is also batch-oriented; one can commit
/// to a batch of matrices at once even if their widths and heights differ.
///
/// When a particular row index is opened, it is interpreted directly as a row index for matrices
/// with the largest height. For matrices with smaller heights, some bits of the row index are
/// removed (from the least-significant side) to get the effective row index. These semantics are
/// useful in the FRI protocol.
///
/// The `DirectMMCS` sub-trait represents an MMS which can be directly constructed from a set of
/// matrices. Other MMCSs may be virtual combinations of child MMCSs, or may be constructed in a
/// streaming manner.
pub trait MMCS<T> {
    type ProverData;
    type Commitment;
    type Proof;
    type Error;
    type Mat: Matrix<T>;

    fn open_batch(row: usize, prover_data: &Self::ProverData) -> (Vec<&[T]>, Self::Proof);

    /// Get the matrices that were committed to.
    fn get_matrices(prover_data: &Self::ProverData) -> &[Self::Mat];

    /// Verify a batch opening.
    fn verify_batch(
        commit: &Self::Commitment,
        dimensions: &[Dimensions],
        index: usize,
        item: Vec<T>,
        proof: &Self::Proof,
    ) -> Result<(), Self::Error>;
}

pub struct Dimensions {
    pub width: usize,
    pub log2_height: usize,
}

/// An MMCS over explicit inputs which are supplied upfront.
pub trait DirectMMCS<T>: MMCS<T> {
    fn commit(&self, inputs: Vec<Self::Mat>) -> (Self::Commitment, Self::ProverData);
}
