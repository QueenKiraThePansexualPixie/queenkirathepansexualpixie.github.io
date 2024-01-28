use chrono::{DateTime, Utc};
use yew::prelude::*;
use yew_router::prelude::*;

mod content {
    use super::*;

    #[derive(Clone, PartialEq)]
    pub(crate) struct Skill {
        id: usize,
        name: String,
        areas: Vec<String>,
        competency: String,
        description: String,
    }

    impl Skill {
        pub(crate) fn get_html(&self) -> Html {
            html! {
                <div class={"card"}>
                    <h5 class={"card-title"}>{self.name}</h5>
                    <p class={"card-detail"}>{self.areas.iter().map(|area| html!{ <span>{area.clone().to_owned() + "."}</span> }).collect::<Html>()}</p>
                    <p class={"card-detail"}>{self.competency}</p>
                    <p class={"card-text"}>{self.description}</p>
                </div>
            }
        }
    }

    pub(crate) struct Achievement {
        id: usize,
        name: String,
        completed: DateTime<Utc>,
        areas: Vec<String>,
        tools: Vec<String>,
        skills: Vec<Skill>,
        description: String,
    }

    impl Achievement {
        pub(crate) fn get_html(&self) -> Html {
            html! {
                <div class={"card"}>
                    <h5 class={"card-title"}>{self.name}</h5>
                    <p class={"card-detail"}>{self.areas.iter().map(|area| html!{ <span>{area.clone().to_owned() + "."}</span> }).collect::<Html>()}</p>
                    <p class={"card-detail"}>{self.completed.to_string()}</p>
                    <p class={"card-text"}>{self.description}</p>
                    <p class={"card-detail"}>{self.tools.iter().map(|tool| html!{ <span>{tool.clone().to_owned() + "."}</span> }).collect::<Html>()}</p>
                    <p class={"card-detail"}>{self.skills.iter().map(|skill| html!{ <span>{skill.get_html()}</span> }).collect::<Html>()}</p>
                </div>
            }
        }
    }

    pub(crate) struct Article {
        id: usize,
        title: String,
        published: DateTime<Utc>,
        topics: Vec<String>,
        summary: String,
        content: Html,
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

                <p>{"Hi, I'm Kira H, and I somehow exist, unfortunately for you."}</p>
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
                                Route::Skill { skill } => html! { <SkillProperties skill={skill} /> },
                                Route::Achievements => html! { <Achievements /> },
                                Route::Achievement { achievement } => html! { <AchievementProperties achievement={achievement} /> },
                                Route::Articles => html! { <Articles /> },
                                Route::Article { article } => html! { <ArticleProperties article={article} /> },
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
