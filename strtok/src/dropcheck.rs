use std::marker::PhantomData;
struct Deserializer1<T> { _t: PhantomData<T> }
struct Deserializer2<T> { _t: PhantomData<fn() -> T> }
struct Deserializer3<T> { _t: PhantomData<fn(T)> }
struct Deserializer3<T> { 
_t1: PhantomData<fn(T)>
_t2: PhantomData<fn(T) -> T>
}
