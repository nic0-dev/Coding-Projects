#include <Eigen/Sparse>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <algorithm>
#include <chrono>

using namespace Eigen;
using namespace std;

vector<string> movie_names;
vector<SparseVector<double>> genre_ratings;
VectorXd user_ratings(20); // Assume user ratings are dense

void load_movie_data() {
    ifstream file("C:/Users/joshc/Documents/COE163/movie_features.csv");
    string line;
    getline(file, line); // Skip header
    int row = 0;

    while (getline(file, line)) {
        SparseVector<double> rating(20); // There are 20 genres
        size_t start = 0, end = line.find(',');

        // Handle movie name
        string movie_name = (line[0] == '"') ? line.substr(1, line.find('"', 1) - 1) : line.substr(0, end);
        movie_names.push_back(movie_name);
        start = end + 1;

        // Handle genre ratings
        int index = 0;
        while ((end = line.find(',', start)) != string::npos) {
            if (line[start] == '1') {
                rating.insert(index) = 1; // Only store '1's
            }
            index++;
            start = end + 1;
        }
        if (line[start] == '1') {
            rating.insert(index) = 1;
        }

        genre_ratings.push_back(rating);
        row++;
    }
    file.close();
}


void load_user_data(int n) {
    ifstream file("C:/Users/joshc/Documents/COE163/user_features.csv");
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
    double rating;

    MovieData(const string& name, double rating) : name(name), rating(rating) {}

    bool operator<(const MovieData& other) const {
        if (fabs(rating - other.rating) > 1.0e-5) {
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
    movies.reserve(movie_names.size());
    for (size_t i = 0; i < movie_names.size(); ++i) 
        movies.emplace_back(movie_names[i],genre_ratings[i].dot(user_ratings));

    sort(movies.begin(), movies.end());

    for (int i = 0; i < 10; ++i) {
        cout << movies[i].name << '\n';
    }

    end = chrono::high_resolution_clock::now();
    chrono::duration<double> elapsed_seconds = end - start;
    cout << "Elapsed time: " << elapsed_seconds.count() << "s\n";

    return 0;
}