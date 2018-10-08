use frunk_core::hlist::{HList, HCons, HNil, h_cons};

pub(crate) type One<T> = HCons<T, HNil>;

pub(crate) fn one<T>(head: T) -> One<T> {
    h_cons(head, HNil)
}

#[derive(Debug)]
pub enum Either<T, U> {
    A(T),
    B(U),
}

pub trait Func<Args: HList> {
    type Output: HList;

    fn call(&self, args: Args) -> Self::Output;
}

impl<F, U> Func<HNil> for F
where
    F: Fn() -> U
{
    type Output = One<U>;

    fn call(&self, args: HNil) -> Self::Output {
        one(self())
    }
}

macro_rules! func_def {
    ( $head: ident ) => {
        impl<F, $head, U> Func<Hlist![$head]> for F
        where
            F: Fn($head) -> U
        {
            type Output = One<U>;

            fn call(&self, args: Hlist![$head]) -> Self::Output {
                let hlist_pat![$head] = args;
                one(self($head))
            }
        }
    };
    ( $head: ident, $( $tail: ident),* ) => {
        func_def!($($tail),*);

        impl<F, U, $head, $($tail),*,> Func<Hlist![$head, $($tail),*,]> for F
        where
            F: Fn($head, $($tail),*,) -> U
        {
            type Output = One<U>;

            fn call(&self, args: Hlist![$head, $($tail),*,]) -> Self::Output {
                let hlist_pat![$head, $($tail),*,] = args;
                one(self($head, $($tail),*,))
            }
        }
    }
}

func_def! {
    T16,
    T15,
    T14,
    T13,
    T12,
    T11,
    T10,
    T9,
    T8,
    T7,
    T6,
    T5,
    T4,
    T3,
    T2,
    T1,
    T0
}