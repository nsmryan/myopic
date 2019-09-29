
pub trait Lensable {
    type Input;
    type Output;
}


pub type GetFun<D, A> = dyn Fn(&D) -> A;

pub trait Getter: Lensable {
    fn get(&self, &<Self as Lensable>::Input) -> <Self as Lensable>::Output;
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


pub type SetFun<D, A> = dyn Fn(&mut D, A);

pub trait Setter: Lensable {
    fn set(&self, &mut <Self as Lensable>::Input, <Self as Lensable>::Output);
}

impl<D, A> Setter for dyn Fn(&mut D, A) {
    fn set(&self, d: &mut D, a: A) {
        return (self)(d, a);
    }
}


mod lens_box {
    use {Lensable, Getter, Setter, SetFun, GetFun};

    pub trait Optical : Lensable + Getter + Setter {
    }

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

mod lens {
    use {Lensable, Getter, Setter};
    use std::marker::PhantomData;

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

mod lens_simple {
    use {Lensable, Getter, Setter};

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

    // TODO attempt at implementing traits for ComposedLens
    // requires generic parameters that implement traits themselves.
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


fn get_bit(d: &u8) -> u8 {
    return d & 0x01;
}

fn set_bit(d: &mut u8, a: u8) {
    *d = (*d & 0xFE) | a;
}

fn get_first_byte(d: &u32) -> u8 {
    return (d & 0xFF) as u8;
}

fn set_first_byte(d: &mut u32, a: u8) {
    *d = (*d & 0xFFFFFF00) | (a as u32);
}

fn get_bool(d: &u8) -> bool {
    return (d & 0x01) == 0x01;
}

fn set_bool(d: &mut u8, a: bool) {
    let val = if a {
        1 
    } else {
        0
    };
    *d = (*d & 0xFE) | val;
}

fn main() {
    {
        println!("Composed Boxed Lens");
        let mut d: u32 = 0;

        let optic: lens_box::Lens<u32, u8> =
            lens_box::Lens::new(Box::new(get_first_byte),
                                Box::new(set_first_byte));

        let optic2: lens_box::Lens<u8, bool> =
            lens_box::Lens::new(Box::new(get_bool), Box::new(set_bool));

        let optic3 =
            lens_box::ComposedLens::new(Box::new(optic), Box::new(optic2));
        println!("{:X}", d);
        optic3.set(&mut d, false);
        println!("{}", optic3.get(&d));
        optic3.set(&mut d, true);
        println!("{}", optic3.get(&d));
        println!("{:X}", d);
    }

    {
        println!("Boxed Lens");
        let func: lens_box::Lens<u8, u8>
            = lens_box::Lens::new(Box::new(|d: &u8| d & 0x01),
                                  Box::new(|d: &mut u8, a: u8| *d = (*d & 0x0E) | a));

        let mut d: u8 = 0;
        func.set(&mut d, 1);
        println!("{}", func.get(&d));
        func.set(&mut d, 0);
        println!("{}", func.get(&d));
    }

    println!("_____________");

    {
        println!("Composed Lens");
        let mut d: u32 = 0;

        let optic: lens::Lens<_, _, u32, u8> =
            lens::Lens::new(get_first_byte, set_first_byte);

        let optic2: lens::Lens<_, _, u8, bool> =
            lens::Lens::new(get_bool, set_bool);

        let optic3 = lens::ComposedLens::new(optic, optic2);
        println!("{:X}", d);
        optic3.set(&mut d, false);
        println!("{}", optic3.get(&d));
        optic3.set(&mut d, true);
        println!("{}", optic3.get(&d));
        println!("{:X}", d);
    }

    {
        println!("Lens");
        let func: lens::Lens<_, _, u8, u8>
            = lens::Lens::new(|d: &u8| d & 0x01,
                        |d: &mut u8, a: u8| *d = (*d & 0x0E) | a);

        let mut d: u8 = 0;
        func.set(&mut d, 1);
        println!("{}", func.get(&d));
        func.set(&mut d, 0);
        println!("{}", func.get(&d));
    }

    println!("_____________");

    {
        println!("Composed Optic");
        let mut d: u32 = 0;

        let optic: lens_simple::Lens<u32, u8> =
            lens_simple::Lens::new(get_first_byte, set_first_byte);

        let optic2: lens_simple::Lens<u8, bool> =
            lens_simple::Lens::new(get_bool, set_bool);

        let optic3 = lens_simple::ComposedLens::new(optic, optic2);
        println!("{:X}", d);
        optic3.set(&mut d, false);
        println!("{}", optic3.get(&d));
        optic3.set(&mut d, true);
        println!("{}", optic3.get(&d));
        println!("{:X}", d);
    }

    {
        println!("Optic");

        let mut d: u8 = 0;
        let optic: lens_simple::Lens<u8, u8> =
            lens_simple::Lens::new(get_bit, set_bit);

        optic.set(&mut d, 0);
        println!("{}", optic.get(&d));
        optic.set(&mut d, 1);
        println!("{}", optic.get(&d));
    }
    println!("_____________");
}

