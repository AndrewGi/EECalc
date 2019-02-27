#pragma once

#include <exception>
#include <stdexcept>
#include <string>
#include <string_view>
#include <array>
#include <tuple>

namespace EECalc {
	
	class Unit {
	public:
		enum class BaseUnit {
			Null,
			Scalar,
			Volts,
			Amps,
			Ohms,
			Watts,
			Joules,
			Newtons,
			Seconds,
			Meters,
			Henries,
			Farads,
			MetersPerSecond,
			End
		};
		struct BaseUnitInfo {
			const BaseUnit baseunit;
			const std::string_view full_name;
			const std::string_view shorthand;
		};
		constexpr static std::array<BaseUnitInfo, static_cast<size_t>(BaseUnit::End)> unit_infos = { {
			{BaseUnit::Null, "NULL", "?"},
			{BaseUnit::Scalar, "Scalar", ""},
			{BaseUnit::Volts, "Volt", "V"},
			{BaseUnit::Amps, "Amp", "I"},
			{BaseUnit::Ohms, "Ohm", "R"},
			{BaseUnit::Watts, "Watt", "W"},
			{BaseUnit::Joules, "Joule", "J"},
			{BaseUnit::Newtons, "Newton", "N"},
			{BaseUnit::Seconds, "Second", "s"},
			{BaseUnit::Meters, "Meters", "m"},
			{BaseUnit::Henries, "Henry", "H"}, //TODO: Henry->Heries is more complicated than just added an h
			{BaseUnit::Farads, "Farad", "F"},
			{BaseUnit::MetersPerSecond, "Meters per second", "m/s"}
		} };
		//TODO: Have this function save state or something to speed up next search
		/**
			BaseUnit:
				The unit that matches the shorthand
			bool:
				True if there could be another unit, just with a longer name
		**/
		static std::pair<BaseUnit, bool> find_unit(std::string_view shorthand, size_t search_start_position = 0) {
			auto in_length = shorthand.length();
			if (in_length == 0)
				return { BaseUnit::Null, false };
			BaseUnit found_unit = BaseUnit::Null;
			bool parital = false;
			for (const auto& unit_info : unit_infos) {
				auto length_to_check = unit_info.shorthand.length();
				if (in_length > length_to_check) {
					continue;
				}
				else if (in_length == length_to_check && shorthand == unit_info.shorthand) {
					found_unit = unit_info.baseunit;
				}
				else { // in_length < length_to_check
					for (size_t i = 0; i < in_length; i++) {
						if (shorthand[i] != unit_info.shorthand[i])
							continue;
						parital = true;
					}
				}
			}
			return { found_unit, parital };
		}
		template<BaseUnit _U1, BaseUnit _U2, BaseUnit _ResultU> // I, R, V ( V = R I )
		struct product_rule {
			static constexpr BaseUnit U1 = _U1;
			static constexpr BaseUnit U2 = _U2;
			static constexpr BaseUnit ResultU = _ResultU;
			constexpr static BaseUnit multiply_type(const BaseUnit u1, const BaseUnit u2) {
				if ((u1 == U1 && u2 == U2) || (u1 == U2 && u2 == U1))
					return ResultU;
				return BaseUnit::Null;
			}
			constexpr static BaseUnit divide_type(BaseUnit numer, BaseUnit denom) {
				if (numer == ResultU && denom == U1)
					return U2;
				if (numer == ResultU && denom == U2)
					return U1;
				return BaseUnit::Null;
			}
		};
		constexpr static std::tuple<
			product_rule<BaseUnit::Ohms, BaseUnit::Amps, BaseUnit::Volts>,
			product_rule<BaseUnit::Joules, BaseUnit::Seconds, BaseUnit::Watts>,
			product_rule<BaseUnit::Volts, BaseUnit::Amps, BaseUnit::Watts>,
			product_rule<BaseUnit::Newtons, BaseUnit::Meters, BaseUnit::Joules>
		> rules{};

		template<size_t _I = (std::tuple_size<decltype(rules)>::value - 1)>
		static constexpr BaseUnit rule_multiply_type(BaseUnit u1, BaseUnit u2) {
			const BaseUnit result_u = std::get<_I>(rules).multiply_type(u1, u2);
			if (result_u != BaseUnit::Null)
				return result_u;
			return rule_multiply_type<_I - 1>(u1, u2);
		}
		
		template<>
		static constexpr BaseUnit rule_multiply_type<0>(BaseUnit u1, BaseUnit u2) {
			return std::get<0>(rules).multiply_type(u1, u2);
		}

		template<size_t _I = (std::tuple_size<decltype(rules)>::value - 1)>
		static constexpr BaseUnit rule_divide_type(BaseUnit u1, BaseUnit u2) {
			BaseUnit result_u = std::get<_I>(rules).divide_type(u1, u2);
			if (result_u != BaseUnit::Null)
				return result_u;
			return rule_divide_type<_I - 1>(u1, u2);
		}

		template<>
		static constexpr BaseUnit rule_divide_type<0>(BaseUnit u1, BaseUnit u2) {
			return std::get<0>(rules).divide_type(u1, u2);
		}
		std::string_view full_name() const {
			switch (_u) {
			case BaseUnit::Scalar:
				return "Scalar";
			case BaseUnit::Volts:
				return "Volts";
			case BaseUnit::Amps:
				return "Amps";
			case BaseUnit::Ohms:
				return "Ohms";
			case BaseUnit::Watts:
				return "Watts";
			default:
				_unrecognized_unit();
			}
		}
		std::string_view short_hand() const {
			switch (_u) {
			case BaseUnit::Scalar:
				return ""; // No unit for scalars
			case BaseUnit::Volts:
				return "V";
			case BaseUnit::Amps:
				return "A";
			case BaseUnit::Ohms:
				return "R";
			case BaseUnit::Watts:
				return "W";
			default: 
				_unrecognized_unit();
			}
		}

		Unit operator*(Unit other) const {
			const BaseUnit result_u = rule_multiply_type(_u, other._u);
			if (result_u == BaseUnit::Null)
				throw UnitMismatchException(*this, other);
			return Unit(result_u);			
		}

		Unit operator/(Unit other) const {
			const BaseUnit result_u = rule_divide_type(_u, other._u);
			if (result_u == BaseUnit::Null)
				throw UnitMismatchException(*this, other);
			return Unit(result_u);
		}
		class UnitMismatchException : std::logic_error {
		public:
			UnitMismatchException(Unit u1, Unit u2) : std::logic_error(std::string("Unit mismatch between ") + std::string(u1.full_name()) + " and " + std::string(u2.full_name())) {}
		};
		Unit(BaseUnit base_unit) : _u(base_unit) {};
		Unit(std::string_view sh) : _u(shorthand_to_BaseUnit(sh)) {};
	static BaseUnit shorthand_to_BaseUnit(std::string_view sh) {
		if (sh.length() == 1) {
			char c = sh[0];
			switch (c) {
			case 0:
				return BaseUnit::Scalar;
			case 'V':
				return BaseUnit::Volts;
			case 'A':
				return BaseUnit::Amps;
			case 'W':
				return BaseUnit::Watts;
			case 'R':
				return BaseUnit::Ohms;
			default:
				break;
			}
		}
		else {

		}
		throw std::invalid_argument(std::string("Unrecognized shorthand: ") + std::string(sh));

	}
	private:
		const BaseUnit _u;
		const int64_t _unit_exponent = 1; // Used for keeping units consistent
		static void _unrecognized_unit() {
			throw std::invalid_argument("Unrecognized unit");
		}
	};
}