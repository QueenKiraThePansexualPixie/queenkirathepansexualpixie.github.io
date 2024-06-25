use crate::{KiraDate, NotFound};
use std::{fmt::Display, ops::Index, rc::Rc};
use yew::prelude::*;

// ********************************* Traits ********************************* //
// ************************************************************************** //

/// Trait for turning a type into an [`Html`] representation.
pub trait ToHtml {
    /// Returns an [`Html`] representation of the caller.
    fn to_html(&self) -> Html;
}

/// Trait for enforcing types have a "not found" value.
pub trait NotFoundValue {
    /// Returns the "not found" value.
    fn not_found() -> Self;
}

/// Trait for finding items in a list.
pub trait Get<K, V> {
    /// Finds the first item in the list matching the given identifier (`id`),
    /// or `None` if no match is found.
    ///
    /// Returns an `Option<&V>`
    fn get(&self, id: K) -> Option<V>;

    /// Finds the first item in the list matching the given identifier (`id`),
    /// or panics with the default unwrap message if no match is found.
    #[allow(clippy::unwrap_used)]
    fn get_unwrap(&self, id: K) -> V {
        self.get(id).unwrap()
    }

    /// Finds the first item in the list matching the given identifier (`id`),
    /// or returns the default value provided if no match is found.
    fn get_unwrap_or(&self, id: K, default: V) -> V {
        self.get(id).unwrap_or(default)
    }

    /// Finds the first item in the list mathing the given identifier (`id`),
    /// or panics with the specified message (`msg`) if no match is found.
    #[allow(dead_code)]
    fn get_expect(&self, id: K, msg: &str) -> V {
        self.get(id).expect(msg)
    }
}

// ********************************** Data ********************************** //
// ************************************************************************** //

// ************* Areas  ************* //
// ********************************** //

/// An area, like an "area of interest". Similar to a [`Topic`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Area(pub &'static str);

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NotFoundValue for Area {
    fn not_found() -> Self {
        Self("<ERR: Area not found>")
    }
}

/// A list of [`Area`]s with `get`ting functionality.
///
/// _potential TODO: implement hashing function for getters_
#[derive(Clone, PartialEq, Eq)]
pub struct AreaList {
    areas: Vec<Area>,
}

impl AreaList {
    /// Returns an iterator over the areas in the list.
    pub fn iter(&self) -> impl Iterator<Item = &Area> {
        self.areas.iter()
    }
}

impl Get<&str, Area> for AreaList {
    /// Finds the first area in the list with the given name (`string`), or
    /// `None` if no match is found.
    ///
    /// Returns an `Option<&Area>`
    fn get(&self, string: &str) -> Option<Area> {
        self.iter().find(|area| area.to_string() == string).copied()
    }

    /// Finds the first area in the list with the given name (`string`), or
    /// panics with the message below if no match is found.
    ///
    /// **Message:** ``Could not find Area `{string}` in AreaList``
    ///
    /// Where the "{string}" is the given `string` parameter.
    fn get_unwrap(&self, string: &str) -> Area {
        self.get(string)
            .unwrap_or_else(|| panic!("Could not find Area `{string}` in AreaList"))
    }
}

impl From<Vec<Area>> for AreaList {
    fn from(areas: Vec<Area>) -> Self {
        Self { areas }
    }
}

// ************* Tools  ************* //
// ********************************** //

/// A tool, like an application or other software, or even a physical tool,
/// used in the completion of a task.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tool(pub &'static str);

impl Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NotFoundValue for Tool {
    fn not_found() -> Self {
        Self("<ERR: Tool not found>")
    }
}

/// A list of [`Tool`]s with `get`ting functionality.
///
/// _potential TODO: implement hashing function for getters_
#[derive(Clone, PartialEq, Eq)]
pub struct ToolList {
    tools: Vec<Tool>,
}

