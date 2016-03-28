pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Either<A, B> {
    pub fn left(self) -> Option<A> {
        match self {
            Either::Left(a) => Some(a),
            _ => None,
        }
    }

    pub fn right(self) -> Option<B> {
        match self {
            Either::Right(b) => Some(b),
            _ => None,
        }
    }
}
