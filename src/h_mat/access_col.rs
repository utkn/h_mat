use super::{HCol, HMat};

pub trait AccessCol<'a, T> {
    type TakeRem;
    fn take_col(&mut self, idx: usize) -> HCol<T, Self::TakeRem>;
    type GetRem;
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T, Self::GetRem>;
    type GetMutRem;
    fn get_col_mut(&'a mut self, idx: usize) -> HCol<&mut T, Self::GetMutRem>;
}

impl<'a, T1, T2, R> AccessCol<'a, T1> for HMat<T1, HMat<T2, R>>
where
    Self: 'a,
    HMat<T2, R>: AccessCol<'a, T2>,
{
    type TakeRem = HCol<T2, <HMat<T2, R> as AccessCol<'a, T2>>::TakeRem>;
    fn take_col(&mut self, idx: usize) -> HCol<T1, Self::TakeRem> {
        HCol {
            elem: self.row.take(idx),
            rem: self.rem.take_col(idx),
        }
    }

    type GetRem = HCol<&'a T2, <HMat<T2, R> as AccessCol<'a, T2>>::GetRem>;
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T1, Self::GetRem> {
        HCol {
            elem: self.row.get(idx),
            rem: self.rem.get_col_ref(idx),
        }
    }

    type GetMutRem = HCol<&'a mut T2, <HMat<T2, R> as AccessCol<'a, T2>>::GetMutRem>;
    fn get_col_mut(&'a mut self, idx: usize) -> HCol<&mut T1, Self::GetMutRem> {
        HCol {
            elem: self.row.get_mut(idx),
            rem: self.rem.get_col_mut(idx),
        }
    }
}

impl<'a, T> AccessCol<'a, T> for HMat<T, ()> {
    type TakeRem = ();
    fn take_col(&mut self, idx: usize) -> HCol<T, Self::TakeRem> {
        HCol {
            elem: self.row.take(idx),
            rem: (),
        }
    }

    type GetRem = ();
    fn get_col_ref(&'a self, idx: usize) -> HCol<&T, Self::GetRem> {
        HCol {
            elem: self.row.get(idx),
            rem: (),
        }
    }

    type GetMutRem = ();
    fn get_col_mut(&mut self, idx: usize) -> HCol<&mut T, Self::GetMutRem> {
        HCol {
            elem: self.row.get_mut(idx),
            rem: (),
        }
    }
}
