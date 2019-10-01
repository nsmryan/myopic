/// The Lensable trait allows types to
/// carry around an Input and Output associated type.
pub trait Lensable {
    type Input;
    type Output;
}


/// The GetFun type is for getters. These getters produce a
/// A from a reference to a D.
pub type GetFun<D, A> = dyn Fn(&D) -> A;

/// The Getter trait is for anything which can get an Output type from
/// a refernence to an Input type.
pub trait Getter: Lensable {
    fn get(&self, d: &<Self as Lensable>::Input) -> <Self as Lensable>::Output;
}

impl<D, A> Lensable for dyn Fn(&D) -> A {
    type Input = D;
    type Output = A;
}

impl<D, A> Lensable for dyn Fn(&mut D, A) {
    type Input = D;
    type Output = A;
}

impl<D, A> Getter for dyn Fn(&D) -> A {
    fn get(&self, d: &D) -> A {
        return (self)(d);
    }
}

/// The Optical trait is necessary to carry around trait objects satisfying
/// multiple traits.
pub trait Optical : Lensable + Getter + Setter {
}


/// The SetFun type is for setters. Note that these setters mutate
/// the value of type D in-place.
pub type SetFun<D, A> = dyn Fn(&mut D, A);

/// The Setter trait is for types that can modify a given value of type Input
/// with a value of type Outpyt.
pub trait Setter: Lensable {
    fn set(&self, d: &mut <Self as Lensable>::Input, a: <Self as Lensable>::Output);
}

impl<D, A> Setter for dyn Fn(&mut D, A) {
    fn set(&self, d: &mut D, a: A) {
        return (self)(d, a);
    }
}


/// The lens_box module contains an implementation of lenses using boxed trait objects.
pub mod lens_box {
    use crate::{Lensable, Getter, Setter, SetFun, GetFun, Optical};

    pub struct Lens<D, A> {
        pub getter: Box<GetFun<D, A>>,
        pub setter: Box<SetFun<D, A>>,
    }

    impl<D, A> Lens<D, A> {
        pub fn new(getter: Box<GetFun<D, A>>, setter: Box<SetFun<D, A>>) -> Self {
            return Lens { getter, setter };
        }
    }

    impl<D, A> Lensable for Lens<D, A> {
        type Input = D;
        type Output = A;
    }

    impl<D, A> Getter for Lens<D, A> {
        fn get(&self, d: &D) -> A {
            return (self.getter)(d);
        }
    }

    impl<D, A> Setter for Lens<D, A> {
        fn set(&self, d: &mut D, a: A) {
            return (self.setter)(d, a);
        }
    }

    impl<D, A> Optical for Lens<D, A> {
    }

    pub struct ComposedLens<D, A, B> {
        pub lhs: Box<dyn Optical<Input=D, Output=A>>,
        pub rhs: Box<dyn Optical<Input=A, Output=B>>,
    }

    impl<D, A, B> ComposedLens<D, A, B> {
        pub fn new(l1: Box<dyn Optical<Input = D, Output = A>>,
                   l2: Box<dyn Optical<Input = A, Output = B>>) -> Self {
            return ComposedLens {
                lhs: l1,
                rhs: l2,
            };
        }
    }

    impl<D, A, B> Lensable for ComposedLens<D, A, B> {
        type Input = D;
        type Output = B;
    }

    impl<D, A, B> Getter for ComposedLens<D, A, B> {
        fn get(&self, d: &D) -> B {
            return self.rhs.get(&self.lhs.get(d));
        }
    }

    impl<D, A, B> Optical for ComposedLens<D, A, B> {
    }

    impl<D, A, B> Setter for ComposedLens<D, A, B> {
        fn set(&self, d: &mut D, b: B) {
            let mut val = self.lhs.get(d);
            self.rhs.set(&mut val, b);
            self.lhs.set(d, val);
        }
    }
}

/// The lens module contains the main implementation of the lens concept
/// as a pair of getter and setter closures. It appears to inline
/// well and is by far the fastest in the micro-benchmarks.
///
/// Note that lens does require closure types, which are unnamable.
/// I have not been able to implement this module without them, so
/// users will have to use type inference, or partially specify types like
/// ```Rust
/// let lens: Lens<_, _, u32, u8> =
/// ```
/// to specify a lense from a u32 to a u8, for example.
pub mod lens {
    use crate::{Lensable, Getter, Setter};
    use std::marker::PhantomData;

    // NOTE I was not able to get rid of D and A even though they are
    // available through the Lensable impl because they are required
    // to implement Lensable and are not constrained in the implementation.
    // This causes a compilation error, requiring me to carry them
    // around as type parameters with PhantomData fields instead.
    pub struct Lens<G, S, D, A> {
        pub getter: G,
        pub setter: S,
        d: PhantomData<D>,
        a: PhantomData<A>,
    }

