#pragma once
#include "Units.hpp"
#include <string_view>
#include <string>
#include <exception>
#include <vector>

namespace EECalc {

	class Tokenizer {
	public:
		class Token {
		public:
			enum class Type {
				Null,
				NUMBER,
				WORD,
				OPERATOR,
				END_OF_INPUT,
			};
			const std::string_view content;
			const Type type;
			constexpr Token(Type type, std::string_view content) : content(content), type(type) {};
			constexpr Token() : Token(Type::Null, "") {}
			class UnexpectSymbol : std::logic_error {
			public:
				UnexpectSymbol(char character) : logic_error((std::string("Unexpected symbol ") + character).c_str()) {}
			};
			std::string_view type_name() const {
				switch (type) {
				case Type::Null:
					return "Null";
				case Type::NUMBER:
					return "Number";
				case Type::WORD:
					return "Word";
				case Type::OPERATOR:
					return "Operator";
				case Type::END_OF_INPUT:
					return "EOF";
				default:
					return "????";
				}
			}
			static Tokenizer::Token null_token;
	};
	Tokenizer(std::string_view input_str) : str(input_str) {}
	
	private:
		std::string str;
		size_t position = 0;
		struct cursor {
			std::string_view str;
			size_t current_position;
			size_t start_position = current_position;
			char next() {
				if (current_position >= str.length())
					return EOF;
				return str[current_position++];
			}
			char peek(size_t peek_offset = 0) const {
				if ((peek_offset + current_position) >= str.length())
					return EOF;
				return str[current_position + peek_offset];
			}
			char rewind() {
				if (current_position==0)
					return EOF;
				return str[--current_position];
			}
			std::string_view get_view() const {
				return str.substr(start_position, current_position - start_position);
			}
		};
	public:
		void update_to(const cursor& c) {
			position = c.current_position;
		}
		cursor get_cursor() const {
			return cursor{ str, position };
		}
		
		Token next_token() {
			if (position == str.length())
				return Token(Token::Type::END_OF_INPUT, "");
			while (str[position] == ' ')
				position++;
			if (position == str.length())
				return Token(Token::Type::END_OF_INPUT, "");
			cursor c = get_cursor();
			char in_char = 0;
			Token::Type token_type = Token::Type::Null;
			auto try_set = [&](Token::Type type) {
				if (token_type == Token::Type::Null) {
					token_type = type;
					return false;
				}
				else if (token_type != type) {
					c.rewind();
					return true;
				}
				return false; //Otherwise it's already set as the type
			};
			for (;;) {
				in_char = c.next();
				switch (in_char) {
				case '0':
				case '1':
				case '2':
				case '3':
				case '4':
				case '5':
				case '6':
				case '7':
				case '8':
				case '9':
					if (try_set(Token::Type::NUMBER))
						break;
					continue;
				case '-':
					if (c.peek() == '.' || isdigit(c.peek()))
						if (try_set(Token::Type::NUMBER))
							break;
						else
							continue;
				case '/':
				case '*':
				case '+':
				case '(':
				case ')':
				case ',':
					if (try_set(Token::Type::OPERATOR))
						break;
					continue;
				case ' ':
					try_set(Token::Type::Null);
					break;
				case '.':
					if (token_type == Token::Type::NUMBER)
						continue;
				case EOF:
					break;
				default:
					if (isalpha(in_char)) {
						if (try_set(Token::Type::WORD))
							break;
						continue;
					}
					throw Token::UnexpectSymbol(in_char);
				}
				break;
			}
			// If we made it here, we broke outta the loop insteading of throwing
			update_to(c);
			if (token_type == Token::Type::Null) { //We got a space
				return next_token();
			}
			return Token(token_type, c.get_view());
		}
		std::vector<Token> extract_tokens() {
			std::vector<Token> tokens;
			for(;;) {
				Token current_token = next_token();
				if (current_token.type == Token::Type::END_OF_INPUT)
					break;
				tokens.push_back(current_token);
			}
			return tokens;
		}
	};
}

std::ostream& operator<<(std::ostream& os, const EECalc::Tokenizer::Token& token) {
	return os << token.type_name() << ":\"" << token.content << '"' ;
}
EECalc::Tokenizer::Token EECalc::Tokenizer::Token::null_token;