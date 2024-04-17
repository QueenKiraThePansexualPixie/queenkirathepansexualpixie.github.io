use chrono::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod typing;
use typing::*;

mod content;
use content::*;

mod pages;
use pages::*;

#[function_component(App)]
fn app() -> Html {
    let skills: SkillList = SkillList::new(vec![
        Skill {
            id: 0,
            name: "Rust".to_string(),
            areas: vec![
                "Backend".to_string(),
                "Frontend".to_string(),
                "Game Development".to_string(),
                "Scripting".to_string(),
                "Systems Programming".to_string(),
                "Web Development".to_string(),
            ],
            competency: "Beginner".to_string(),
            description: "High-level systems programming language, designed for interacting more safely with low-level concepts.".to_string(),
        },
        Skill {
            id: 1,
            name: "C++".to_string(),
            areas: vec![
                "Backend".to_string(),
                "Database Development".to_string(),
                "Digital Graphics".to_string(),
                "Frontend".to_string(),
                "Game Development".to_string(),
                "Scripting".to_string(),
                "Systems Programming".to_string(),
                "Web Development".to_string(),
            ],
            competency: "Novice".to_string(),
            description: "Low-level, high-control, systems programming language. Higher level than roughly 2 of the hundreds of programming language that exist in today's landscape.".to_string(),
        },
    ]);

    let achievements: AchievementList = AchievementList::new(vec![Achievement {
        id: 0,
        name: "<NAME>".to_string(),
        completed: KiraDate::new(2023, 5, 24),
        areas: vec!["Programming".to_string(), "Web Development".to_string()],
        tools: vec!["Visual Studio Code".to_string(), "VS Code".to_string()],
        skills: vec![],
        description: "".to_string(),
    }]);

    let creations: CreationList = CreationList::new(vec![Creation {
        id: 0,
        name: "<NAME>".to_string(),
        completed: KiraDate::new(2023, 9, 16),
        areas: vec![],
        tools: vec![],
        skills: vec![],
        description: "".to_string(),
    }]);

    let articles: ArticleList = ArticleList::new(vec![Article {
        id: 0,
        title: "<NAME>".to_string(),
        published: KiraDate::new(2023, 8, 17),
        topics: vec![],
        summary: "".to_string(),
        content: html! {
            <div>
                <h1>{"<NAME>"}</h1>
            </div>
        },
    }]);

    html! {
        <div>
            <header  id={"header"}>
                <h1>{ "Hello, world!" }</h1>
                <img src={"icon.png"} alt={"Website Icon"} />
            </header>
            <ul id={"nav"}>
                <li>
                    <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Skills}>{ "Skills" }</Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Achievements}>{ "Achievements" }</Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Creations}>{ "Creations" }</Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Articles}>{ "Articles" }</Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>>
                </li>
            </ul>
            <main id={"main"}>
                <BrowserRouter>
                    <Switch<Route>
                        render={move |route: Route| {
                            match route {
                                Route::Home => html! { <Home /> },
                                Route::Skills => html! { <Skills skills={skills.clone()} /> },
                                Route::Skill { skill } => html! {
                                    <ContentComponent content={
                                        match skills.find_first_with_name(&skill) {
                                            Some(skill) => Content::Skill(skill.to_owned()),
                                            None => Content::NotFound,
                                        }
                                    } />
                                },
                                Route::Achievements => html! { <Achievements achievements={achievements.clone()} /> },
                                Route::Achievement { achievement } => html! {
                                    <ContentComponent content={
                                        match achievements.find_first_with_name(&achievement) {
                                            Some(achievement) => Content::Achievement(achievement.to_owned()),
                                            None => Content::NotFound,
                                        }
                                    } />
                                },
                                Route::Creations => html! { <Creations creations={creations.clone()} /> },
                                Route::Creation { creation } => html! {
                                    <ContentComponent content={
                                        match creations.find_first_with_name(&creation) {
                                            Some(creation) => Content::Creation(creation.to_owned()),
                                            None => Content::NotFound,
                                        }
                                    } />
                                },
                                Route::Articles => html! { <Articles articles={articles.clone()} /> },
                                Route::Article { article } => html! {
                                    <ContentComponent content={
                                        match articles.find_first_with_title(&article) {
                                            Some(article) => Content::Article(article.to_owned()),
                                            None => Content::NotFound,
                                        }
                                    } />
                                },
                                Route::Contact => html! { <Contact /> },
                                Route::NotFound => html! { <NotFound /> },
                            }
                        }}
                    />
                </BrowserRouter>
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
