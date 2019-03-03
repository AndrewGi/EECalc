#pragma once
#include <memory>
#include <map>
namespace EECalc {

	using default_real_t = double; 
	template<typename Real = default_real_t> //Might change from a template to just a 'using Real =' statement?
	class Math {
	public:
		struct Value {
			struct UnassignedException : std::invalid_argument{
				UnassignedException(Real attempted_real) : std::invalid_argument(
					std::string("attempted to assign '") + std::to_string(attempted_real) + "' to an unassignable"
				) {}
			};
			const Unit unit;
			virtual Real as_real() = 0;
			virtual void set_real_value(Real r) { //Override this if the value is Assignedable (ex: mutable variables)
				throw UnassignedException(r); 
			};
			Value(const Unit& uint) : unit(unit) {}
			Value() : Value(Unit::BaseUnit::Scalar) {}
			using P = std::unique_ptr<Value>;
		};
		struct Constant : Value {
			const Real value;
			Real as_real() override {
				return value;
			}
			Constant(Real value, Unit unit = Unit::BaseUnit::Scalar) : Value(unit) {}
			using P = std::unique_ptr<Constant>;
		};
		static typename Constant::P evalute(Value& value) {
			return std::make_unique<Constant>(value.as_real(), value.unit);
		}
		struct BinaryOperation : Value {

			enum class Operator {
				Add,
				Subtract,
				Multiply,
				Divide,
				UpdateAssignable,
			};
			static Unit get_unit(const Value& left, Operator operation, const Value& right) {
				switch (operation) {
				case Operator::UpdateAssignable:
				case Operator::Add:
				case Operator::Subtract:
					if (left.unit == right.unit)
						return left.unit;
					throw Unit::UnitMismatchException();
				case Operator::Multiply:
					return left.unit * right.unit;
				case Operator::Divide:
					return left.unit / right.unit;
					
				default:
					throw std::invalid_argument("unrecognized operator");

				}
			}
			typename Value::P left;
			typename Value::P right;
			const Operator operation;
			BinaryOperation(typename Value::P left, Operator operation, typename Value::P right) :
				left(std::move(left)), right(std::move(right)), operation(operation),
				Value(get_unit(*left, operation, *right)) {
				
			}
			Real as_real() {
				switch (operation) {
				case Operator::Add:
					return left->as_real() + right->as_real();
				case Operator::Subtract:
					return left->as_real() - right->as_real();
				case Operator::Multiply:
					return left->as_real() * right->as_real();
				case Operator::Divide:
					return left->as_real() / right->as_real();
				case Operator::UpdateAssignable: // Calling as_real for Operator::UpdateAssignable can and will cause mutations to variables
					left->set_real_value(right->as_real());
					return left;
				default:
					throw std::logic_error("invalid operator");
				}
			}
		};
		static typename Value::P make_operation(typename Value::P left, typename BinaryOperation::Operator operation, typename Value::P right) {
			std::unique_ptr<BinaryOperation> op = std::make_unique<BinaryOperation>(std::move(left), operation, std::move(right));
			if (typeid(*(op.left.get())) == typeid(Constant)
				&& typeid(*(op.right.get())) == typeid(Constant)) {
				//Both operators are constant so we can optimze away the expression
				return evalute(*op);
			}
			return op;
		}
		struct UnaryOperation : Value {
			enum class Operator {
				Negate,
				AbsoluteValue,
			};
			Value::P operand;
			Operator operation;
			UnaryOperation(Operator operation, typename Value::P operand) :
				operation(operation), operand(std::move(operand)), Value(operand->unit) {}
			Real as_real() {
				switch (operation) {
				case Operator::Negate:
					return -operand->as_real();
				case Operator::AbsoluteValue:
					Real r = operand->as_real();
					if (r < 0)
						return -r;
					return r;
				default:
					throw std::logic_error("invalid unary operator");
				}
			}
		};

		static typename Value::P make_operation(typename UnaryOperation::Operator operation, typename Value::P operand) {
			std::unique_ptr<UnaryOperation> op = std::make_unique<UnaryOperation>(operation, std::move(operand));
			if (typeid(*(op.operand.get())) == typeid(Constant)) {
				//Constant operand so lets optimze the operator away
				return evalute(*op);
			}
			return op;
		}
		/**
			Use proxy class 'Variable' instead of directly calling VariableBank methods
		*/
		struct VariableBank {
			struct UndefinedVariable : std::logic_error {
				UndefinedVariable(std::string variable_name) : std::logic_error(std::string("undefined variable : ") + variable_name) {}
			}
			struct VariableEntry {
				const Unit unit;
				Real value;
				VariableEntry& operator=(typename Value::P& new_value) {
					unit = new_value;
					value = new_value.as_real();
				}
			}
			std::map<std::string, VariableEntry, std::less<>> vars;
			VariableEntry& get(std::string_view name) {
				if (auto& var_i = vars.find(name); i != vars.end())
					return *var_i;
				throw UndefinedVariable(name);
			}
			void update(std::string_view name, typename Value::P& new_value) {
				
			}
		};
		struct Variable : Value {
			const std::string_view name;
			typename VariableBank::VariableEntry& owner_entry;
			Variable(std::string_view name, VariableBank& bank) : name(name), owner_entry(bank.get(name))
			Real as_real() {
				return owner_entry.value;
			}
		};
		std::unique<Variable> make_variable(std::string_view name, VariableBank& bank) {
			return std::make_unique<Variable>(name, bank);
		}
		
	};
}