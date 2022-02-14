#include <iostream>

using namespace std;

extern "C" void say_hello() {
    cout << "Hello from C++ test" << endl;
}
