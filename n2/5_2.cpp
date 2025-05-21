#include <iostream>
#include <vector>
#include <string>
#include <cstdlib>
#include <algorithm>

using namespace std;

string ticket() {
    int num = rand() % 1000;
    string t = "T";
    if (num < 10) t += "00";
    else if (num < 100) t += "0";
    t += to_string(num);
    return t;
}

int main() {
    int n;
    cout << "Доступные команды: ENQUEUE, DISTRIBUTE\n";
    cout << "Введите количество окон: ";
    cin >> n;

    vector<pair<string, int>> tasks;
    string command;

    while (true) {
        cout << "Команда: ";
        cin >> command;

        if (command == "ENQUEUE") {
            int time;
            cin >> time;
            string tick = ticket();
            tasks.push_back(make_pair(tick, time));
            cout << tick << endl;
        }
        else if (command == "DISTRIBUTE") {
            sort(tasks.begin(), tasks.end(), [](const pair<string, int>& a, const pair<string, int>& b) {
                return a.second > b.second;
            });

            vector<vector<string>> window_queue(n);
            vector<int> window_time(n, 0);

            for (size_t i = 0; i < tasks.size(); ++i) {
                const string& tick = tasks[i].first;
                int time = tasks[i].second;
                int min_index = 0;
                for (int j = 1; j < n; ++j) {
                    if (window_time[j] < window_time[min_index])
                        min_index = j;
                }

                window_queue[min_index].push_back(tick);
                window_time[min_index] += time;
            }

            for (int i = 0; i < n; ++i) {
                cout << "Окно " << i + 1 << " (" << window_time[i] << " минут): ";
                for (size_t j = 0; j < window_queue[i].size(); ++j) {
                    cout << window_queue[i][j];
                    if (j < window_queue[i].size() - 1) cout << ", ";
                }
                cout << "\n";
            }
            break;
        }
        else {
            cout << "Неизвестная команда" << endl;
        }
    }

    return 0;
}
//
