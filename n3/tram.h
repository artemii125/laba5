#pragma once
#include <string>
#include <vector>
#include <map>
#include <set>
#include <fstream>
using namespace std;
class Tram {
private:
    map<string, vector<string>> tram_to_stops;
    map<string, set<string>> stop_to_trams;

public:
    void create_tram(const string& tram, const vector<string>& stops);
    void print_trams_stop(const string& stop, ostream& out) const;
    void print_stops_tram(const string& tram, ostream& out) const;
    void print_trams(ostream& out) const;
};