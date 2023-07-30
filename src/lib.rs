/// Clone all the identifiers supplied as arguments.
///
/// `clone!(x, y, z);` will generate:
///
/// ```
/// # #[macro_use] extern crate clonelet;
/// # let (x, y, z) = (0, 0, 0);
/// let x = x.clone();
/// let y = y.clone();
/// let z = z.clone();
/// ```
///
/// This is useful for capturing variables by copy in closures. For example:
///
/// ```
/// # #[macro_use] extern crate clonelet;
/// # let (x, y, z) = (0, 0, 0);
/// let closure = {
///     clone!(x, y, z);
///     move || {}
/// };
/// ```
///
/// If you need a mutable clone, `clone!(mut x)` will generate:
///
/// ```
/// # #[macro_use] extern crate clonelet;
/// # let x = 0;
/// let mut x = x.clone();
/// ```
///
/// You can also clone struct members:
///
/// ```
/// # #[macro_use] extern crate clonelet;
/// struct MyStruct {
///     x: i32
/// }
///
/// impl MyStruct {
///     fn clone_x(&self) {
///         // This will generate `let x = self.x.clone();`
///         clone!(self.x);
///     }
/// }
/// ```
#[macro_export]
macro_rules! clone{
    ($(,)?) => {};
    ($scope:ident . $name:ident $(, $($tail:tt)*)?) => {
        let $name = $scope . $name.clone();
        $($crate::clone!($($tail)*);)?
    };
    (mut $scope:ident . $name:ident $(, $($tail:tt)*)?) => {
        let mut $name = $scope . $name.clone();
        $($crate::clone!($($tail)*);)?
    };
    ($name:ident $(, $($tail:tt)*)?) => {
        let $name = $name.clone();
        $($crate::clone!($($tail)*);)?
    };
    (mut $name:ident $(, $($tail:tt)*)?) => {
        let mut $name = $name.clone();
        $($crate::clone!($($tail)*);)?
    };
}

#[cfg(test)]
mod tests {
    use std::mem;

    #[derive(Clone)]
    struct Test;

    impl Drop for Test {
        fn drop(&mut self) {}
    }

    impl Test {
        fn mutate(&mut self) {}
    }

    struct Scope {
        y: Test,
    }

    impl Scope {
        fn clone_member(&self) {
            clone!(self.y);
            mem::drop(y);
        }
    }

    #[test]
    fn basic() {
        let x = Test;

        {
            clone!(x);
            mem::drop(x);
        }

        mem::drop(x);
    }

    #[test]
    fn mutate() {
        let x = Test;

        {
            clone!(mut x);
            x.mutate();
        }
    }

    #[test]
    fn multi() {
        let x = Test;
        let y = Test;

        {
            clone!(x, mut y);
            y.mutate();
            mem::drop(x);
            mem::drop(y);
        }

        mem::drop(x);
        mem::drop(y);
    }

    #[test]
    fn scope() {
        let x = Scope { y: Test };

        {
            clone!(x.y);
            mem::drop(y);
        }

        mem::drop(x);
    }

    #[test]
    fn mutable_scope() {
        let x = Scope { y: Test };
        x.clone_member();

        {
            clone!(mut x.y);
            y.mutate();
            mem::drop(y);
        }

        mem::drop(x);
    }
}
