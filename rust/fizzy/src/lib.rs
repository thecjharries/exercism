// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<'a, T> {
    _matcher: Box<dyn Fn(T) -> bool + 'a>,
    _subs: String,
}

impl<'a, T> Matcher<'a, T> {
    pub fn new<F, S>(_matcher: F, _subs: S) -> Matcher<'a, T>
    where
        F: Fn(T) -> bool + 'a,
        S: ToString,
    {
        Matcher {
            _matcher: Box::new(_matcher),
            _subs: _subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T>(Vec<Matcher<'static, T>>);

impl<T> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy(Vec::new())
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, _matcher: Matcher<'static, T>) -> Self {
        self.0.push(_matcher);
        self
    }

    #[cfg(not(tarpaulin_include))]
    pub fn apply<I>(self, _iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
        T: Clone + ToString,
    {
        _iter.map(move |n| {
            let mut s = String::new();
            for m in &self.0 {
                if (m._matcher)(n.clone()) {
                    s.push_str(&m._subs);
                }
            }
            if s.is_empty() {
                s.push_str(&n.to_string());
            }
            s
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: std::ops::Rem<Output = T> + PartialEq + ToString,
    u8: Into<T>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