    impl<G, S, D, A> Lensable for Lens<G, S, D, A> {
        type Input = D;
        type Output = A;
    }

    impl<G, S, D, A> Getter for Lens<G, S, D, A>
        where G: Fn(&D) -> A {
        fn get(&self, d: &D) -> A {
            return (self.getter)(d);
        }
    }

    impl<G, S, D, A> Setter for Lens<G, S, D, A>
        where S: Fn(&mut D, A) {
        fn set(&self, d: &mut D, a: A) {
            return (self.setter)(d, a);
        }
    }

    impl<G, S, D, A> Lens<G, S, D, A> {
        pub fn new(g: G, s: S) -> Self {
            return Lens { getter: g,
                          setter: s,
                          d: PhantomData,
                          a: PhantomData,
            };
        }
    }

    pub struct ComposedLens<L1, L2> {
        pub lhs: L1,
        pub rhs: L2,
    }

    impl<L1, L2> ComposedLens<L1, L2> {
        pub fn new(l1: L1, l2: L2) -> Self {
            ComposedLens {
                lhs: l1,
                rhs: l2,
            }
        }
    }

    impl<L1, L2> Lensable for ComposedLens<L1, L2> 
        where L1: Lensable,
              L2: Lensable {
        type Input = L1::Input;
        type Output = L2::Output;
    }

    impl<L1, L2> Getter for ComposedLens<L1, L2>
        where L1: Getter + Lensable,
              L2: Getter + Lensable<Input = L1::Output> {
        fn get(&self, d: &L1::Input) -> L2::Output {
            return self.rhs.get(&self.lhs.get(d));
        }
    }

    impl<L1, L2> Setter for ComposedLens<L1, L2>
        where L1: Setter + Getter + Lensable,
              L2: Setter + Getter + Lensable<Input = L1::Output> {
        fn set(&self, d: &mut L1::Input, b: L2::Output) {
            let mut val = self.lhs.get(d);
            self.rhs.set(&mut val, b);
            return self.lhs.set(d, val);
        }
    }
}

/// The lens_fn module contains an implementation of lens as getter/setter
/// pairs using function pointers (fn in Rust). This was intended as a kind
/// of control in the benchmarking to see if Box incurred penality vs
/// regular functions.
/// It appears that there is no reason to use lens_fn- its no faster then
/// boxed trait objects and requires separate functions for each getter/setter
/// instead of allowing boxed closures like lens_box.
pub mod lens_fn {
    use crate::{Lensable, Getter, Setter};

    pub struct Lens<D, A> {
        pub getter: fn(&D) -> A,
        pub setter: fn(&mut D, A),
    }

    impl<D, A> Lens<D, A> {
        pub fn new(getter: fn(&D) -> A,
               setter: fn(&mut D, A)) -> Self {
            return Lens {
                getter,
                setter,
            };
        }
    }

    impl<D, A> Lensable for Lens<D, A> {
        type Input = D;
        type Output = A;
    }

    impl<D, A> Getter for Lens<D, A> {
        fn get(&self, d: &D) -> A {
            return (self.getter)(d);
        }
    }

    impl<D, A> Setter for Lens<D, A> {
        fn set(&self, d: &mut D, a: A) {
            return (self.setter)(d, a);
        }
    }

    pub struct ComposedLens<O, O2> {
        pub lhs: O,
        pub rhs: O2,
    }

    impl<O, O2> Lensable for ComposedLens<O, O2>
        where O: Lensable,
              O2: Lensable {
        type Input = O::Input;
        type Output = O2::Output;
    }

    impl<O, O2> ComposedLens<O, O2> {
        pub fn new(o: O, o2: O2) -> Self {
            return ComposedLens {
                lhs: o,
                rhs: o2,
            };
        }
    }

    impl<O, O2> Getter for ComposedLens<O, O2>
        where O: Getter + Lensable,
              O2: Getter + Lensable<Input = O::Output> {
        fn get(&self, d: &O::Input) -> O2::Output {
            return self.rhs.get(&self.lhs.get(d));
        }
    }

    impl<O, O2> Setter for ComposedLens<O, O2>
        where O: Setter + Lensable + Getter,
              O2: Setter + Lensable<Input = O::Output> {
        fn set(&self, d: &mut O::Input, b: O2::Output) {
            let mut a = self.lhs.get(d);
            self.rhs.set(&mut a, b);
            self.lhs.set(d, a);
        }
    }
}

