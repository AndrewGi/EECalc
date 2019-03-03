#pragma once
#include "Parser.hpp"
#include "Math.hpp"

namespace EECalc {
	class Builder {
	public:
		std::vector<std::variant<typename Math<>::Value, typename Parser::Token>> tokens;
		using Iterator = typename decltype(tokens)::iterator;
		Builder(decltype(tokens) tokens) : tokens(std::move(tokens)) {}
	private:
		std::vector<std::variant<typename Value::P, typename Parser::Token>> tokens;
		template<class TokenT, class CallableT>
		void for_each_token(CallableT callable) {
			for (auto i = tokens.begin(); i < tokens.end(); i++) {
				if (std::holds_alternative<typename Parser::Token>(*i)) {
					std::visit([&](const auto& val) {
						if constexpr (std::is_same_v<TokenT>, decltype(val) > )
							i = callable(i);
					}, i);
				}
			}
		};
		Iterator reduce(const Iterator i) {
			if (std::holds_alternative<typename Value::P>(*i))
				throw std::invalid_argument("reduce called on Value type");

			typename Parser::tokens tok = std::get<typename Parser::Token>(*i);
			std::visit([&](const auto& val) {
				using T = decltype(val);
				if constexpr (std::is_same_v<double, T>) { //TOK == double
					typename Parser::tokens next_tok = std::get<Parser::Token>(*(i + 1));
					if (std::holds_alternative<Unit>(next_tok)) {
						//We have (DOUBLE UNIT) so lets make a const
						*i = std::make_unique<Constant>(val, std::get<Unit>(next_tok));
						tokens.erase(i + 1); //Erase the Unit
						return i;
					}
					//Otherwise we just have a scalar
					*i = std::make_unique<Constant>(val);
					return i;
				}

				if constexpr (std::is_same_v<Unit, T>) { //TOK == Unit

				}

				if constexpr (std::is_same_v<Parser::Operator, T>) { //TOK == Operator

				}
			}, tok);
			return;
		}
	public:
		void parse() {
			for_each_token<double>([&](Iterator i) {
				reduce(i);
			});
			for_each_token<Operator>([&](Iterator i) {
				if (*i == Parser::Operator::Add
			}
		}
	};
}