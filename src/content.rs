use super::*;

#[derive(Clone, PartialEq)]
pub(crate) enum Content {
    Skill(Skill),
    Achievement(Achievement),
    Creation(Creation),
    Article(Article),
    NotFound,
}

#[derive(Properties, PartialEq)]
pub(crate) struct ContentProperties {
    pub(crate) content: Content,
}

#[function_component(ContentComponent)]
pub(crate) fn content_component(ContentProperties { content }: &ContentProperties) -> Html {
    match content {
        Content::Skill(skill) => skill.get_html(),
        Content::Achievement(achievement) => achievement.get_html(),
        Content::Creation(creation) => creation.get_html(),
        Content::Article(article) => article.get_html(),
        Content::NotFound => html! { <NotFound /> },
    }
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
            <div key={self.id} class={"card"}>
                <h3 class={"card-title"}>{self.name.clone()}</h3>
                <p class={"card-detail"}>{
                    self.areas.iter().map(|area| html!{
                        <span>{area.clone().to_owned() + ". "}</span>
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

    pub(crate) fn get_html(&self) -> Html {
        html! {
            <div class={"content-list"}>
                {
                    self.skills.iter().map(
                        |skill| skill.get_html()
                    ).collect::<Html>()
                }
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct SkillListProperties {
    pub(crate) skills: SkillList,
}

#[derive(Clone, PartialEq)]
pub(crate) struct Achievement {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) completed: KiraDate,
    pub(crate) areas: Vec<String>,
    pub(crate) tools: Vec<String>,
    pub(crate) skills: Vec<Skill>,
    pub(crate) description: String,
}

impl Achievement {
    pub(crate) fn get_html(&self) -> Html {
        html! {
            <div key={self.id} class={"card"}>
                <h3 class={"card-title"}>{self.name.clone()}</h3>
                <p class={"card-detail"}>{
                    self.areas.iter().map(|area| html!{
                        <span>{area.clone().to_owned() + ". "}</span>
                    }).collect::<Html>()
                }</p>
                <p class={"card-detail"}>{self.completed.to_string("")}</p>
                <p class={"card-text"}>{self.description.clone()}</p>
                <p class={"card-detail"}>{
                    self.tools.iter().map(|tool| html!{
                        <span>{tool.clone().to_owned() + ". "}</span>
                    }).collect::<Html>()
                }</p>
                <p class={"card-detail"}>{
                    self.skills.iter().map(|skill| html!{
                        <span>{skill.name.clone().to_owned() + ". "}</span>
                    }).collect::<Html>()
                }</p>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct AchievementProperties {
    pub(crate) achievement: Achievement,
}

#[derive(Clone, PartialEq)]
pub(crate) struct AchievementList {
    achievements: Vec<Achievement>,
}

impl AchievementList {
    pub(crate) fn new(achievements: Vec<Achievement>) -> AchievementList {
        AchievementList { achievements }
    }

    pub(crate) fn find_first_with_name(&self, name: &str) -> Option<&Achievement> {
        self.achievements
            .iter()
            .find(|achievement| achievement.name == name)
    }

    pub(crate) fn get_html(&self) -> Html {
        html! {
            <div class={"content-list"}>
                {
                    self.achievements.iter().map(
                        |achievement| achievement.get_html()
                    ).collect::<Html>()
                }
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct AchievementListProperties {
    pub(crate) achievements: AchievementList,
}

#[derive(Clone, PartialEq)]
pub(crate) struct Creation {
    pub(crate) id: usize,
    pub(crate) name: String,
    pub(crate) completed: KiraDate,
    pub(crate) areas: Vec<String>,
    pub(crate) tools: Vec<String>,
    pub(crate) skills: Vec<Skill>,
    pub(crate) description: String,
}

impl Creation {
    pub(crate) fn get_html(&self) -> Html {
        html! {
            <div key={self.id} class={"card"}>
                <h3 class={"card-title"}>{self.name.clone()}</h3>
                <p class={"card-detail"}>{
                    self.areas.iter().map(|area| html!{
                        <span>{area.clone().to_owned() + ". "}</span>
                    }).collect::<Html>()
                }</p>
                <p class={"card-detail"}>{self.completed.to_string("")}</p>
                <p class={"card-text"}>{self.description.clone()}</p>
                <p class={"card-detail"}>{
                    self.tools.iter().map(|tool| html!{
                        <span>{tool.clone().to_owned() + "."}</span>
                    }).collect::<Html>()
                }</p>
                <p class={"card-detail"}>{
                    self.skills.iter().map(|skill| html!{
                        <span>{skill.name.clone().to_owned() + ". "}</span>
                    }).collect::<Html>()
                }</p>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct CreationProperties {
    pub(crate) creation: Creation,
}

#[derive(Clone, PartialEq)]
pub(crate) struct CreationList {
    creations: Vec<Creation>,
}

impl CreationList {
    pub(crate) fn new(creations: Vec<Creation>) -> CreationList {
        CreationList { creations }
    }

    pub(crate) fn find_first_with_name(&self, name: &str) -> Option<&Creation> {
        self.creations.iter().find(|creation| creation.name == name)
    }

    pub(crate) fn get_html(&self) -> Html {
        html! {
            <div class={"content-list"}>
                {
                    self.creations.iter().map(
                        |creation| creation.get_html()
                    ).collect::<Html>()
                }
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct CreationListProperties {
    pub(crate) creations: CreationList,
}

#[derive(Clone, PartialEq)]
pub(crate) struct Article {
    pub(crate) id: usize,
    pub(crate) title: String,
    pub(crate) published: KiraDate,
    pub(crate) topics: Vec<String>,
    pub(crate) summary: String,
    pub(crate) content: Html,
}

impl Article {
    pub(crate) fn get_html(&self) -> Html {
        html! {
            <article key={self.id} class={"card"}>
                <h3 class={"card-title"}>{self.title.clone()}</h3>
                <p class={"card-detail"}>{self.published.to_string("")}</p>
                <p class={"card-detail"}>{
                    self.topics.iter().map(|topic| html!{
                        <span>{topic.clone().to_owned() + ". "}</span>
                    }).collect::<Html>()
                }</p>
                <p class={"card-text"}>{self.summary.clone()}</p>
                <div class={"card-content"}>{self.content.clone()}</div>
            </article>
        }
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct ArticleProperties {
    pub(crate) article: Article,
}

#[derive(Clone, PartialEq)]
pub(crate) struct ArticleList {
    articles: Vec<Article>,
}

impl ArticleList {
    pub(crate) fn new(articles: Vec<Article>) -> ArticleList {
        ArticleList { articles }
    }

    pub(crate) fn find_first_with_title(&self, title: &str) -> Option<&Article> {
        self.articles.iter().find(|article| article.title == title)
    }

    pub(crate) fn get_html(&self) -> Html {
        html! {
            <div class={"content-list"}>
                {
                    self.articles.iter().map(
                        |article| article.get_html()
                    ).collect::<Html>()
                }
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct ArticleListProperties {
    pub(crate) articles: ArticleList,
}
