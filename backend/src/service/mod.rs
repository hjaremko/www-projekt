use crate::model::{NewPostRequest, Post, Rights, User, Comment, RegisterRequest};
use crate::repository::{PostsRepository, UserRepository, CommentsRepository};
use chrono::{DateTime, Utc};

pub struct UserService {}

impl UserService {}

impl UserService {
    pub fn get_all() -> Vec<User> {
        UserRepository::get_all().unwrap()
    }

    pub fn get(id: i32) -> User {
        User {
            id,
            name: "User".to_string(),
            rights: Rights::Common,
            password: "".to_string(),
            login: "".to_string(),
        }
    }

    pub fn get_by_login(login: &str) -> User {
        let users = UserRepository::get_all().unwrap();
        let user = users.into_iter().find(|x| x.login == login).unwrap();

        user
    }

    pub fn create(request: RegisterRequest) -> User {
        UserRepository::create(request).unwrap();
        User {
            id: 0,
            login: "".to_string(),
            password: "".to_string(),
            name: "".to_string(),
            rights: Rights::Administrator,
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

pub struct CommentsService {}

impl CommentsService {
    pub fn get_all(post_id: usize) -> Vec<Comment> {
        CommentsRepository::get_all_from_post(post_id).unwrap()
    }
}
