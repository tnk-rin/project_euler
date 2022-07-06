#include <cstdint>
#include <iostream>
#include <vector>

uint64_t traverse_table(long table[21][21]) {
	uint64_t p = 0;
	bool all_visited = false;

	while (!all_visited) {
		// check all 4 directions
		for(int i = 0; i < 4; ++i) {
			//lmao im just using an excel doc for this
		}
	}

	return p;
}

int main() {
	long table[21][21] = { 0 };
	uint64_t legal_plays = 0;

	table[20][20] = 1;

	legal_plays = traverse_table(table);

	printf("\nnumber of moves: %lu\n", legal_plays);
	return 0;
}
