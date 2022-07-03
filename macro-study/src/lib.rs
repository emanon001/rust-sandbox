use if_chain::if_chain;
mod if_let_chain;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn if_chain() {
        let opt = Some(1);
        if_chain! {
            if let Some(v) = opt;
            if v == 1;
            then {
                assert!(true);
            }
        }
    }
}
