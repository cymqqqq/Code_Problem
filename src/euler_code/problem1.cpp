/*
A palindromic number reads the same both ways.
The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/
#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
using namespace std;
bool pal(string a) {
    int len = a.size() - 1;
    int low = 0;
    while(low < len) {
        if(a[low] != a[len]) {
            return false;
        }
        low++;
        len--;
    }
    return true;
}
int main() {
    vector<string> a;
    for(int i = 100; i < 1000; i++) {
        a.push_back(to_string(i));
    }
    int len_b = a.size();
    vector<int> b(len_b);
    for(int i = 0; i < len_b - 1; i++) {
        for(int j = 1; j < len_b - 1; j++) {
            int pa = stoi(a[i]) * stoi(a[j]);
            if(pal(to_string(pa))) {
                b.push_back(pa);
            }
        }
    }
    sort(b.begin(),b.end());
    cout <<"the largest pal is" <<b[b.size() - 1];

    return 0;
}
