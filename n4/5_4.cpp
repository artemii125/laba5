#include <iostream>
#include <map>
#include <set>
#include <string>

using namespace std;

int main() {
    int n;
    cin >> n;

    map<string, string> regions;
    set<string> deleted_names;

    for (int i = 0; i < n; ++i) {
        string command;
        cin >> command;

        if (command == "CHANGE") {
            string region, new_center;
            cin >> region >> new_center;

            if (regions.count(region)) {
                string old_center = regions[region];
                if (old_center == new_center) {
                    cout << "Incorrect" << endl; //новый = старый
                } else {
                    regions[region] = new_center;
                    cout << "Region " << region << " has changed its administrative center from "
                         << old_center << " to " << new_center << endl;
                }
            }
            else if (deleted_names.count(region)) {
                cout << "Incorrect" << endl; //переименнованый нельзя использ 
            }
            else {
                regions[region] = new_center;
                cout << "New region " << region << " with administrative center " << new_center << endl;
            }

        } else if (command == "RENAME") {
            string old_region, new_region;
            cin >> old_region >> new_region;

            if (old_region == new_region || !regions.count(old_region) || regions.count(new_region)) {
                cout << "Incorrect" << endl; //если = старый или не сущ или новый регион использ
            } else {
                regions[new_region] = regions[old_region];
                regions.erase(old_region);
                deleted_names.insert(old_region);
                cout << old_region << " has been renamed to " << new_region << endl;
            }

        } else if (command == "ABOUT") {
            string region;
            cin >> region;

            if (regions.count(region)) {
                cout << region << " has administrative center " << regions[region] << endl;
            } else {
                cout << "Incorrect" << endl; //если регион не найден
            }

        } else if (command == "ALL") {
            for (const auto& item : regions) {
                cout << item.first << " - " << item.second << endl;
            }
        }
    }

    return 0;
}
