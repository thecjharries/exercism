pub fn map<F, S, T>(input: Vec<S>, mut _function: F) -> Vec<T>
where
    F: FnMut(S) -> T,
{
    let mut output = Vec::new();
    for i in input {
        output.push(_function(i));
    }
    output
}
