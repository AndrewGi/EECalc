#pragma once
#include "Parser.hpp"
#include <memory>
namespace EECalc {

	using default_real_t = double;
	template<typename Real = default_real_t>
	class Math {
	public:
		struct Value {
			const Unit unit;
			virtual Real as_real() = 0;
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
			};
			static Unit get_unit(const Value& left, Operator operation, const Value& right) {
				switch (operation) {
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
			Value::P left;
			Value::P right;
			const Operator operation;
			BinaryOperation(Value::P left, Operator operation, Value::P right) :
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
				default:
					throw std::logic_error("invalid operator");
				}
			}
		};
		static typename Value::P make_operation(Value::P left, BinaryOperation::Operator operation, Value::P right) {
			std::unique_ptr<BinaryOperation> op = std::make_unique<BinaryOperation>(std::move(left), operation, std::move(right));
			if (typeid(*(op.left.get())) == typeid(Constant)
				&& typeid(*(op.right.get())) == typeid(Consant)) {
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
		static typename Value::P make_operation(UnaryOperation::Operator operation, Value::P operand) {
			std::unqiue_ptr<UnaryOperation> op = std::make_unique<UnaryOperation>(operation, std::move(operand));
			if (typeid(*(op.operand.get())) == typeid(Constant)) {
				//Constant operand so lets optimze the operator away
				return evalute(*op);
			}
			return op;
		}
	};
}