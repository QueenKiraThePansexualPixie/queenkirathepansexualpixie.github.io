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
    #[at("/creations")]
    Creations,
    #[at("/creations/:creation")]
    Creation { creation: String },
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

            <hr />

            <p>
                {"I was born in the small town of Wincanton in rural England. "}
                {"It was a Wednesday, Wednesday the 14th of September, in 2005. "}
            </p>
        </div>
    }
}

#[function_component(Skills)]
pub(crate) fn skills(SkillListProperties { skills }: &SkillListProperties) -> Html {
    html! {
        <div>
            <h1>{"Skills"}</h1>

            <p>{"These are my skills."}</p>

            <hr />

            <div>{skills.get_html()}</div>
        </div>
    }
}

#[function_component(Achievements)]
pub(crate) fn achievements(
    AchievementListProperties { achievements }: &AchievementListProperties,
) -> Html {
    html! {
        <div>
            <h1>{"Achievements"}</h1>

            <p>{"These are my achievements."}</p>

            <hr />

            <div>{achievements.get_html()}</div>
        </div>
    }
}

#[function_component(Creations)]
pub(crate) fn creations(CreationListProperties { creations }: &CreationListProperties) -> Html {
    html! {
        <div>
            <h1>{"Creations"}</h1>

            <p>{"These are my creations."}</p>

            <hr />

            <div>{creations.get_html()}</div>
        </div>
    }
}

#[function_component(Articles)]
pub(crate) fn articles(ArticleListProperties { articles }: &ArticleListProperties) -> Html {
    html! {
        <div>
            <h1>{"Articles"}</h1>

            <p>{"These are my articles."}</p>

            <hr />

            <div>{articles.get_html()}</div>
        </div>
    }
}

#[function_component(Contact)]
pub(crate) fn contact() -> Html {
    html! {
        <div>
            <h1>{"Contact"}</h1>

            <p>{
                "If you have any questions, feel free to contact me."
            }</p>

            <div class={"contact-details-container"}>
                <a
                    class={"contact-icon"}
                    href={"mailto:kira.hudson.v0@gmail.com"}
                    title={"kira.hudson.v0@gmail.com"}
                    id={"Email"}
                    target={"_blank"}
                >
                    <i class={"fa-solid fa-square-envelope"}></i>
                </a>

                <a
                    class={"contact-icon"}
                    href={"https://github.com/QueenKiraThePansexualPixie/"}
                    id={"GitHub"}
                    title={"@QueenKiraThePansexualPixie"}
                    target={"_blank"}
                >
                    <i class={"fa-brands fa-square-github"}></i>
                </a>

                <a
                    class={"contact-icon"}
                    href={"https://www.tumblr.com/blog/kira-is-pan/"}
                    id={"Tumblr"}
                    title={"@kira-is-pan"}
                    target={"_blank"}
                >
                    <i class={"fa-brands fa-square-tumblr"}></i>
                </a>

                <a
                    class={"contact-icon"}
                    href={"https://www.instagram.com/kirathepanpixie/"}
                    id={"Instagram"}
                    title={"@kirathepanpixie"}
                    target={"_blank"}
                >
                    <i class={"fa-brands fa-square-instagram"}></i>
                </a>

                <a
                    class={"contact-icon"}
                    href={"https://www.pinterest.co.uk/kirathepansexualpixie/"}
                    id={"Pinterest"}
                    title={"@kirathepansexualpixie"}
                    target={"_blank"}
                >
                    <i class={"fa-brands fa-square-pinterest"}></i>
                </a>

                <a
                    class={"contact-icon"}
                    href={"https://www.reddit.com/user/KiraThePanPixie/"}
                    id={"Reddit"}
                    title={"@KiraThePanPixie"}
                    target={"_blank"}
                >
                    <i class={"fa-brands fa-square-reddit"}></i>
                </a>
            </div>
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
