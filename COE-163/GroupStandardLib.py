import csv
from collections import defaultdict

def load_data(filename):
    with open(filename, newline='') as csvfile:
        reader = csv.DictReader(csvfile)
        data = [row for row in reader]
    return data

def dot_product(user_features, movie_features):
    result = 0
    for feature in user_features:
        if feature in movie_features:
            result += float(user_features[feature]) * float(movie_features[feature])
    return round(result, 2)

def calculate_user_ratings(user_preferences, movies):
    ratings = {}
    for movie in movies:
        movie_name = movie['Movie']
        movie_features = {k: v for k, v in movie.items() if k != 'Movie'}
        ratings[movie_name] = dot_product(user_preferences, movie_features)
    return ratings

def get_all_user_ratings(users, movies):
    user_ratings = {}
    for user in users:
        user_id = user['User ID']
        user_features = {k: v for k, v in user.items() if k not in ['User ID', 'User Name']}
        user_ratings[user_id] = calculate_user_ratings(user_features, movies)
    return user_ratings

def sort_ratings(ratings):
    return sorted(ratings.items(), key=lambda x: (-x[1], x[0]))

users = load_data('user_features.csv')
movies = load_data('movie_features.csv')

all_user_ratings = get_all_user_ratings(users, movies)

user_input = input("Enter user ID: ")
movie_ratings = all_user_ratings[user_input]
sorted_movies = sort_ratings(movie_ratings)
top_movies = sorted_movies[:10]
for movie, ratings in top_movies:
    print(movie)
