#include <array>

unsigned short int nextHappyYear (unsigned short int year)
{
  year++;
  std::array<bool, 10> isNumberS;
  for (int j = 0; j < 10; j ++) isNumberS[j] = false;
  std::array<int, 4> symbolS;
  std::array<int, 4> addS = {1000, 100, 10 , 1};
  
  auto set = [&]() {
    auto x = year;
    for (auto i = 0; i < 4; i++) {
      symbolS[3 - i] = x % 10; 
      x /= 10;
    }
  };
  
  int add{0};
  auto reverse = [&](auto x) {
    for (auto i = 0; i <= x; i++) {
      isNumberS[symbolS[i]] = false;
    }
    add = addS[x];
  };
  
  auto control = [&] () {
    set();
    for (auto i = 0; i < 4; i++) {
      if (isNumberS[symbolS[i]]) {
        reverse(i);
        return i;
      }
      isNumberS[symbolS[i]] = true;
    }
    reverse(3);
    return 4;
  };
  
  for (; control() != 4; year = (year / add + 1) * add );
  return year ; 
}