#include <bits/stdc++.h>
using namespace std;

int main(){
    ios_base::sync_with_stdio(false);
    cin.tie(NULL);

    #ifndef ONLINE_JUDGE
    freopen("input.txt", "r", stdin);
    freopen("output.txt", "w", stdout);
    #endif
    
    int t; cin >> t;
    while(t--) {
        int n; cin >> n;
        if(n > 52)
            cout << (char)('a' + n - 53) << "zz" << endl;
        else if(n > 26)
            cout << "a" << (char)('a' + n - 28) << "z" << endl;
        else
            cout << "aa" << (char)('a' + n - 3) << endl;
            
    }
    return 0;
}