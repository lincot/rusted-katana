#include <array>
#include <algorithm>

// This algorithm does not use sequential search.

unsigned short int nextHappyYear (unsigned short int year)
{
  std::array<int, 4> symbolS;
  std::array<bool, 4> isUsedS;
  
  auto set = [&] (auto x, auto z) {
    symbolS[x] = z;
    if (z < 4) isUsedS[z] = true;
  };
  
  auto clear = [&] (auto x) {
    auto z = symbolS[x];
    if (z < 4) isUsedS[z] = false;
  };
  
  auto control = [&] (auto x, auto z) {
    for (auto k = 0; k < x; k ++) if (symbolS[k] == z) return false;
    return true;
  };

  auto compute = [&] (auto x) {
    unsigned short int result{0};
    for (auto k = 0; k <= x; k ++) result = 10 * result + symbolS[k];
    for (auto k = x + 1; k < 4; k ++) {
      auto l = 0;
      for (;isUsedS[l]; l ++);
      isUsedS[l] = true;
      result = 10 * result + l;
    }
    return result;
  };
  
  std::function<unsigned short int(unsigned short int, unsigned short int)> next = [&] (auto x, auto z) -> unsigned short int {
    if (z == 10) {
      auto y = x - 1;
      clear(y);
      return next(y, symbolS[y] + 1);    
    }; 
    if (control(x, z)) {
      set(x, z);
      return compute(x);
    } 
    else {
      return next(x, z + 1);
    }
  };
  
  year++;
  
  for (auto i = 0; i < 4; i ++) isUsedS[i] = false; 
  
  for (int i = 3; i >= 0; i--) {
    symbolS[i] = year % 10;
    year /= 10;
  }
  
  set(0, symbolS[0]);
 
  for (auto i = 1; i < 4 ; i ++) {
    for (auto j = 0; j < i; j ++) {
      if (symbolS[i] == symbolS[j]) {
        return next(i, symbolS[i]);
      }
      set(i, symbolS[i]);
    }
  }
  return compute(3); 
}