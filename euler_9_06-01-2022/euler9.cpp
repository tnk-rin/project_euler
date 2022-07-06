#include <iostream>

int main() {
	for(int a = 0; a < 1000; ++a) {
		for(int b = a; b < 1000; ++b) {
			for(int c = b; c < 1000; ++c) {
				if (((a * a) + (b * b)) == (c * c)) {
					if ((a + b + c) == 1000)
						std::cout << a << ", " << b << ", " << c << " : " << (a * b * c) << std::endl;
				}
			}
		}
	}
}
