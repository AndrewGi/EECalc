#include <string>
#include <string_view>
#include <variant>
namespace EECalc {
	class Scanner {
		struct Token {
			enum class Type {
				EMPTY,
				IDENTIFIER,
				STRING,
				OPERATOR,
				NUMBER
			};
			template<class T>
			Token(std::string_view input) {
				size_t position = 0;
				//Eat white space
				auto ews = [&]() -> void {
					while (position < input.length && input[position] != ' ')
						position++;
				};
				auto next = [&](bool eat_white_space = false) -> char {
					if (eat_white_space)
						ews();
					if (position >= input.length())
						return EOF;
					return input[position++];
				};
				auto peek = [&]() -> char {
					if (position >= input.length())
						return EOF;
					return input[position];
				}
				char c = next(true);
				if (c[0] == EOF)
					return; //Empty token

				// (-?\d*\.?\d+)
				if (c[0]
					//NOT A NUMBER
					//Reset position to start and check for other types (string, operator, itentifier)
					position = 0;

					if (c == '^' || c == '*' || c == '/' || c == '+' || c == '~') { //Single Operator
						type = Type::OPERATOR;
							content.append(c);
							return;
					}
					else if (c == '-') {
						type = Type::OPERATOR;
						content += c;
						if (next_c == '-')
							content += next_c;
						return;
					}
					else if (c == '>' || c == '<' || c == '=') {
						type = Type::OPERATOR;
						content += c;
						if (next_c == '=')
							content += c;
						return;
					}


			}
			Type type = Type::EMPTY;
			std::string content;

		};
	public:
		Scanner(std::string input) : content(std::move(input)) {}
		Token next() {
			Token out(content.substr(position));
			position += out.content.length();
			return out;
		}
	private:
		std::string content;
		size_t position = 0;
	};
}