impl ToolList {
    /// Returns an iterator over the tools in the list.
    pub fn iter(&self) -> impl Iterator<Item = &Tool> {
        self.tools.iter()
    }
}

impl Get<&str, Tool> for ToolList {
    /// Finds the first tool in the list with the given name (`string`), or
    /// `None` if no match is found.
    ///
    /// Returns an `Option<Tool>`
    fn get(&self, string: &str) -> Option<Tool> {
        self.iter().find(|tool| tool.to_string() == string).copied()
    }

    /// Finds the first tool in the list with the given name (`string`), or
    /// panics with the message below if no match is found.
    ///
    /// **Message:** ``Could not find Tool `{string}` in ToolList``
    ///
    /// Where the "{string}" is the given `string` parameter.
    fn get_unwrap(&self, string: &str) -> Tool {
        self.get(string)
            .unwrap_or_else(|| panic!("Could not find Tool `{string}` in ToolList"))
    }
}

impl From<Vec<Tool>> for ToolList {
    fn from(tools: Vec<Tool>) -> Self {
        Self { tools }
    }
}

// ************* Topics ************* //
// ********************************** //

/// A tool, like an application or other software, or even a physical tool,
/// used in the completion of a task.
#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq)]
pub enum Topic {
    Area(Rc<Area>),
    Tool(Rc<Tool>),
    Other(&'static str),
}

impl Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Area(area) => write!(f, "{area}"),
            Self::Tool(tool) => write!(f, "{tool}"),
            Self::Other(other) => write!(f, "{other}"),
        }
    }
}

// ********** Competencies ********** //
// ********************************** //

#[derive(Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum Competency {
    None = 0,
    Novice = 1,
    Intermediate = 2,
    Advanced = 3,
    Expert = 4,
}

impl Display for Competency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "None",
                Self::Novice => "Novice",
                Self::Intermediate => "Intermediate",
                Self::Advanced => "Advanced",
                Self::Expert => "Expert",
            }
        )
    }
}

impl From<Competency> for usize {
    fn from(competency: Competency) -> Self {
        competency as Self
    }
}

impl From<usize> for Competency {
    fn from(competency: usize) -> Self {
        match competency.clamp(0, 4) {
            0 => Self::None,
            1 => Self::Novice,
            2 => Self::Intermediate,
            3 => Self::Advanced,
            4 => Self::Expert,
            _ => unreachable!(),
        }
    }
}

// ******************************** Content  ******************************** //
// ************************************************************************** //

/// Type union for the different content types.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, PartialEq)]
pub enum Content {
    Skill(Skill),
    Achievement(Achievement),
    Creation(Creation),
    Article(Article),
    NotFound,
}

impl NotFoundValue for Content {
    fn not_found() -> Self {
        Self::NotFound
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Properties, PartialEq)]
pub struct ContentProperties {
    pub content: Content,
}

#[function_component(ContentComponent)]
pub fn content_component(ContentProperties { content }: &ContentProperties) -> Html {
    match content {
        Content::Skill(skill) => skill.to_html(),
        Content::Achievement(achievement) => achievement.to_html(),
        Content::Creation(creation) => creation.to_html(),
        Content::Article(article) => article.to_html(),
        Content::NotFound => html! { <NotFound /> },
    }
}

// ************* Skills ************* //
// ********************************** //

#[derive(Clone, PartialEq, Eq)]
pub struct Skill {
    pub id: usize,
    pub name: String,
    pub areas: Vec<Rc<Area>>,
    pub competency: Competency,
    pub description: String,
}

impl ToHtml for Skill {
    fn to_html(&self) -> Html {
        html! {
            <div key={self.id} class={"card"}>
                <h3 class={"card-title"}>{self.name.clone()}</h3>
                <p class={"card-detail"}>
                    {self.areas.iter().map(|area| html!{<span>{area.to_string() + ". "}</span>}).collect::<Html>()}
                </p>
                <p class={"card-detail"}>{format!("Competency: {}", self.competency)}</p>
                <p class={"card-text"}>{self.description.clone()}</p>
            </div>
        }
    }
}

