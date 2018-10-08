use frunk_core::hlist::HList;

#[derive(Debug)]
pub enum Either<T, U> {
    A(T),
    B(U),
}

pub trait Func<Args: HList> {
    type Output;

    fn call(&self, args: Args) -> Self::Output;
}

