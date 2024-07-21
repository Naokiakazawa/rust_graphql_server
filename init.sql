-- テーブルが存在する場合は削除
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS users;

-- usersテーブルの作成
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- postsテーブルの作成
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- commentsテーブルの作成
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    post_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (post_id) REFERENCES posts(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- サンプルデータの挿入
-- 5人のユーザーを追加
INSERT INTO users (username, email, password_hash) VALUES
('john_doe', 'john@example.com', 'hashed_password_1'),
('jane_smith', 'jane@example.com', 'hashed_password_2'),
('bob_johnson', 'bob@example.com', 'hashed_password_3'),
('alice_williams', 'alice@example.com', 'hashed_password_4'),
('charlie_brown', 'charlie@example.com', 'hashed_password_5');

-- 10個の投稿を追加
INSERT INTO posts (user_id, title, content) VALUES
(1, 'First Post', 'This is the content of the first post.'),
(2, 'Thoughts on Technology', 'Technology is rapidly changing our world...'),
(3, 'Recipe: Chocolate Cake', 'Here''s my favorite recipe for chocolate cake...'),
(4, 'Book Review: Sci-Fi Novel', 'I recently read an amazing science fiction novel...'),
(5, 'Gardening Tips', 'Here are some tips for maintaining a beautiful garden...'),
(1, 'Travel Experience: Japan', 'My recent trip to Japan was unforgettable...'),
(2, 'Fitness Journey', 'I''ve been on a fitness journey for the past 6 months...'),
(3, 'Movie Review: Action Thriller', 'Last night, I watched an exciting action thriller...'),
(4, 'DIY Home Decor', 'Today, I''ll share some DIY home decor ideas...'),
(5, 'Reflections on Remote Work', 'Working remotely has its challenges and benefits...');

-- コメントの追加
INSERT INTO comments (post_id, user_id, content) VALUES
(1, 2, 'Great post!'),
(1, 3, 'I agree with your points.'),
(2, 4, 'Interesting thoughts on technology.'),
(2, 5, 'I''d like to add that...'),
(3, 1, 'This recipe looks delicious!'),
(4, 2, 'I love sci-fi novels. Thanks for the review.'),
(5, 3, 'These gardening tips are very helpful.'),
(6, 4, 'Japan is on my travel bucket list!'),
(7, 5, 'Keep up the good work on your fitness journey!'),
(8, 1, 'I watched that movie too. It was amazing!'),
(9, 2, 'Love these DIY ideas. So creative!'),
(10, 3, 'Remote work has changed my life too.');