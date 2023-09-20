pub enum Step<T, E> {
    Done,
    Yield(T),
    Skip,
    Error(E),
}

pub trait EIterator {
    type Item;
    type Error;

    fn enext(&mut self) -> Step<Self::Item, Self::Error>;

    fn map_error<E2, F>(self, f: F) -> MapError<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Error) -> E2,
    {
        MapError {
            iter: self,
            func: f,
        }
    }

    fn iter(self) -> ToResultIterator<Self>
    where
        Self: Sized,
    {
        ToResultIterator(self)
    }
}

pub struct ToResultIterator<I>(I);

impl<I> Iterator for ToResultIterator<I>
where
    I: EIterator,
{
    type Item = Result<I::Item, I::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.enext() {
                Step::Done => {
                    return None;
                }
                Step::Skip => (),
                Step::Error(e) => {
                    return Some(Err(e));
                }
                Step::Yield(x) => {
                    return Some(Ok(x));
                }
            }
        }
    }
}

pub struct MapError<I, F> {
    iter: I,
    func: F,
}

impl<E, I: EIterator, F> EIterator for MapError<I, F>
where
    F: FnMut(I::Error) -> E,
{
    type Item = I::Item;
    type Error = E;

    fn enext(&mut self) -> Step<Self::Item, Self::Error> {
        match self.iter.enext() {
            Step::Done => Step::Done,
            Step::Skip => Step::Skip,
            Step::Error(e) => Step::Error((self.func)(e)),
            Step::Yield(x) => Step::Yield(x),
        }
    }
}

pub trait ToEIter
where
    Self: Sized,
{
    fn eiter(self) -> ResultIterator<Self> {
        ResultIterator(self)
    }
}

impl<I, T, E> ToEIter for I where I: Iterator<Item = Result<T, E>> {}

pub struct Map<I, F> {
    iter: I,
    func: F,
}

pub struct ResultIterator<I>(I);
impl<I, T, E> EIterator for ResultIterator<I>
where
    I: Iterator<Item = Result<T, E>>,
{
    type Item = T;
    type Error = E;

    fn enext(&mut self) -> Step<Self::Item, Self::Error> {
        match self.0.next() {
            Some(Ok(x)) => Step::Yield(x),
            Some(Err(e)) => Step::Error(e),
            None => Step::Done,
        }
    }
}
