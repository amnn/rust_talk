#include <iostream>
using namespace std;

int *mk_int() { int x; return &x; }

int *sum(int lo, int hi)
{
  int *acc = mk_int();
  for(int i = lo; i <= hi; ++i)
    *acc += i;

  return acc;
}

int main(int, char **)
{
  for(int i = 0; i < 1000; ++i) {
    int *x = sum(0, i);
    cout << "Sum from 0 to " << i << ": " << *x << "\n";
  }

  return 0;
}
