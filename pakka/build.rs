fn main() {
  std::env::set_var("CC", "clang");
  std::env::set_var("CXX", "clang++");
  std::env::set_var("AR", "llvm-ar");
}
