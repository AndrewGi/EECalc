#pragma once
#include <vector>
#include <variant>
#include <charconv>
#include <string_view>
#include "Math.hpp"
#include "Tokenizer.hpp"
#include "Units.hpp"

using namespace std::string_literals;

namespace EECalc {

	class Parser {
	public:
		struct Unexpected : std::logic_error {
			Unexpected(std::string msg, Tokenizer::Token tok) : std::logic_error(msg + ": " + std::string(tok.content)) {};
		};
		enum class Operator {
			Add,
			Subtract,
			Negate,
			Multiply,
			Divide,
			Raise,
			OpenParenthese,
			CloseParenthese
		};
		Operator what_operator(std::string_view str) {
			if (str.length() == 1) {
				switch (str[0]) {
				case '+':
					return Operator::Add;
				case '-':
					return Operator::Subtract;
				case '/':
					return Operator::Divide;
				case '*':
					return Operator::Multiply;
				case '^':
					return Operator::Raise;
				case '(':
					return Operator::OpenParenthese;
				case ')':
					return Operator::CloseParenthese;
				default:
					break;
				}
			}
			throw std::invalid_argument("unrecognized operator");
			
		}
		using Token = std::variant<Math<>::Value::P, Operator, std::string_view>;
	private:
		std::vector<Tokenizer::Token> tokens;
		size_t position = 0;
		const Tokenizer::Token& next() {
			if (position == tokens.size())
				return Tokenizer::Token::null_token;
			return tokens[position++];
		}
		const Tokenizer::Token& peek(size_t offset=0) const {
			if ((position+offset) >= tokens.size())
				return Tokenizer::Token::null_token;
			return tokens[position+offset];
		}
		const Tokenizer::Token& peek_behind() const {
			if (position == 0)
				return Tokenizer::Token::null_token;
			return tokens[position - 1];
		}
	public:
		Token next_token() {
			using TTToken = Tokenizer::Token::Type;
			const auto& current_tok = next();
			const auto& content = current_tok.content;
			const auto& behind_tok = peek_behind();
			switch (current_tok.type) {
				case TTToken::OPERATOR: {
					Operator current_operator = what_operator(content);
					//Check for unary operator
					if (behind_tok.type == TTToken::OPERATOR || behind_tok.type == TTToken::Null) {
						//We got a unary operator
						if (current_operator == Operator::Subtract)
							return Operator::Negate;
						//Unregoginzed unary operator
						throw Unexpected("unrecognized unary", current_tok);
					}
					return current_operator; //Return the operator
				}
				case TTToken::NUMBER: {
					double real_value = NAN;
					if (std::from_chars(content.data(), content.data() + content.length(), real_value).ec == std::errc::invalid_argument)
						throw Unexpected("unrecognized number", current_tok);

					//Check for unit
					auto[base_unit, is_more] = Unit::find_unit(content, 0);
					if (base_unit == Unit::BaseUnit::Null) {
						//We matched no unit so treat it as a variable
						return content;
					}
					size_t search_position_count = content.length();
					while (is_more) {
						const auto next_token = peek(); //Peek next token
						const auto[new_unit, even_more] = Unit::find_unit(next_token.content, search_position_count);
						is_more = even_more;
						if (new_unit != Unit::BaseUnit::Null) { //We captured something
							base_unit = new_unit;
							next(); //Consume it
							search_position_count += next_token.content.length();
						}
					}
					return real_value;
				}
				case TTToken::WORD: {
					//Check for unit
					
					return base_unit;
				}

				default:
					break;
			}

			throw Unexpected("unhandled token", current_tok); //Unrecognized token

		}
	public:
		Parser(std::vector<Tokenizer::Token> tokens) : tokens(std::move(tokens)) {}
	};
}

std::ostream& operator<<(std::ostream& os, const EECalc::Parser::Token& tok) {
	std::visit([&](const auto& content) {
		using T = decltype(content);
		if constexpr (std::is_same_v<T, double>)
			os << std::to_string(content);
		else
			os << content;
	}, tok);
	return os;
}