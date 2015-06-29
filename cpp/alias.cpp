#include <iostream>
#include <vector>
using namespace std;

template <typename T> void
make_room(vector<T> &v, size_t requirement)
{
  size_t newSize = max(requirement, v.size());
  v.reserve(newSize);
  v.resize(newSize);
}

template <typename T> void
array_copy(vector<T> &src, vector<T> &dest,
           size_t srcOff, size_t destOff,
           size_t len)
{
  make_room(dest, destOff + len);
  for(unsigned int i = 0; i < len; ++i)
    dest[destOff + i] = src[srcOff + i];
}

int main(int, char **)
{
  auto v = vector<int> {1, 2, 3};
  array_copy(v, v, 0, 1, 3);

  for(auto i : v) { cout << i << '\n'; }
}
