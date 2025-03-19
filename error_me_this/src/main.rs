




fn main() {
    
    // There are two types of errors in this rusty world
    // 1. Recoverable errors --> usually report the bug to the user and retry the operation
    // 2. Unrecoverable errors --> stop / crash the program because it usually refers to a bug
    // there are no exceptions in rust! instead Result<T, E> is used for recoverable
    // and the macro panic! is used for those unrecoverable ones

    // panic!("crash and burn");

    let v = vec![1,2,3];

    v[99]; // this will panic

}
