# Smart Pointers

## References vs Smart Pointers

* **reference** - the most common pointer in Rust
  * indicated by the `&` symbol and enables the borrowing of values pointed to by the reference.

* **smart pointers** - another class of pointers that provide additional capabilities and metadata.
  * Usually implemented using structs that have `Deref` and `Drop` traits
  * Example: *reference counting* smart pointer, `Rc<T>`

* References have no overhead.

* References *borrow* data while most smart pointers *own* data.

## Common Rust Smart Pointers

* `Box<T>`
  * allocate values on the heap
* `Rc<T>`
  * reference counting pointer that allows for multiple ownership
  * Note: for thread safe reference counting, use the type `Arc<T>`
* `RefCell<T>`

## `Box<T>`

## `Rc<T>` and `Arc<T>`

## `RefCell<T>` (maybe?)
