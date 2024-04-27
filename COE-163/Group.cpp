#include <bits/stdc++.h>
using namespace std;

struct MovieFeature {
    string movie;
    map<string, double> features;
};

vector<map<string, string>> load_data(const string& filename) {
    vector<map<string, string>> data;
    ifstream file(filename);
    string line, colname;
    vector<string> colnames;

    if (file.is_open()) {
        if (getline(file, line)) {
            istringstream ss(line);
            while (getline(ss, colname, ',')) {
                colnames.push_back(colname);
            }
        }
        while (getline(file, line)) {
            istringstream ss(line);
            string field;
            map<string, string> row;
            size_t i = 0;
            while (getline(ss, field, ',')) {
                row[colnames[i]] = field;
                i++;
            }
            data.push_back(row);
        }
    }
    file.close();
    return data;
}

double safe_stod(const string& str) {
    try {
        return stod(str);
    } catch (const invalid_argument& ia) {
        cerr << "Invalid argument: " << str << " - " << ia.what() << endl;
        return 0.0;
    } catch (const out_of_range& oor) {
        cerr << "Out of range error: " << str << " - " << oor.what() << endl;
        return 0.0;
    }
}

map<string, double> calculate_ratings(const map<string, string>& user_features, const vector<MovieFeature>& movies) {
    map<string, double> ratings;
    for (const auto& movie : movies) {
        double rating = 0.0;
        for (const auto& uf : user_features) {
            if (uf.first != "User" && uf.first != "User ID" && movie.features.find(uf.first) != movie.features.end()) {
                double user_feature_value = safe_stod(uf.second);
                rating += user_feature_value * movie.features.at(uf.first);
            }
        }
        ratings[movie.movie] = round(rating * 100.0) / 100.0;
    }
    return ratings;
}

int main() {
    auto users = load_data("user_features.csv");
    auto movies_data = load_data("movie_features.csv");

    vector<MovieFeature> movies;
    for (const auto& md : movies_data) {
        MovieFeature mf;
        mf.movie = md.at("Movie");
        for (const auto& m : md) {
            if (m.first != "Movie" && m.first != "User" && m.first != "User ID") {
                try {
                    mf.features[m.first] = safe_stod(m.second);
                } catch (...) {
                    // Ignore errors in conversion and do not add to features
                }
            }
        }
        movies.push_back(mf);
    }

    map<string, map<string, double>> user_ratings;
    for (const auto& user : users) {
        user_ratings[user.at("User ID")] = calculate_ratings(user, movies);
    }

    string userId;
    cout << "Enter User ID: ";
    cin >> userId;

    // Sorting the movies
    vector<pair<string, double>> sorted_movies(user_ratings[userId].begin(), user_ratings[userId].end());
    sort(sorted_movies.begin(), sorted_movies.end(), [](const auto& a, const auto& b) {
        return a.second > b.second || (a.second == b.second && a.first < b.first);
    });

    // Printing top 10 movies
    for (size_t i = 0; i < 10 && i < sorted_movies.size(); ++i) {
        cout << sorted_movies[i].first << endl;
    }

    return 0;
}