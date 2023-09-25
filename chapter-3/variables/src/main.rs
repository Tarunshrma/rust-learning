fn main() {
    //******************************************************/
    //                          Const and Var
    //******************************************************/
    //NOTE: by default variables are immutable
    //NOTE: very strong compile time check to avoid run time checks
    let immutableVar = "abc";
    println!("immutableVar value is {immutableVar}");

    let mut mutableVar = "xyx";
    println!("mutableVar value is {mutableVar}");

    mutableVar = "123";
    println!("mutableVar value is {mutableVar}");

    //const
    //NOTE: const values cannot be mutable.. can not add mut to const
    const constValue: f64 = 3.14;
    println!("constValue value is {constValue}");

    //******************************************************/
    //                          Shadowing
    //******************************************************/
    //NOTE: you can re-use the variable by declaring it again with same name
    let immutableVar = "This is how you can mutate me";
    println!("immutableVar value is: {immutableVar}");
}
