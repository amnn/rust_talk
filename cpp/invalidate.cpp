#include <iostream>
#include <vector>
using namespace std;

int main(int, char **)
{
  auto v = vector<int> {1, 2, 3};

  for(const auto &i : v)
    for(int j = 0; j <= i; ++j) {
      cout << i << ", " << j << '\n';
      v.insert(v.begin(), i + j);
    }

  return 0;
}
