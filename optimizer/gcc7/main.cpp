#include <iostream>

int hoge(int i) {
  int hoge_r = 0;
  return hoge_r;
}

int foo(int i) {
  int foo_r = i;
  return i;
}

int main(int argc, char** argv)
{
  std::cout << hoge(100) << std::endl;
  std::cout << foo(100) << std::endl;
  std::cout << foo(101) << std::endl;
}
