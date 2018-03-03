use std::ops::{FnOnce, AddAssign};
use std::boxed::{Box, FnBox};
use std::vec::Vec;
use std::marker::PhantomData;

pub struct HookedFnOnce<'a, F, Args, O>
    where F: FnOnce<Args, Output = O> {
    fun: F,
    hooks: Vec<Box<FnBox() + 'a>>,
    _pd: PhantomData<Args>,
}

impl<'a, F, Args, O> HookedFnOnce<'a, F, Args, O> 
    where F: FnOnce<Args, Output = O> + 'a {
    pub fn new(fun: F) -> Self {
        Self {fun: fun, hooks: vec![], _pd: PhantomData}
    }
}

impl<'a, F, Args, O> FnOnce<Args> for HookedFnOnce<'a, F, Args, O>
    where F: FnOnce<Args, Output = O> + 'a {
    type Output = O;
    extern "rust-call" fn call_once(self, args: Args) -> O {
        for hook in self.hooks {
            (hook)();
        }
        self.fun.call_once(args)
    }
}

impl<'a, Rhs, F, Args, O> AddAssign<Rhs> for HookedFnOnce<'a, F, Args, O>
    where F: FnOnce<Args, Output = O> + 'a,
          Rhs: FnOnce() + 'a {
    default fn add_assign(&mut self, rhs: Rhs)  {
        self.hooks.push(Box::new(rhs));
    }
}

/*
impl<Rhs, F, A, O> AddAssign<HookedFnOnce<Rhs, (), ()>> for HookedFnOnce<F, A, O>
    where F: FnOnce(A) -> O,
          Rhs: FnOnce() + 'static {
    fn add_assign(&mut self, rhs: Rhs)  {
        self.hooks.push(Box::new(rhs));
    }
}
*/

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;
    use std::borrow::Cow;

    #[test]
    fn fn_once() {
        let i: RefCell<u32> = RefCell::new(0);
        
        let mut comp = HookedFnOnce::new(|| {
            *(i.borrow())
        });

        comp += || {
            *(i.borrow_mut()) += 4;
        };

        assert!(comp() == 4);
        
        //println!("{}", i);
        //assert!(Rc::try_unwrap(i).unwrap() == 1);
    }

    #[test]
    fn nested_fn_once() {
        let i: RefCell<u32> = RefCell::new(0);
        
        let mut comp1 = HookedFnOnce::new(|| {
            *(i.borrow())
        });

        comp1 += || {
            *(i.borrow_mut()) += 2;
        };

        let mut comp2 = HookedFnOnce::new(comp1);

        comp2 += || {
            *(i.borrow_mut()) += 2;
        };

        assert!(comp2() == 4);
    }
}
