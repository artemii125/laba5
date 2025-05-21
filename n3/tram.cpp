#include "tram.h"
#include <iostream>
using namespace std;

void Tram::create_tram(const string& tram, const vector<string>& stops) {
    if (tram_to_stops.count(tram)){
        cout << "Трамвай с номером " << " уже существует." << endl; 
    }
    if (stops.size() < 2) {
        cout << "У трамвая должно быть как минимум 2 остановки." << endl;
        return;
    }

    vector<string> cleaned_stops;
    for (const auto& stop : stops) {
        bool exists = false;
        for (const auto& existing : cleaned_stops) {
            if (existing == stop) {
                exists = true;
                break;
            }
        }
        if (!exists) {
            cleaned_stops.push_back(stop);
        }
    }

    if (cleaned_stops.size() < 2) {
        cout << "Были добавлены только 2 одинаковых остановки." << endl;
        return;
    }

    tram_to_stops[tram] = cleaned_stops;
    for (const auto& stop : cleaned_stops) {
        stop_to_trams[stop].insert(tram);
    }
}

void Tram::print_trams_stop(const string& stop, ostream& out) const {
    if (tram_to_stops.empty()) {
        out << "Trams is absent" << endl;
        return;
    }
    if (stop_to_trams.count(stop)) {
        for (const auto& tram : stop_to_trams.at(stop)) {
            out << tram << " ";
        }
        out << endl;
    } else {
        out << "Stop is absent" << endl;
    }
}


void Tram::print_stops_tram(const string& tram, ostream& out) const {
    if (tram_to_stops.empty()) {
        out << "Trams is absent" << endl;
        return;
    }
    if (!tram_to_stops.count(tram)) {
        out << "No tram" << endl;
        return;
    }
    for (const auto& stop : tram_to_stops.at(tram)) {
        out << "Stop " << stop << ": ";
        bool empty = true;
        for (const auto& other : stop_to_trams.at(stop)) {
            if (other != tram) {
                out << other << " ";
                empty = false;
            }
        }
        if (empty) out << "0";
        out << endl;
    }
}
void Tram::print_trams(ostream& out) const {
    if (tram_to_stops.empty()){
        out << "Trams is absent" << endl;
    }
    for (const auto& [tram, stops] : tram_to_stops) {
        out << "TRAM " << tram << ": ";
        for (const auto& stop : stops) {
            out << stop << ", ";
        }
        out << endl;
    } 
}
