#include <iostream>
#include <fstream>
#include <stdio.h>
// #include <ifstream>
bool isValidId(long id);
using namespace std;
int main()
{
    ofstream file("numbers.txt");
    // printf("Hello World\n");
    long n;
    long i = 0;
    while (i < 10000000000)
    {
        n = 0;
        while (n++ < 1000000000)
        {
            if (isValidId(i))
            {
                file << i;
                file << "\n";
            }
            i++;
            // file << i;
            // file << "\n";
        }
        printf("%ld\n", i);
    }
    file.close();
    return 0;
}
bool isValidId(long id)
{
    int sum = 0;
    while (id > 0)
    {
        sum += id % 10;
        id /= 10;
    }
    return sum % 10 == 0;
}