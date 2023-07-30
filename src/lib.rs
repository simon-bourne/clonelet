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
#[macro_export]
macro_rules! clone{
    ($(,)?) => {};
    ($name:ident $(, $($tail:tt)*)?) => {
        let $name = $name.clone();
        $($crate::clone!($($tail)*);)?
    };
    (mut $name:ident $(, $($tail:tt)*)?) => {
        let mut $name = $name.clone();
        $($crate::clone!($($tail)*);)?
    };
}