impl NotFoundValue for Skill {
    fn not_found() -> Self {
        Self {
            id: usize::MAX,
            name: String::from("<ERR: Skill not found>"),
            areas: Vec::new(),
            competency: Competency::None,
            description: String::from("<An Error Occurred - This Skill was not found>"),
        }
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct SkillProperties {
    pub skill: Skill,
}

#[derive(Clone, PartialEq, Eq)]
pub struct SkillList {
    skills: Vec<Skill>,
}

impl SkillList {
    pub fn new(skills: Vec<Skill>) -> Self {
        Self { skills }
    }
}

impl Index<usize> for SkillList {
    type Output = Skill;

    fn index(&self, index: usize) -> &Self::Output {
        &self.skills[index]
    }
}

impl Get<&str, Skill> for SkillList {
    fn get(&self, name: &str) -> Option<Skill> {
        self.skills.iter().find(|skill| skill.name == name).cloned()
    }
}

impl ToHtml for SkillList {
    fn to_html(&self) -> Html {
        html! {
            <div class={"content-list"}>
                {self.skills.iter().map(ToHtml::to_html).collect::<Html>()}
            </div>
        }
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct SkillListProperties {
    pub skills: SkillList,
}

// ********** Achievements ********** //
// ********************************** //

#[derive(Clone, PartialEq, Eq)]
pub struct Achievement {
    pub id: usize,
    pub name: String,
    pub completed: KiraDate,
    pub areas: Vec<Rc<Area>>,
    pub tools: Vec<Rc<Tool>>,
    pub skills: Vec<Rc<Skill>>,
    pub description: String,
}

impl ToHtml for Achievement {
    fn to_html(&self) -> Html {
        html! {
            <div key={self.id} class={"card"}>
                <h3 class={"card-title"}>{self.name.clone()}</h3>
                <p class={"card-detail"}>{
                    self.areas.iter().map(|area| html!{
                        <span>{area.to_string() + ". "}</span>
                    }).collect::<Html>()
                }</p>
                <p class={"card-detail"}>{self.completed.to_string()}</p>
                <p class={"card-text"}>{self.description.clone()}</p>
                <p class={"card-detail"}>{
                    self.tools.iter().map(|tool| html!{
                        <span>{tool.to_string() + ". "}</span>
                    }).collect::<Html>()
                }</p>
                <p class={"card-detail"}>{
                    self.skills.iter().map(|skill| html!{
                        <span>{skill.name.clone() + ". "}</span>
                    }).collect::<Html>()
                }</p>
            </div>
        }
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct AchievementProperties {
    pub achievement: Achievement,
}

#[derive(Clone, PartialEq, Eq)]
pub struct AchievementList {
    achievements: Vec<Achievement>,
}

impl AchievementList {
    pub fn new(achievements: Vec<Achievement>) -> Self {
        Self { achievements }
    }
}

impl Index<usize> for AchievementList {
    type Output = Achievement;

    fn index(&self, index: usize) -> &Self::Output {
        &self.achievements[index]
    }
}

impl Get<&str, Achievement> for AchievementList {
    fn get(&self, name: &str) -> Option<Achievement> {
        self.achievements
            .iter()
            .find(|achievement| achievement.name == name)
            .cloned()
    }
}

impl ToHtml for AchievementList {
    fn to_html(&self) -> Html {
        html! {
            <div class={"content-list"}>
                {self.achievements.iter().map(ToHtml::to_html).collect::<Html>()}
            </div>
        }
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct AchievementListProperties {
    pub achievements: AchievementList,
}

// *********** Creations  *********** //
// ********************************** //

#[derive(Clone, PartialEq, Eq)]
pub struct Creation {
    pub id: usize,
    pub name: String,
    pub completed: KiraDate,
    pub areas: Vec<Rc<Area>>,
    pub tools: Vec<Rc<Tool>>,
    pub skills: Vec<Rc<Skill>>,
    pub description: String,
}

impl ToHtml for Creation {
    fn to_html(&self) -> Html {
        html! {
            <div key={self.id} class={"card"}>
                <h3 class={"card-title"}>{self.name.clone()}</h3>
                <p class={"card-detail"}>{
                    self.areas.iter().map(|area| html!{
                        <span>{area.to_string() + ". "}</span>
                    }).collect::<Html>()
                }</p>
                <p class={"card-detail"}>{self.completed.to_string()}</p>
                <p class={"card-text"}>{self.description.clone()}</p>
                <p class={"card-detail"}>{
                    self.tools.iter().map(|tool| html!{
                        <span>{tool.to_string() + "."}</span>
                    }).collect::<Html>()
                }</p>
                <p class={"card-detail"}>{
                    self.skills.iter().map(|skill| html!{
                        <span>{skill.name.clone() + ". "}</span>
                    }).collect::<Html>()
                }</p>
            </div>
        }
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct CreationProperties {
    pub creation: Creation,
}

#[derive(Clone, PartialEq, Eq)]
pub struct CreationList {
    creations: Vec<Creation>,
}

impl CreationList {
    pub fn new(creations: Vec<Creation>) -> Self {
        Self { creations }
    }
}

impl Index<usize> for CreationList {
    type Output = Creation;

    fn index(&self, index: usize) -> &Self::Output {
        &self.creations[index]
    }
}

impl Get<&str, Creation> for CreationList {
    fn get(&self, name: &str) -> Option<Creation> {
        self.creations
            .iter()
            .find(|creation| creation.name == name)
            .cloned()
    }
}

impl ToHtml for CreationList {
    fn to_html(&self) -> Html {
        html! {
            <div class={"content-list"}>
                {self.creations.iter().map(ToHtml::to_html).collect::<Html>()}
            </div>
        }
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct CreationListProperties {
    pub creations: CreationList,
}

// ************ Articles ************ //
// ********************************** //

#[derive(Clone, PartialEq)]
pub struct Article {
    pub id: usize,
    pub title: String,
    pub published: KiraDate,
    pub topics: Vec<Rc<Topic>>,
    pub summary: String,
    pub content: Html,
}

impl ToHtml for Article {
    fn to_html(&self) -> Html {
        html! {
            <article key={self.id} class={"card"}>
                <h3 class={"card-title"}>{self.title.clone()}</h3>
                <p class={"card-detail"}>{self.published.to_string()}</p>
                <p class={"card-detail"}>{
                    self.topics.iter().map(|topic| html!{
                        <span>{topic.to_string() + ". "}</span>
                    }).collect::<Html>()
                }</p>
                <p class={"card-text"}>{self.summary.clone()}</p>
                <div class={"card-content"}>{self.content.clone()}</div>
            </article>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ArticleProperties {
    pub article: Article,
}

#[derive(Clone, PartialEq)]
pub struct ArticleList {
    articles: Vec<Article>,
}

impl ArticleList {
    pub fn new(articles: Vec<Article>) -> Self {
        Self { articles }
    }
}

impl Index<usize> for ArticleList {
    type Output = Article;

    fn index(&self, index: usize) -> &Self::Output {
        &self.articles[index]
    }
}

impl Get<&str, Article> for ArticleList {
    fn get(&self, title: &str) -> Option<Article> {
        self.articles
            .iter()
            .find(|article| article.title == title)
            .cloned()
    }
}

impl ToHtml for ArticleList {
    fn to_html(&self) -> Html {
        html! {
            <div class={"content-list"}>
                {self.articles.iter().map(ToHtml::to_html).collect::<Html>()}
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ArticleListProperties {
    pub articles: ArticleList,
}
