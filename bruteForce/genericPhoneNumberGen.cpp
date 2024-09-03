#include <iostream>
#include <fstream>
#include <stdio.h>
#include <string>
#include <list>
using namespace std;
string padZero(int num);
int main(int argc, char *argv[])
{
    list<string> phoneNumbersStart;
    if (argc > 1)
    {
        for (int i = 1; i < argc; i++)
        {
            phoneNumbersStart.push_back(argv[i]);
        }
    }
    else
    {
        ifstream ReadFile("phoneNumbersStarts.txt");
        string line;
        int count = 0;
        while (getline(ReadFile, line))
        {
            count++;
            phoneNumbersStart.push_back(line);
        }
        ReadFile.close();
    }
    ofstream WriteFile("interPhoneNumbers.txt");
    string WriteText = "";
    for (string start : phoneNumbersStart)
    {
        for (int i = 0; i < 10000000; i++)
        {
            WriteText.append(start);
            WriteText.append(padZero(i));
            WriteText.append("\n");
        }
        printf("%s\n", start.c_str());
    }
    WriteFile << WriteText;
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