#include "suvorov/core/date.hpp"

#include <stdexcept>

namespace suvorov {
namespace {

bool is_leap_year(int year) {
    return (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
}

int days_in_month(int year, int month) {
    switch (month) {
    case 2:
        return is_leap_year(year) ? 29 : 28;
    case 4:
    case 6:
    case 9:
    case 11:
        return 30;
    default:
        return 31;
    }
}

}  // namespace

bool Date::valid() const {
    if (month < 1 || month > 12 || day < 1) {
        return false;
    }
    return day <= days_in_month(year, month);
}

void Date::advance_one_day() {
    ++day;
    if (day <= days_in_month(year, month)) {
        return;
    }

    day = 1;
    ++month;
    if (month <= 12) {
        return;
    }

    month = 1;
    ++year;
}

bool operator==(const Date& lhs, const Date& rhs) {
    return lhs.year == rhs.year && lhs.month == rhs.month && lhs.day == rhs.day;
}

bool operator!=(const Date& lhs, const Date& rhs) {
    return !(lhs == rhs);
}

bool operator<(const Date& lhs, const Date& rhs) {
    if (lhs.year != rhs.year) {
        return lhs.year < rhs.year;
    }
    if (lhs.month != rhs.month) {
        return lhs.month < rhs.month;
    }
    return lhs.day < rhs.day;
}

bool operator<=(const Date& lhs, const Date& rhs) {
    return lhs < rhs || lhs == rhs;
}

bool operator>(const Date& lhs, const Date& rhs) {
    return rhs < lhs;
}

bool operator>=(const Date& lhs, const Date& rhs) {
    return rhs <= lhs;
}

int biological_age(const Date& birth, const Date& on) {
    if (!birth.valid() || !on.valid()) {
        throw std::invalid_argument("Age date is invalid");
    }
    if (on < birth) {
        throw std::invalid_argument("Birth date is after the age date");
    }

    int years = on.year - birth.year;
    if (on.month < birth.month ||
        (on.month == birth.month && on.day < birth.day)) {
        --years;
    }
    return years;
}

}  // namespace suvorov
