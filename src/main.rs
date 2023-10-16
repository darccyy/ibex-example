use ibex::prelude::*;

#[macro_use]
mod features;
mod blogs;
use blogs::BlogPost;

const URL_ROOT: &str = "/ibex-example";

fn main() {
    let blogs = blogs::get_blog_posts();

    let routes = routes![
        (/)
            => index_page(&blogs),
        (/post/[i])
            for (i, blog) in blogs.into_iter().enumerate()
            => blog_page(blog),
        (/test)
            => test_page(),
    ];

    let files = route::render_routes(routes);
    route::write_files(files).unwrap();
}

fn index_page(blogs: &[BlogPost]) -> Document {
    view! {
        @header[false]

        h1 { "Read blogs posts" }
        ul {
            [*for (i, blog) in (blogs.into_iter().enumerate()) {
                li {
                    a [href=[:?url!(format!("/post/{i}"))]] {
                        b { [&blog.title] }
                        ~ "-" ~
                        i { [&blog.author] }
                    }
                }
            }]
        }
    }
    .into()
}

fn blog_page(blog: BlogPost) -> Document {
    view! {
        @header[true]

        h2 { [blog.title] }
        h3 { i {[blog.author]} }

        p {
            [blog.body]
        }

        img [src=[:?blog.image]]/
    }
    .into()
}

fn header(home_link: bool) -> View {
    let text = "My Website";

    view! {
        HEAD {
            title { [text] }
        }
        h1 {
            [text]
            [*if (home_link) {
                small {
                    ~ "-" ~
                    a [href={url!("/")}] { "Back to home page" }
                }
            }]
        }
    }
}

fn test_page() -> Document {
    view! {
        @use_red
        "Some unicode characters: "
        "ŝĝŭĉĵĥ"
        @red { "Bruh" }
    }
    .into()
}

fn red(children: View) -> View {
    view! {
        div [class="color"] {
            [children]
        }
    }
}

fn use_red() -> View {
    view! {
        HEAD {
            style {
                ".color { background-color: red; }"
            }
        }
    }
}
