use chrono::{DateTime, Utc};
use yew::prelude::*;
use yew_router::prelude::*;

mod content {
    use super::*;

    #[derive(Clone, PartialEq)]
    pub(crate) enum Content {
        Skill(Skill),
        Achievements(Achievement),
        Article(Article),
        NotFound,
    }

    #[derive(Properties, PartialEq)]
    pub(crate) struct ContentProperties {
        pub(crate) content: Content,
    }

    #[derive(Clone, PartialEq)]
    pub(crate) struct Skill {
        pub(crate) id: usize,
        pub(crate) name: String,
        pub(crate) areas: Vec<String>,
        pub(crate) competency: String,
        pub(crate) description: String,
    }

    impl Skill {
        pub(crate) fn get_html(&self) -> Html {
            html! {
                <div class={"card"}>
                    <h5 class={"card-title"}>{self.name.clone()}</h5>
                    <p class={"card-detail"}>{
                        self.areas.iter().map(|area| html!{
                            <span>{area.clone().to_owned() + "."}</span>
                        }).collect::<Html>()
                    }</p>
                    <p class={"card-detail"}>{self.competency.clone()}</p>
                    <p class={"card-text"}>{self.description.clone()}</p>
                </div>
            }
        }
    }

    #[derive(Properties, PartialEq)]
    pub(crate) struct SkillProperties {
        pub(crate) skill: Skill,
    }

    #[derive(Clone, PartialEq)]
    pub(crate) struct SkillList {
        skills: Vec<Skill>,
    }

    impl SkillList {
        pub(crate) fn new(skills: Vec<Skill>) -> SkillList {
            SkillList { skills }
        }

        pub(crate) fn find_first_with_name(&self, name: &str) -> Option<&Skill> {
            self.skills.iter().find(|skill| skill.name == name)
        }
    }

    #[derive(Properties, PartialEq)]
    pub(crate) struct SkillListProperties {
        pub(crate) skills: Vec<Skill>,
    }

    #[function_component(ContentComponent)]
    pub(crate) fn content_component(ContentProperties { content }: &ContentProperties) -> Html {
        match content {
            Content::Skill(skill) => skill.get_html(),
            Content::Achievements(achievement) => achievement.get_html(),
            Content::Article(article) => article.get_html(),
            Content::NotFound => html! { <NotFound /> },
        }
    }

    #[derive(Clone, PartialEq)]
    pub(crate) struct Achievement {
        pub(crate) id: usize,
        pub(crate) name: String,
        pub(crate) completed: DateTime<Utc>,
        pub(crate) areas: Vec<String>,
        pub(crate) tools: Vec<String>,
        pub(crate) skills: Vec<Skill>,
        pub(crate) description: String,
    }

    impl Achievement {
        pub(crate) fn get_html(&self) -> Html {
            html! {
                <div class={"card"}>
                    <h5 class={"card-title"}>{self.name.clone()}</h5>
                    <p class={"card-detail"}>{
                        self.areas.iter().map(|area| html!{
                            <span>{area.clone().to_owned() + "."}</span>
                        }).collect::<Html>()
                    }</p>
                    <p class={"card-detail"}>{self.completed.to_string()}</p>
                    <p class={"card-text"}>{self.description.clone()}</p>
                    <p class={"card-detail"}>{
                        self.tools.iter().map(|tool| html!{
                            <span>{tool.clone().to_owned() + "."}</span>
                        }).collect::<Html>()
                    }</p>
                    <p class={"card-detail"}>{
                        self.skills.iter().map(|skill| html!{
                            <span>{skill.get_html()}</span>
                        }).collect::<Html>()
                    }</p>
                </div>
            }
        }
    }

    #[derive(Clone, PartialEq)]
    pub(crate) struct Article {
        pub(crate) id: usize,
        pub(crate) title: String,
        pub(crate) published: DateTime<Utc>,
        pub(crate) topics: Vec<String>,
        pub(crate) summary: String,
        pub(crate) content: Html,
    }

    impl Article {
        pub(crate) fn get_html(&self) -> Html {
            html! {
                <article class={"card"}>
                    <h5 class={"card-title"}>{self.title.clone()}</h5>
                    <p class={"card-detail"}>{self.published.to_string()}</p>
                    <p class={"card-detail"}>{
                        self.topics.iter().map(|topic| html!{
                            <span>{topic.clone().to_owned() + "."}</span>
                        }).collect::<Html>()
                    }</p>
                    <p class={"card-text"}>{self.summary.clone()}</p>
                    <div class={"card-content"}>{self.content.clone()}</div>
                </article>
            }
        }
    }
}
use content::*;

