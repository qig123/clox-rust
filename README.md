# clox-rust
cargo run --features "debug_trace_execution" ./test.lox

cargo run -- test.lox

# TODO: 考虑重构解析器，消除 self.previous 字段，
 以提高代码的健壮性。可以将 Token 作为参数在函数间传递，
 以取代对共享状态的依赖。

 # 添加字符串，使用了Rc<String>
  已经实现
# 添加对全局变量的支持
    语法变化
    statement      → exprStmt
               | printStmt ;

    declaration    → varDecl
               | statement ;
    到目前为止，我们的虚拟机都认为“程序”是一个表达式，所以我们将会支持语句，我们一步步来，为了支持语句，我们要支持声明。所以我们的语法
    会变成这样     declaration    → varDecl | statement ; varDecl我们后续实现，我们先实现decl->statment ,然后 statment -> printStmt,
    printStmt ->expression ....
    我们先实现printStmt这一种，后面还有更多的statment， 所以我们的vm，compiler都要修改，
     比如： 通常来说 print语句，就应该先求值再打印，所以我们的vm里面的return 指令就不用打印了
    可能还要实现新的虚拟机指令，要怎样做呢？
  
               

