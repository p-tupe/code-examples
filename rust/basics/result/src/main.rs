//! This code represents several common ways in which Rust's result type can be consumed
//! Refer https://doc.rust-lang.org/std/result/ for in-depth official stuff

#![allow(unused_variables)]
#![allow(clippy::manual_unwrap_or_default)]
#![allow(clippy::manual_unwrap_or)]
#![allow(clippy::bind_instead_of_map)]

fn main() {
    //////////////
    // `unwrap` //
    //////////////

    // Using bare unwrap to extract value (NOT RECOMMENDED)
    let x = f().unwrap();
    let Ok(x) = f() else {
        panic!("f() has some err");
    };

    // Use default value if not Ok
    let x = f().unwrap_or_default();
    let x = match f() {
        Ok(x) => x,
        Err(e) => Default::default(), // type's default value
    };

    // PREFERRED
    let x = f().unwrap_or(2); // eager eval, ignore errors
    let x = match f() {
        Ok(x) => x,
        Err(e) => 2,
    };

    let x = f().unwrap_or_else(|e| {
        eprintln!("error caught: {}", e);
        2
    }); // lazy eval for function calls
    let x = match f() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("error caught: {}", e);
            2
        }
    };

    /////////////////
    // `ok`, `err` //
    /////////////////

    let x = f().ok(); // converts Result<T, E> -> Option<T>
    let x = f().is_ok(); // converts Result<T, E> -> Ok(T) ? true : false
    let x = f().is_ok_and(|x| x == 1); // same, with extra predicate
    let x = f().err(); // converts Result<T, E> -> Option<E>
    let x = f().is_err(); // converts Result<T, E> -> Err(E) ? true : false
    let x = f().is_err_and(|x| x == "some error"); // same, with extra predicate

    //////////
    // `or` //
    //////////

    // This work same as unwrap_or, except it returns another Result value on Err
    let x: Result<i32, &'static str> = f().or(Ok(2));
    let x: Result<i32, &'static str> = match f() {
        Ok(x) => Ok(x),
        Err(e) => Ok(2),
    };

    // Prefer this when it's a function call (and not just a value)
    let x: Result<i32, &'static str> = f().or_else(|e| {
        eprintln!("error caught: {}", e);
        Ok(2)
    });
    let x: Result<i32, &'static str> = match f() {
        Ok(x) => Ok(x),
        Err(e) => {
            eprintln!("error caught: {}", e);
            Ok(2)
        }
    };

    ///////////
    // `and` //
    ///////////

    // Return value inside and(..) only if f() is Ok else return Err
    let x = f().and(Ok(2));
    let x: Result<i32, &'static str> = match f() {
        Ok(x) => Ok(2),
        Err(e) => Err(e),
    };

    // Lazy variant of above
    let x = f().and_then(|x| Ok(x + 1)); // same as map(|x| x + 1) below
    let x = match f() {
        Ok(x) => Ok(x + 1),
        Err(e) => Err(e),
    };

    /*
     * From my understanding, it's best to use let Ok(x) = f() else { .. }
     * or match f() { .. } forms when inside an imperative flow.
     *
     * The and/or/map/filter/... forms are useful when Result is a part of
     * a longer chain. They also makes the code more terse.
     */

    ///////////
    // `map` //
    ///////////

    let fx = |x| x + 1;
    let fe = |e| 2;

    let x = f().map(fx); // map Ok(T) -> Ok(U), or return Err
    let x = match f() {
        Ok(x) => Ok(fx(x)),
        Err(e) => Err(e),
    };

    let x = f().map_or(2, fx); // map if Ok, or another value
    let x = match f() {
        Ok(x) => fx(x),
        Err(e) => 2,
    };

    let x = f().map_or_else(fe, fx); // map if Ok, or another function
    let x = match f() {
        Ok(x) => fx(x),
        Err(e) => 2,
    };

    let x = f().map_or_default(fx); // map if Ok, or default value
    let x = match f() {
        Ok(x) => fx(x),
        Err(e) => Default::default(),
    };
}

/// This is fallible function that returns a result type
fn f() -> Result<i32, &'static str> {
    // Try changing `Ok(1)` to `Err("some error")`
    Ok(1)
}
