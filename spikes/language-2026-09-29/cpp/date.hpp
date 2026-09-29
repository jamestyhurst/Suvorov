#pragma once

namespace suvorov {

struct Date {
    int year;
    int month;
    int day;

    bool valid() const;
    void advance_one_day();
};

bool operator==(const Date& lhs, const Date& rhs);
bool operator!=(const Date& lhs, const Date& rhs);
bool operator<(const Date& lhs, const Date& rhs);
bool operator<=(const Date& lhs, const Date& rhs);
bool operator>(const Date& lhs, const Date& rhs);
bool operator>=(const Date& lhs, const Date& rhs);

int biological_age(const Date& birth, const Date& on);

}  // namespace suvorov
