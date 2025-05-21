#include <iostream>
#include <map>
#include <string>

using namespace std;

const int max_products = 10;
const int zones = 10, rack = 3, sections = 1, shelves = 5;
const int cells = zones * rack * sections * shelves;
const int all = cells * max_products;

map<string, map<string, int>> tovar;

bool check(const string& adress){
    if (adress.size() != 4) return false;
    if (adress[0] < 'A' || adress[0] > 'J') return false;
    if (adress[1] < '1' || adress[1] > '3') return false;
    if (adress[2] != '1') return false;
    if (adress[3] < '1' || adress[3] > '5') return false;
    return true;
}

void ADD(const string& name, int qty, const string& adress) {
    if (!check(adress)){
        cout << "Неправильный адрес!\n";
        cout << "\n";
        return;
    }
    int places = 0;
    if (tovar.count(adress)) {
        for (const auto& it : tovar[adress])
            places += it.second;
    }

    if (places + qty > max_products) {
        cout << "Ошибка: ячейка " << adress << " вмещает только 10 шт." << "\n";
        cout << "Сейчас: " << places << "\n";
        cout << "\n";
        return;
    }

    tovar[adress][name] += qty;
    cout << "Добавлено " << name << " в количестве " << qty << " шт. " << " в ячейку " << adress << "\n";
    cout << "\n";
}

void REMOVE(const string& name, int qty, const string& adress) {
    if (!check(adress)){
        cout << "Неправильный адрес\n";
        cout << "\n";
        return;
    }
    if (!tovar.count(adress) || !tovar[adress].count(name)) {
        cout << "Ошибка: товара " << name << " нет в ячейке " << adress << "\n";
        cout << "\n";
        return;
    }

    if (tovar[adress][name] < qty) {
        cout << "Ошибка: недостаточно " << name << " в ячейке " << adress << "\n";
        cout << "\n";
        return;
    }
    tovar[adress][name] -= qty;
    if (tovar[adress][name] == 0) {
        tovar[adress].erase(name);
        if (tovar[adress].empty()) {
            tovar.erase(adress);
        }
    }

    cout << "Удалено " << name << " в количестве " << qty << " шт. " << " из ячейки " << adress << "\n";
    cout << "\n";
}

void INFO() {
    int used = 0;
    for (const auto& [adress, contents] : tovar) {
    for (const auto& [name, qty] : contents)
            used += qty;
    }

    double percent = (100.0 * used) / cells;
    cout << "Заполненность склада: " << percent << "%\n";
    cout << "Содержимое занятых ячеек:\n";
    for (const auto& [adress, contents] : tovar) {
        cout << adress << ": ";
        for (const auto& [name, qty] : contents)
            cout << name << " = " << qty << "\n";
        cout << "\n";
    }

    int empty = cells - tovar.size();
    cout << "Пустых ячеек: " << empty << "\n";
}


int main() {
    cout << "Перечень команд: ADD, REMOVE, INFO\n\n";

    while (true) {
        cout << "Введите команду: ";
        string operation, name, adress;
        int qty;
        cin >> operation;
        if (operation == "ADD") {
            cin >> name >> qty >> adress;
            ADD(name, qty, adress);
        } 
        else if (operation == "REMOVE") {
            cin >> name >> qty >> adress;
            REMOVE(name, qty, adress);
        } 
        else if (operation == "INFO") {
            INFO();
        }
        else {
            cout << "Неизвестная команда\n\n";
        }
    }

    return 0;
}
