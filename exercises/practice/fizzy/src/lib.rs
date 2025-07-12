/**
 * An alternative would have been
 * struct Matcher<T, F>
 * where
 *    F: Fn(T),
 * {
 *    callback: F,
 * }
 * but each F creates a different concrete type of Matcher, so it can’t be used generically at runtime or stored in containers.
 **/
/**
 * `dyn` is needed because without it (`Box<Fn(i32)>`, which does not work and is ambiguous), Rust cannot infer whether you want:
 *  - a generic type implementing the trait (static dispatch),
 *  - or a trait object (dynamic dispatch).
**/
pub struct Matcher<T> {
    is_match: Box<dyn Fn(T) -> bool>, // Box is needed because Fn have unknown size.
    substitute: String,
}

impl<T> Matcher<T> {
    pub fn new<F: Fn(T) -> bool + 'static, S: ToString>(_matcher: F, _subs: S) -> Matcher<T> {
        // 'static says that F me be a function or closure owns everything (i.e. doesn't borrow local variables) and is validd for the whole duration of the program
        Self {
            is_match: Box::new(_matcher),
            substitute: _subs.to_string(),
        }
    }
}

pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.matchers.push(_matcher);
        self
    }

    pub fn apply<I: Iterator<Item = T>>(self, _iter: I) -> impl Iterator<Item = String> {
        _iter // first outter iteration on the input sequence
            .map(move |el| {
                // then apply all matchers
                let res: String = self
                    .matchers
                    .iter()
                    .filter(|&matcher| (matcher.is_match)(el)) // parenthesis needed because `is_match` is a field, not a method
                    .map(|matcher| matcher.substitute.clone())
                    .collect();

                if res.is_empty() {
                    // no match
                    el.to_string()
                } else {
                    res
                }
            })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<
    T: std::ops::Rem<Output = T> + std::cmp::PartialEq + From<u8> + Copy + std::fmt::Display,
>() -> Fizzy<T> {
    Fizzy::<T>::new()
        .add_matcher(Matcher::new(
            |n: T| n % T::from(3) == T::from(0),
            "fizz".to_string(),
        ))
        .add_matcher(Matcher::new(
            |n: T| n % T::from(5) == T::from(0),
            "buzz".to_string(),
        ))
}