mod pages {
    use super::*;

    #[derive(Clone, Routable, PartialEq)]
    pub(crate) enum Route {
        #[at("/")]
        Home,
        #[at("/skills")]
        Skills,
        #[at("/skills/:skill")]
        Skill { skill: String },
        #[at("/achievements")]
        Achievements,
        #[at("/achievements/:achievement")]
        Achievement { achievement: String },
        #[at("/articles")]
        Articles,
        #[at("/articles/:article")]
        Article { article: String },
        #[at("/contact")]
        Contact,
        #[not_found]
        #[at("/404")]
        NotFound,
    }

    #[function_component(Home)]
    pub(crate) fn home() -> Html {
        html! {
            <div>
                <h1>{"Home"}</h1>

                <p>{
                    "Hi, I'm Kira H, and I somehow exist, unfortunately for you."
                }</p>
            </div>
        }
    }

    #[function_component(Skills)]
    pub(crate) fn skills() -> Html {
        html! {
            <div>
                <h1>{"Skills"}</h1>
            </div>
        }
    }

    #[function_component(Achievements)]
    pub(crate) fn achievements() -> Html {
        html! {
            <div>
                <h1>{"Achievements"}</h1>
            </div>
        }
    }

    #[function_component(Articles)]
    pub(crate) fn articles() -> Html {
        html! {
            <div>
                <h1>{"Articles"}</h1>
            </div>
        }
    }

    #[function_component(Contact)]
    pub(crate) fn contact() -> Html {
        html! {
            <div>
                <h1>{"Contact"}</h1>
            </div>
        }
    }

    #[function_component(NotFound)]
    pub(crate) fn not_found() -> Html {
        html! {
            <div>
                <h1>{"Error 404"}</h1>

                <p>{"Error 404 : Page Not Found"}</p>

                <p>{"Please navigate your way back to the main site"}</p>

                <p>
                    <b>{"EMERGENCY EXIT:"}</b>{" "}<a href={"/"}>{"Home"}</a>
                </p>
            </div>
        }
    }
}
use pages::*;

#[function_component(App)]
fn app() -> Html {
    let skills: SkillList = SkillList::new(vec![
        Skill {
            id: 0,
            name: "Rust".to_string(),
            areas: vec!["Programming".to_string(), "Web Development".to_string()],
            competency: "Beginner".to_string(),
            description: "".to_string(),
        },
        Skill {
            id: 1,
            name: "Python".to_string(),
            areas: vec!["Programming".to_string(), "Web Development".to_string()],
            competency: "Intermediate".to_string(),
            description: "".to_string(),
        },
    ]);
    let achievements: AchievementList = AchievementList::new(vec![
        Achievement {
            id: 0,
            name: "<NAME>".to_string(),
            completed: Utc::now(),
            areas: vec!["Programming".to_string(), "Web Development".to_string()],
            tools: vec!["Visual Studio Code".to_string(), "VS Code".to_string()],
            skills: vec![],
            description: "".to_string(),
        },
    ]);

    html! {
        <div>
            <header  id={"header"}>
                <h1>{ "Hello, world!" }</h1>
            </header>
            <ul id={"nav"}>
                <li><a href={"/"}>{ "Home" }</a></li>
                <li><a href={"/skills"}>{ "Skills" }</a></li>
                <li><a href={"/achievements"}>{ "Achievements" }</a></li>
                <li><a href={"/articles"}>{ "Articles" }</a></li>
                <li><a href={"/contact"}>{ "Contact" }</a></li>
            </ul>
            <main id={"main"}>
                <BrowserRouter>
                    <Switch<Route>
                        render={|route: Route| {
                            match route {
                                Route::Home => html! { <Home /> },
                                Route::Skills => html! { <Skills /> },
                                Route::Skill { skill } => html! {
                                    <ContentComponent content={
                                        match skills.find_first_with_name(&skill) {
                                            Some(skill) => Content::Skill(skill.to_owned()),
                                            None => Content::NotFound,
                                        }
                                    } />
                                },
                                Route::Achievements => html! { <Achievements /> },
                                Route::Achievement { achievement } => html! {
                                    <ContentComponent content={
                                        match achievements.find_first_with_name(&achievement) {
                                            Some(achievement) => Content::Skill(achievement.to_owned()),
                                            None => Content::NotFound,
                                        }
                                    } />
                                },
                                Route::Articles => html! { <Articles /> },
                                Route::Article { article } => html! {
                                    <ContentComponent content={
                                        match articles.find_first_with_name(&article) {
                                            Some(article) => Content::Skill(article.to_owned()),
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
