use std::mem;
use std::ops::Add;

use futures::{Async, Future, Poll};
use frunk_core::hlist::{HList};

use ::reject::CombineRejection;
use super::{FilterBase, Filter};

#[derive(Clone, Copy, Debug)]
pub struct And<T, U> {
    pub(super) first: T,
    pub(super) second: U,
}

impl<T, U, OUT> FilterBase for And<T, U>
where
    T: Filter,
    U: Filter + Clone + Send,
    T::Extract: Send,
    T::Extract: Add<U::Extract, Output = OUT>,
    U::Error: CombineRejection<T::Error>,
    OUT: HList + Send
{
    type Extract = OUT;
    type Error = <U::Error as CombineRejection<T::Error>>::Rejection;
    type Future = AndFuture<T, U>;

    fn filter(&self) -> Self::Future {
        AndFuture {
            state: State::First(self.first.filter(), self.second.clone()),
        }
    }
}

#[allow(missing_debug_implementations)]
pub struct AndFuture<T: Filter, U: Filter> {
    state: State<T, U>,
}

enum State<T: Filter, U: Filter> {
    First(T::Future, U),
    Second(Option<T::Extract>, U::Future),
    Done,
}

impl<T, U, OUT> Future for AndFuture<T, U>
where
    T: Filter,
    U: Filter,
    //T::Extract: Combine<U::Extract>,
    T::Extract: Add<U::Extract, Output = OUT>,
    U::Error: CombineRejection<T::Error>,
    OUT: HList + Send
{
    type Item = OUT;
    type Error = <U::Error as CombineRejection<T::Error>>::Rejection;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let ex1 = match self.state {
            State::First(ref mut first, _) => {
                try_ready!(first.poll())
            },
            State::Second(ref mut ex1, ref mut second) => {
                let ex2 = try_ready!(second.poll());
                let ex3 = ex1.take().unwrap().hlist().combine(ex2.hlist()).flatten();
                return Ok(Async::Ready(ex3));
            },
            State::Done => panic!("polled after complete"),
        };

        let mut second = match mem::replace(&mut self.state, State::Done) {
            State::First(_, second) => second.filter(),
            _ => unreachable!(),
        };

        match second.poll()? {
            Async::Ready(ex2) => {
                Ok(Async::Ready(ex1.hlist().combine(ex2.hlist()).flatten()))
            },
            Async::NotReady => {
                self.state = State::Second(Some(ex1), second);
                Ok(Async::NotReady)
            },
        }
    }
}

