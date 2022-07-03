#[macro_export]
macro_rules! if_let_chain {
    (let $p:pat = $expr:expr, $(let $rest_p:pat = $rest_expr:expr)+, $consequence: block $(else $alternative: block)?) => {
        if let $p = $expr {
            if_let_chain!($(let $rest_p = $rest_expr)*, $consequence $(else $alternative)?);
        }$(else {
            $alternative;
        })?
    };
    (let $p:pat = $expr:expr, $consequence: block $(else $alternative: block)?) => {
        if let $p = $expr {
            $consequence;
        }$(else {
            $alternative;
        })?
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn if_let_chain() {
        let a = Some(1);
        let b = Some(2);
        if_let_chain! (
            let Some(a) = a,
            let Some(b) = b,
            {
                assert_eq!(a + b, 3);
            }
        );
    }

    #[test]
    fn if_let_chain_else() {
        let a = Some(1);
        let b = Some(2);
        let c: Option<i32> = None;
        if_let_chain! (
            let Some(a) = a,
            let Some(b) = b,
            {
                assert_eq!(a + b, 3);
            } else {
                assert!(false);
            }
        );
        if_let_chain! (
            let Some(_a) = a,
            let Some(_c) = c,
            {
                assert!(false);
            } else {
                assert!(true);
            }
        );
    }
}
