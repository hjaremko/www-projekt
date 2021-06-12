use crate::model::{NewPostRequest, Post, Rights, User};
use crate::repository::{PostsRepository, UserRepository};
use chrono::{DateTime, Utc};

pub struct UserService {}

impl UserService {
    pub fn get_all() -> Vec<User> {
        UserRepository::get_all().unwrap()
    }

    pub fn get(id: i32) -> User {
        User {
            id,
            name: "User".to_string(),
            rights: Rights::Common,
        }
    }
}

pub struct PostsService {}

impl PostsService {
    pub fn get_all() -> Vec<Post> {
        PostsRepository::get_all().unwrap()
    }

    pub fn get_page(page_number: usize, posts_per_page: usize) -> Vec<Post> {
        let all_posts = PostsRepository::get_all().unwrap();
        let mut pages = all_posts.chunks(posts_per_page);

        let page = pages.nth(page_number);

        if let None = page {
            return vec![];
        }

        page.unwrap().into()
    }

    pub fn add_post(request: NewPostRequest) -> Post {
        let author = UserService::get(request.author_id);

        let now: DateTime<Utc> = Utc::now();
        let now = now.format("%F %R");

        println!("UTC now is: {}", now);

        let post = Post {
            id: PostsService::get_all().len() as i32,
            date: now.to_string(),
            title: request.title,
            author,
            content: request.content,
        };

        PostsRepository::add_post(&post).unwrap();
        post
    }
}
