
// Maxine:
//   this code was anonymously contributed by a close friend of mine

use core::ops::{Deref, DerefMut};
use std::cell::{Cell, OnceCell};

pub trait Prototype {
    type Proto = ??;
    fn getPrototype(&self) -> Self::Proto;
}

pub struct LifetimeVirtual<'a, T> {
    factory: Cell<Box<dyn FnOnce() -> T + 'a>>,
    val: OnceCell<T>,
}

impl<'a, T> LifetimeVirtual<'a, T> {
    pub fn new(factory: impl FnOnce() -> T + 'a) -> Self {
        Self {
            factory: Cell::new(Box::new(factory)),
            val: OnceCell::new(),
        }
    }

    fn set_or_panic(&self, val: T) {
        assert!(
            self.val.set(val).is_ok(),
            "Somehow an empty OnceCell was already set."
        )
    }

    fn get_factory(&self) -> Box<dyn FnOnce() -> T + 'a> {
        self.factory.replace(Box::new(|| {
            panic!("{}", "Attempt to construct memoised value twice.")
        }))
    }
}

/// Sets the underlying value of a `Virtual` value.  Note that though this can
/// be done via a mutable dereference, e.g.:
/// ```ignore
/// let virt_i32 = Virtual::<i32>::new(expensive_fn);
/// *virt_i32 = 42;
/// ```
/// This would unnecessarily invoke `expensive_fn`.  This can be avoided via:
/// ```ignore
/// set_virtual(virt_i32, 42);
/// ```
/// where necessary.
///
/// Note: This is not a method since `[Lifetime]Vitrtual` implements
/// `Deref[Mut]` and therefore we wish to avoid name collisions with the
/// methods of T.
pub fn set_virtual<'a, T>(virt: &mut LifetimeVirtual<'a, T>, val: T) {
    match virt.val.get_mut() {
        Some(actual) => *actual = val,
        None => {
            // We get the factory just to implicitly set it to panic if it's
            // called, which should now never be necessary.
            let _ = virt.get_factory();
            virt.set_or_panic(val)
        }
    }
}

// pub type Prototype<T> = 
pub type Virtual<T> = LifetimeVirtual<'static, T>;

impl<'a, T> Deref for LifetimeVirtual<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.val.get() {
            Some(t) => t,
            None => self.val.get_or_init(self.get_factory()),
        }
    }
}

impl<'a, T> DerefMut for LifetimeVirtual<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.val.get().is_none() {
            self.set_or_panic(self.get_factory()())
        }
        self.val.get_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_factory() {
        let mut name = String::from("Bokosmole");
        let virt = Virtual::<String>::new(move || {
            println!("Factory for {} go brrr", name);
            name.push('!');
            name
        });
        assert_eq!(*virt, "Bokosmole!");
    }

    #[test]
    fn mutable_ref_factory() {
        let mut expensive_count: i32 = 0;
        {
            let mut virt_a = LifetimeVirtual::<i32>::new(|| {
                expensive_count += 1;
                42
            });

            assert_eq!(*virt_a, 42);
            assert_eq!(*virt_a, 42);
            *virt_a = 32;
            assert_eq!(*virt_a, 32);
        }
        assert_eq!(expensive_count, 1);
    }
    
    #[test]
    fn set_virtual_eliminate_factory_call() {
        let mut expensive_count: i32 = 0;
        {
            let mut virt_a = LifetimeVirtual::<i32>::new(|| {
                expensive_count += 1;
                42
            });

            set_virtual(&mut virt_a, 32);
            assert_eq!(*virt_a, 32);
        }
        assert_eq!(expensive_count, 0);
    }
    
    struct Foo {
        pub bar: i32,
    }
    
    impl Foo {
        fn get(&self) -> i32 { self.bar }
        fn set(&mut self, new : i32) { self.bar = new; }
    }
    
    #[test]
    fn implicit_deref() {
        let mut virt = Virtual::<Foo>::new(|| {Foo{bar: 1}});
        assert_eq!(virt.get(), 1);
        virt.set(2);
        assert_eq!(virt.get(), 2);
    }
}

