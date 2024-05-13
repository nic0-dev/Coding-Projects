#include <iostream>
#include <sstream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm> 
#include <cctype> 
#include <cmath>
#include <chrono> 
using namespace std;

string movie_names[398];
double genre_rating[398][20];
double user_rating[20];

void load_movie_data() {
    ifstream file("movie_features.csv");
    string line, data;
    int row = 0;

    if(file.is_open()){
        getline(file, line);                            // skip header line
        while(getline(file, line)) {
            size_t start = 0, end = line.find(',');
            if (line[0] == '"') {                       // title contains a comma
                end = line.find('"', 1) + 1;
                movie_names[row] = line.substr(1, end - 2);
            }
            else
                movie_names[row] = line.substr(0, end);
            start = end + 1;
            int index = 0;
            while ((end = line.find(',', start)) != string::npos) {
                genre_rating[row][index++] = line[start] - '0';
                start = end + 1;
            }
            genre_rating[row][index] = line[start] - '0';
            row++;
        }
        file.close();
    }
}

void load_user_data(int n) {
    ifstream file("user_features.csv");
    string line, data;
    int row = 0;

    if(file.is_open()) {
        while (getline(file, line) && row < n) 
            row++;
        size_t start = 0, end;
        int index = 0;
        while ((end = line.find(',', start)) != string::npos) {
            if (index > 1) user_rating[index - 2] = stod(line.substr(start, end - start));
            index++;
            start = end + 1;
        }
        user_rating[index - 2] = stod(line.substr(start));
        file.close();
    }
}

struct MovieData {
    string name;
    int index;
    double rating;

    MovieData(const string& name, int index, const double* genre_ratings, const double* user_ratings) {
    this->name = name;
    this->index = index;
    this->rating = 0.0;
    for (int i = 0; i < 20; ++i) 
        rating += genre_ratings[i] * user_ratings[i];
    }

    // operator for sorting by dot product (descending) and name (alphabetical)
    bool operator<(const MovieData& other) const {
        const double epsilon = 1.0e-5; 
        if (fabs(rating - other.rating) > epsilon) {
            return rating > other.rating;
        }
        return name < other.name; 
    }
};

int main() {
    string user_id;
    cin >> user_id;

    chrono::time_point<std::chrono::system_clock> start, end;
    start = chrono::system_clock::now();

    load_movie_data();
    load_user_data(stoi(user_id.substr(1)));

    vector<MovieData> movies;
    for (int i = 0; i < 398; ++i) {
        movies.emplace_back(movie_names[i], i, genre_rating[i], user_rating);
    }
    sort(movies.begin(), movies.end());
    for (int i = 0; i < 10; ++i) {
        cout << movies[i].name << "\n";
    }

    end = chrono::system_clock::now();
 
    chrono::duration<double> elapsed_seconds = end - start;
    time_t end_time = chrono::system_clock::to_time_t(end);
 
    cout << "Elapsed time: " << elapsed_seconds.count() << "s\n";
    return 0;
}