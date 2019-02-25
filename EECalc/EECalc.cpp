// EECalc.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include "pch.h"
#include <iostream>
#include "Tokenizer.hpp"
#include "Parser.hpp"
#include "Math.hpp"

int main()
{
	std::string input = "1W * 10s";
	EECalc::Tokenizer toker(input);
	EECalc::Parser parser(toker.extract_tokens());
	std::cout << parser.next_token();
	std::cin.ignore();
	return 0;
}
