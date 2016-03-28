pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub trait LeftRight {
    type Left;
    type Right;

    fn left(self) -> Option<Self::Left>;

    fn right(self) -> Option<Self::Right>;
}

impl<A, B> LeftRight for Either<A, B> {
    type Left = A;
    type Right = B;

    fn left(self) -> Option<A> {
        match self {
            Either::Left(a) => Some(a),
            _ => None,
        }
    }

    fn right(self) -> Option<B> {
        match self {
            Either::Right(b) => Some(b),
            _ => None,
        }
    }
}

impl<'a, A, B> LeftRight for &'a Either<A, B> {
    type Left = &'a A;
    type Right = &'a B;

    fn left(self) -> Option<&'a A> {
        match self {
            &Either::Left(ref a) => Some(a),
            _ => None,
        }
    }

    fn right(self) -> Option<&'a B> {
        match self {
            &Either::Right(ref b) => Some(b),
            _ => None,
        }
    }
}

impl<'a, A, B> LeftRight for &'a mut Either<A, B> {
    type Left = &'a mut A;
    type Right = &'a mut B;

    fn left(self) -> Option<&'a mut A> {
        match self {
            &mut Either::Left(ref mut a) => Some(a),
            _ => None,
        }
    }

    fn right(self) -> Option<&'a mut B> {
        match self {
            &mut Either::Right(ref mut b) => Some(b),
            _ => None,
        }
    }
}
