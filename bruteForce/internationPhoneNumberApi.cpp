#include <iostream>
#include <fstream>
#include <stdio.h>
#include <string>
#include <list>
using namespace std;

int main()
{
    ifstream ReadFile("phoneNumbersStarts.txt");
    ofstream WriteFile("interPhoneNumbers.txt");
    string line;
    int count = 0;
    list<string> phoneNumbersStart;
    while (getline(ReadFile, line))
    {
        count++;
        phoneNumbersStart.push_back(line);
    }
    for (string start : phoneNumbersStart)
    {
        for (int i = 0; i < 10000000; i++)
        {
            file << start;
            file << padZero(i);
            file << "\n";
        }
        printf("%s\n", start.c_str());
    }
    ReadFile.close();
    WriteFile.close();
    return 0;
}
string padZero(int num)
{
    string str = to_string(num);
    while (str.length() < 7)
    {
        str = "0" + str;
    }
    return str;
}