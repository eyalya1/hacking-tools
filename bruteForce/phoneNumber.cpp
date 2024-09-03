#include <iostream>
#include <fstream>
#include <stdio.h>
#include <string>
using namespace std;
int main()
{
    string starts[] = {
        "050",
        "052",
        "053",
        "054",
        "058",
        "059",
        "072",
        "073"};
    ofstream file("PhoneNumbers.txt");
    for (string start : starts)
    {
        for (int i = 0; i < 10000000; i++)
        {
            file << start;
            file << i;
            file << "\n";
        }
        printf("%s\n", start.c_str());
    }
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