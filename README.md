# clox-rust
cargo run --features "debug_trace_execution"

cargo run -- test.lox

# TODO: 考虑重构解析器，消除 self.previous 字段，
 以提高代码的健壮性。可以将 Token 作为参数在函数间传递，
 以取代对共享状态的依赖。
