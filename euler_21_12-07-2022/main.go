package main

import "fmt"

func getDivisors(n int) []int {
	var divisors []int;
	for i := 1; i <= n/2; i++ {
		if n % i == 0 {
			divisors = append(divisors, i);
		}
	}
	return divisors;
}

func sumArray(arr []int) int {
	var n = 0;
	for _, e := range arr {
		n += e;
	}
	return n;
}

func isAmicablePair(n int) bool {
	var a, b = 0, 0;
	a = sumArray(getDivisors(n));
	b = sumArray(getDivisors(a));
	return n == b && n != a;
}

func main() {
	var amicableNums []int;
	for i := 1; i < 10000; i++ {
		if isAmicablePair(i) {
			amicableNums = append(amicableNums, i);
		}
	}
	fmt.Println(amicableNums);
	fmt.Println(sumArray(amicableNums));
}
