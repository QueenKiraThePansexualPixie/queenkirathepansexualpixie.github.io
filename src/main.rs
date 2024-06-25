use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

mod content;
mod pages;
mod typing;

#[allow(clippy::wildcard_imports)]
use content::*;
#[allow(clippy::wildcard_imports)]
use pages::*;
#[allow(clippy::wildcard_imports)]
use typing::*;

#[function_component(App)]
fn app() -> Html {
    let areas: AreaList = AreaList::from(vec![
        Area("development"),
        Area("development/backend"),
        Area("development/database"),
        Area("development/frontend"),
        Area("development/game"),
        Area("development/systems"),
        Area("development/web"),
        Area("graphics/digital"),
        Area("language"),
        Area("scripting"),
    ]);
    let tools: ToolList = ToolList::from(vec![
        Tool("Blender"),
        Tool("Illustrator"),
        Tool("Photoshop"),
        Tool("Unity"),
        Tool("Visual Studio"),
        Tool("Visual Studio Code"),
    ]);

    let skills: SkillList = SkillList::new(vec![
        Skill {
            id: 0,
            name: "Rust".to_string(),
            areas: vec![
                Rc::new(areas.get_unwrap("development/backend")),
                Rc::new(areas.get_unwrap("development/frontend")),
                Rc::new(areas.get_unwrap("development/game")),
                Rc::new(areas.get_unwrap("development/systems")),
                Rc::new(areas.get_unwrap("development/web")),
                Rc::new(areas.get_unwrap("scripting")),
            ],
            competency: Competency::Intermediate,
            description: "A high-level systems programming language, designed for interacting more safely with low-level concepts.".to_string(),
        },
        Skill {
            id: 1,
            name: "Python".to_string(),
            areas: vec![
                Rc::new(areas.get_unwrap("development/backend")),
                Rc::new(areas.get_unwrap("development/frontend")),
                Rc::new(areas.get_unwrap("development/game")),
                Rc::new(areas.get_unwrap("development/web")),
                Rc::new(areas.get_unwrap("scripting")),
            ],
            competency: Competency::Intermediate,
            description: "A high-level, interpreted programming language with a focus on readbility and fast launch-times over fast and powerful programs.".to_string(),
        },
        Skill {
            id: 2,
            name: "C++".to_string(),
            areas: vec![
                Rc::new(areas.get_unwrap("development/backend")),
                Rc::new(areas.get_unwrap("development/database")),
                Rc::new(areas.get_unwrap("development/frontend")),
                Rc::new(areas.get_unwrap("development/game")),
                Rc::new(areas.get_unwrap("development/systems")),
                Rc::new(areas.get_unwrap("development/web")),
                Rc::new(areas.get_unwrap("graphics/digital")),
                Rc::new(areas.get_unwrap("scripting")),
            ],
            competency: Competency::Novice,
            description: "A low-level, high-control, systems programming language.".to_string(),
        },
        Skill {
            id: 3,
            name: "Web Development".to_string(),
            areas: vec![
                Rc::new(areas.get_unwrap("development/backend")),
                Rc::new(areas.get_unwrap("development/frontend")),
                Rc::new(areas.get_unwrap("development/web")),
            ],
            competency: Competency::Intermediate,
            description: "Web development.".to_string(),
        },
    ]);

    let achievements: AchievementList = AchievementList::new(vec![Achievement {
        id: 0,
        name: "<NAME>".to_string(),
        completed: KiraDate::new(2023, 5, 24),
        areas: vec![],
        tools: vec![],
        skills: vec![],
        description: "<DESCRIPTION>".to_string(),
    }]);

    let creations: CreationList = CreationList::new(vec![Creation {
        id: 0,
        name: "Web Profile".to_string(),
        completed: KiraDate::new(2023, 9, 16),
        areas: vec![Rc::new(
            areas.get_unwrap_or("development/web", Area::not_found()),
        )],
        tools: vec![Rc::new(
            tools.get_unwrap_or("Visual Studio Code", Tool::not_found()),
        )],
        skills: vec![Rc::new(
            skills.get_unwrap_or("Web Development", Skill::not_found()),
        )],
        description: "A personal profile website.".to_string(),
    }]);

    let articles: ArticleList = ArticleList::new(vec![Article {
        id: 0,
        title: "<NAME>".to_string(),
        published: KiraDate::new(2023, 8, 17),
        topics: vec![],
        summary: "<SUMMARY>".to_string(),
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
                                    <ContentComponent
                                        content={skills.get(&skill).map_or_else(|| Content::NotFound, Content::Skill)} />
                                },
                                Route::Achievements => html! { <Achievements achievements={achievements.clone()} /> },
                                Route::Achievement { achievement } => html! {
                                    <ContentComponent
                                        content={achievements.get(&achievement).map_or_else(|| Content::NotFound, Content::Achievement)} />
                                },
                                Route::Creations => html! { <Creations creations={creations.clone()} /> },
                                Route::Creation { creation } => html! {
                                    <ContentComponent
                                        content={creations.get(&creation).map_or_else(|| Content::NotFound, Content::Creation)} />
                                },
                                Route::Articles => html! { <Articles articles={articles.clone()} /> },
                                Route::Article { article } => html! {
                                    <ContentComponent
                                        content={articles.get(&article).map_or_else(|| Content::NotFound, Content::Article)} />
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
