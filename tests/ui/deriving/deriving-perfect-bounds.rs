// Test for #26925: derive should not add unnecessary trait bounds for type
// params that only appear inside fn pointer types.

//@ check-pass

#[derive(Clone, Copy)]
struct FnPointer<T>(fn(T));

#[derive(Clone, Copy)]
struct FnReturning<T>(fn() -> T);

#[derive(Clone, Copy)]
struct TwoFnPtrs<T>(fn(T), fn(T) -> bool);

#[derive(Clone, Copy)]
struct MultiFn<T, U>(fn(T), fn(U) -> bool);

#[derive(Clone, Copy)]
enum FnEnum<T> {
    Handler(fn(T)),
    Mapper(fn(T) -> bool),
}

#[derive(Clone)]
struct MixedFnConcrete<T> {
    callback: fn(T),
    name: String,
}

#[derive(Clone)]
struct VecOfFn<T>(Vec<fn(T)>);

#[derive(Debug)]
struct DebugFn<T> {
    f: fn(T) -> bool,
}

#[allow(unpredictable_function_pointer_comparisons)]
#[derive(PartialEq, Eq)]
struct CmpFn<T>(fn(T));

#[derive(Hash)]
struct HashFn<T>(fn(T) -> u64);

// Direct T should still require T: Clone.
#[derive(Clone)]
struct Direct<T>(T);

fn assert_clone<T: Clone>() {}
fn assert_copy<T: Copy>() {}

struct NotClone;

fn main() {
    assert_clone::<FnPointer<NotClone>>();
    assert_copy::<FnPointer<NotClone>>();
    assert_clone::<FnReturning<NotClone>>();
    assert_copy::<FnReturning<NotClone>>();
    assert_clone::<TwoFnPtrs<NotClone>>();
    assert_copy::<TwoFnPtrs<NotClone>>();
    assert_clone::<MultiFn<NotClone, NotClone>>();
    assert_copy::<MultiFn<NotClone, NotClone>>();
    assert_clone::<FnEnum<NotClone>>();
    assert_copy::<FnEnum<NotClone>>();
    assert_clone::<MixedFnConcrete<NotClone>>();
    assert_clone::<VecOfFn<NotClone>>();

    // Direct<T> still needs T: Clone.
    assert_clone::<Direct<String>>();
}
