#include <iostream>
#include <fstream>
#include <sstream>
#include "tram.h"
using namespace std;

enum class Type {
    CREATE_TRAM,
    TRAMS_IN_STOP,
    STOPS_IN_TRAM,
    TRAMS
};

map<string, Type> commandMap = {
    {"CREATE_TRAM", Type::CREATE_TRAM},
    {"TRAMS_IN_STOP", Type::TRAMS_IN_STOP},
    {"STOPS_IN_TRAM", Type::STOPS_IN_TRAM},
    {"TRAMS", Type::TRAMS}
};

bool Command(const string& command) {
    return commandMap.count(command);
}
int main() {
    Tram manager;
    cout << "Доступные команды: CREATE_TRAM, TRAMS_IN_STOP, STOPS_IN_TRAM, TRAMS" << endl;

    string command;
    while (cin >> command) {
        if (command == "CREATE_TRAM") {
            string tram;
            int stopcount;
            cin >> tram >> stopcount;
            vector<string> stops;
            string stop;
            for (int i = 0; i < stopcount && cin >> stop; ++i) {
                stops.push_back(stop);
            }
            if (stops.size() != stopcount) {
                cout << "Введено меньше остановок, чем указано." << endl;
            } else {
                manager.create_tram(tram, stops);
            }
        } else if (command == "TRAMS_IN_STOP") {
            string stop;
            cin >> stop;
            manager.print_trams_stop(stop, cout);
        } else if (command == "STOPS_IN_TRAM") {
            string tram;
            cin >> tram;
            manager.print_stops_tram(tram, cout);
        } else if (command == "TRAMS") {
            manager.print_trams(cout);

        } else {
            cout << "Неизвестная команда: " << command << endl;
        }
    }
    return 0;
}
    
